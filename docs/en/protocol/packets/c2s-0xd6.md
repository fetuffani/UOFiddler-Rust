# 0xD6 - BatchQueryProperties

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Requests the properties (tooltips) of several objects.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | 3 + 4 x n |
| 3 | 4 | u32 | serial | serial of the object (repeated n times) |

Notes:

The reference client sends at most 15 serials per packet. Response: 0xD6 Mega Cliloc for each visible object.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`BatchQueryProperties`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingEntityPackets.cs` (`BatchQueryProperties`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MegaClilocRequest`)
