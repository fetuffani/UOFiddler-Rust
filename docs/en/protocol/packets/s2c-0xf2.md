# 0xF2 - Time Sync Response
Direction: server -> client. Size: 25 bytes.
Purpose: Time synchronization response.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF2 |
| 1 | 8 | i64 | time1 | Server time |
| 9 | 8 | i64 | time2 | Server time |
| 17 | 8 | i64 | time3 | Server time |

Notes: ModernUO writes `Core.TickCount` three times; Source-X writes the Unix time in milliseconds three times. The individual meaning of each field is not specified.

## Implementation reference
- ModernUO: `Projects/Server/Network/Packets/OutgoingMovementPackets.cs` (`SendTimeSyncResponse`)
- Source-X: `src/network/send.cpp` (`PacketTimeSyncResponse`)
