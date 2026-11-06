# 0x4B - Check Client Version

Direction: client -> server. Size: 9 bytes (fixed). Available: Not specified.

Purpose: Client version check.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4B |
| 1 | 8 | u8[8] | data | Not specified |

Notes:

No reference server implementation handles this id; the client 7.0.10.3 contains the packet constructor unused.

Implementation reference:

- ClassicUO size table: `src/ClassicUO.Client/Network/PacketsTable.cs` (9 bytes)
