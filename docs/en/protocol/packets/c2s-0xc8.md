# 0xC8 - SetUpdateRange

Direction: client -> server. Size: 2 bytes (fixed). Available: in game.

Purpose: Reports the desired view range.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC8 |
| 1 | 1 | u8 | range | range in tiles |

Notes:

The server clamps the value to its minimum and maximum and responds with 0xC8 (2 bytes) containing the effective range.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`SetUpdateRange`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`SetUpdateRange`)
- Source-X: `src/network/receive.cpp` (`PacketViewRange`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ClientViewRange`)
