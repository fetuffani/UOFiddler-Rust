# 0xF1 - Time Sync Request

Direction: client -> server. Size: 9 bytes (fixed). Available: in game.

Purpose: Requests time synchronization.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF1 |
| 1 | 8 | u64 | clientTime | client time |

Notes:

Response: 0xF2 Time Sync Response (25 bytes: id + 3 x 8 bytes). ModernUO keeps the handling disabled.

Implementation reference:

- Source-X: `src/network/receive.cpp` (`PacketTimeSyncRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMovementPackets.cs` (`TimeSyncReq`, disabled)
