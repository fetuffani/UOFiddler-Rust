# 0xC8 - Update Range
Direction: server -> client. Size: 2 bytes.
Purpose: Sets the client's view range.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC8 |
| 1 | 1 | u8 | range | Range in tiles |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ChangeUpdateRange`)
- Source-X: `src/network/send.cpp` (`PacketVisualRange`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ClientViewRange`)
