# 0xD6 - Mega Cliloc (object property list)
Direction: both. Size: variable.
Purpose: S->C delivers the property list (tooltip) of an object; C->S requests the properties of several objects at once.

S->C:

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | Total size |
| 3 | 2 | u16 | unknown | 0x0001 (the client ignores the packet if it is greater than 1) |
| 5 | 4 | u32 | serial | Object |
| 9 | 2 | u16 | reserved | 0x0000 |
| 11 | 4 | u32 | revision | Hash/revision of the list (the same one sent in 0xDC, without bit 30) |
| 15 | ... | entry[] | entries | Entries |
| end | 4 | u32 | terminator | 0x00000000 |

Entry:

| Size | Type | Field | Description |
|---|---|---|---|
| 4 | u32 | cliloc | Cliloc number (0 ends the list) |
| 2 | u16 | byteLength | Length in bytes of the arguments |
| byteLength | UTF-16LE | arguments | Arguments, without a null terminator; multiple arguments are separated by tab |

Notes:
- The first cliloc in the list is the object's name (header).
- ModernUO limits each argument to 504 characters (the old 2D client corrupts memory above ~510).
- The hash is computed by the server over the body: in ServUO it is an XOR of `cliloc & 0x3FFFFFF` and `(cliloc >> 26) & 0x3F` plus the hash of the argument text; in ModernUO it is the xxHash3 of the entry block masked with `0x3FFFFFF`. The client only compares the value for equality.
- Lines with a cliloc that has no translation are discarded by the client.

C->S (batch query):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | `3 + 4*n` |
| 3 | 4*n | u32[n] | serials | Objects queried (ClassicUO sends at most 15 per packet) |

Notes: the server requires `(length - 3) % 4 == 0`. The same request for a single object can be made with 0xBF/0x10 (old format).

Implementation reference:
- ServUO `Server/ObjectPropertyList.cs` (`ObjectPropertyList`), `Server/Network/PacketHandlers.cs` (`BatchQueryProperties`)
- ModernUO `Projects/Server/PropertyList/ObjectPropertyList.cs`
- Source-X `src/network/send.cpp` (`PacketPropertyList`)
- ClassicUO `PacketHandlers.cs` (`MegaCliloc`), `OutgoingPackets.cs` (`Send_MegaClilocRequest`)

## Example

Properties of item 0x40001234 with revision 0x0012ABCD and two entries: cliloc 1042971 with argument `Sword` and cliloc 1075085 without an argument (41 bytes):

```
D6 00 29 00 01 40 00 12 34 00 00 00 12 AB CD
00 0F EA 1B 00 0A 53 00 77 00 6F 00 72 00 64 00
00 10 67 8D 00 00
00 00 00 00
```

| Excerpt | Meaning |
|---|---|
| `D6 00 29` | id and length = 41 |
| `00 01` | unknown = 1 |
| `40 00 12 34` | serial |
| `00 00` | reserved |
| `00 12 AB CD` | revision |
| `00 0F EA 1B` | cliloc 1042971 |
| `00 0A` | 10 bytes of arguments |
| `53 00 77 00 6F 00 72 00 64 00` | "Sword" in UTF-16LE |
| `00 10 67 8D` `00 00` | cliloc 1075085 without arguments |
| `00 00 00 00` | terminator |

---
