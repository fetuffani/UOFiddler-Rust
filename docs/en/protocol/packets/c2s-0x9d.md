# 0x9D - GMSingle

Direction: client -> server. Size: 51 bytes (fixed). Available: in game (God client only).

Purpose: GM command (God client).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9D |
| 1 | 50 | u8[50] | data | Not specified |

Notes:

ServUO only dumps the content.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`GMSingle`)
