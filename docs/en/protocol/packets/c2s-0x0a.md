# 0x0A - Edit

Direction: client -> server. Size: 11 bytes (fixed). Available: in game (God client only).

Purpose: Map/static editing by the God client.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x0A |
| 1 | 1 | u8 | type | 10 = static, 7 = NPC, 4 = dynamic |
| 2 | 2 | i16 | x | X position |
| 4 | 2 | i16 | y | Y position |
| 6 | 2 | i16 | graphic | graphic id |
| 8 | 1 | i8 | z | Z position |
| 9 | 2 | u16 | hue | color |

Notes:

ServUO only logs the content and disconnects accounts without staff privileges.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`Edit`)
