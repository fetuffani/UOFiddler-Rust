# 0xC9 - TripTime

Direction: client -> server. Size: 6 bytes (fixed). Available: in game.

Purpose: Trip time measurement (network ping).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC9 |
| 1 | 1 | u8 | value | value to be returned |
| 2 | 4 | u32 | unknown | Not specified |

Notes:

Response: packet 0xC9 (6 bytes) with the value from offset 1.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`TripTime`)
