# 0xC9 - Trip Time
Direction: server -> client. Size: 6 bytes.
Purpose: Latency measurement (ping) response.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC9 |
| 1 | 1 | u8 | unknown | Value repeated from the request |
| 2 | 4 | i32 | tick | Server tick count |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`TripTimeResponse`)
