# 0xD0 - ConfigurationFile

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Sends a configuration file.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD0 |
| 1 | 2 | u16 | length | total size |
| 3 | n | u8[] | data | Not specified |

Notes:

Reference servers ignore the content.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ConfigurationFile`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`ConfigurationFile`)
