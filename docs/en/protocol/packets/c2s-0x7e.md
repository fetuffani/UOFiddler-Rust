# 0x7E - GodviewQuery

Direction: client -> server. Size: 2 bytes (fixed). Available: in game (God client only).

Purpose: Godview mode query.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7E |
| 1 | 1 | u8 | value | Not specified |

Notes:

ServUO only logs the value.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`GodviewQuery`)
