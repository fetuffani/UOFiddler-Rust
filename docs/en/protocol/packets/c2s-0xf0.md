# 0xF0 - Movement Request (batch)

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Sends several movement steps at once (KR/SA clients with the new movement).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF0 |
| 1 | 2 | u16 | length | 4 + 34 x steps |
| 3 | 1 | u8 | steps | number of steps |

**Step (34 bytes, repeated steps times, starting at offset 4)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 8 | u64 | time1 | timestamp (start) |
| +8 | 8 | u64 | time2 | timestamp (end) |
| +16 | 1 | u8 | sequence | step sequence |
| +17 | 1 | u8 | direction | direction (0-7) |
| +18 | 4 | u32 | mode | 1 = walk; 2 = run |
| +22 | 4 | i32 | x | X position |
| +26 | 4 | i32 | y | Y position |
| +30 | 4 | i32 | z | Z position |

Notes:

The server applies each step in order, with the same sequence rules as 0x02; mode = 2 is equivalent to the 0x80 bit of the direction. Source-X only handles the packet when SA movement is enabled and length matches the received size. ModernUO keeps the handling disabled, since the id is also used by assistant extensions.

Implementation reference:

- Source-X: `src/network/receive.cpp` (`PacketMovementReqNew`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMovementPackets.cs` (`NewMovementReq`, disabled)
