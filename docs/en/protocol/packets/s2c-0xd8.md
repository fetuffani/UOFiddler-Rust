# 0xD8 - Custom House
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Delivers to the client the complete structure of a custom house, compressed per plane.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD8 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1 | u8 | compression | 0x03 (zlib per plane) |
| 4 | 1 | u8 | response | 0x00 or 0x01; the client ignores it |
| 5 | 4 | u32 | serial | Serial of the house |
| 9 | 4 | u32 | revision | Revision number |
| 13 | 2 | u16 | itemCount | Total number of components |
| 15 | 2 | u16 | bufferLength | 1 + sum(4 + compressed size of each block) |
| 17 | 1 | u8 | blockCount | Number of blocks |
| 18 | variable | block[] | blocks | Blocks |

Notes: Block: `u8 index` (high nibble = mode: 2 grid, 0 list of 5-byte entries, 1 list of 4-byte entries; low nibble = plane), `u8 sizeLow`, `u8 compressedLow`, `u8 ((size >> 4) & 0xF0) | ((compressed >> 8) & 0x0F)`, then `compressed` zlib bytes. The sizes are 12 bits. Planes, coordinates, offset blocks and the customization pipeline are described in house-customization.md (sections 3.4 and 4).

## Implementation reference
- ServUO: `Scripts/Multis/HouseFoundation.cs` (`DesignStateDetailed`)
- ModernUO: `Projects/UOContent/Multis/Houses/HousePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketHouseDesign`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CustomHouse`)
- Project document: `docs/protocol/house-customization.md`
