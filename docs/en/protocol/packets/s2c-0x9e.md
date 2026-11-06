# 0x9E - Sell List
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Opens the vendor's sell window with the items the player can sell.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9E |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | vendor | Vendor serial |
| 7 | 2 | u16 | count | Number of entries |
| 9 | variable | entry[] | items | `count` entries |

Notes: Entry:

| Size | Type | Field |
|---|---|---|
| 4 | u32 | item serial |
| 2 | u16 | itemId |
| 2 | u16 | hue |
| 2 | u16 | amount |
| 2 | u16 | price |
| 2 | u16 | nameLength |
| nameLength | ascii | name (without NUL) |

If `name` is numeric, ClassicUO treats it as a cliloc number; if empty, it uses the name from tiledata.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`VendorSellList`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingVendorSellPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketVendorSellList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`SellList`)
