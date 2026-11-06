# 0xB6 - Object Help Request

Direction: client -> server. Size: 9 bytes (fixed). Available: in game.

Purpose: Requests contextual help about an object.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB6 |
| 1 | 4 | u32 | serial | serial of the object |
| 5 | 1 | u8 | unknown | Not specified |
| 6 | 3 | char[3] | language | ASCII language code |

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ObjectHelpRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingEntityPackets.cs` (`ObjectHelpRequest`)
