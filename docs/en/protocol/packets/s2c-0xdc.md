# 0xDC - SE Introduced Revision
Direction: server -> client. Size: fixed 9 bytes total.
Purpose: Reports the revision (hash) of an object's property list, so the client only requests 0xD6 if the revision has changed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDC |
| 1 | 4 | u32 | serial | Object |
| 5 | 4 | u32 | revision | `0x40000000 + hash` (ServUO and ModernUO) |

Notes: there is no length field; the size is fixed at 9. The client compares the revision with the one it has cached and, if it differs, requests 0xD6.

Implementation reference:
- ServUO `Server/ObjectPropertyList.cs` (`OPLInfo`)
- ModernUO `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateOPLInfo`)
- Source-X `send.cpp` (`PacketPropertyListVersion`)
- ClassicUO `PacketHandlers.cs` (`OPLInfo`)

## Example

Revision of object 0x00001234 with hash 0x0000002A (sent as `0x40000000 + hash`):

```
DC 00 00 12 34 40 00 00 2A
```

`DC` id; `00 00 12 34` serial; `40 00 00 2A` revision. There is no length field (9 bytes fixed).
