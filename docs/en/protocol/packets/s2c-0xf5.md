# 0xF5 - New Map Message
Direction: server -> client. Size: 21 bytes.
Purpose: Opens a map gump with the facet id (replaces 0x90).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF5 |
| 1 | 4 | u32 | serial | Serial of the map item |
| 5 | 2 | u16 | gumpId | 0x139D |
| 7 | 2 | u16 | startX | Top-left corner X |
| 9 | 2 | u16 | startY | Top-left corner Y |
| 11 | 2 | u16 | endX | Bottom-right corner X |
| 13 | 2 | u16 | endY | Bottom-right corner Y |
| 15 | 2 | u16 | width | Width |
| 17 | 2 | u16 | height | Height |
| 19 | 2 | u16 | facet | Facet id (0 Felucca, 1 Trammel, 2 Ilshenar, 3 Malas, 4 Tokuno, 5 Ter Mur) |

Notes: Divergence: ServUO and ClassicUO treat the pair as width followed by height; Source-X writes height first and then width.

## Implementation reference
- ServUO: `Scripts/Items/Tools/MapItem.cs` (`NewMapDetails`)
- Source-X: `src/network/send.cpp` (`PacketDisplayMapNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayMap`)
