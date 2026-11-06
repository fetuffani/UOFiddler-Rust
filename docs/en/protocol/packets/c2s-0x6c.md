# 0x6C - Target Response

Direction: client -> server. Size: 19 bytes (fixed). Available: in game.

Purpose: Responds to a target cursor (the server's 0x6C): object, position, or cancellation.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6C |
| 1 | 1 | u8 | targetType | 0 = object; 1 = position/static |
| 2 | 4 | u32 | cursorId | id of the cursor received in the target request |
| 6 | 1 | u8 | cursorType | 0 neutral, 1 harmful, 2 beneficial, 3 cancel |
| 7 | 4 | u32 | serial | object serial (0 if position) |
| 11 | 2 | u16 | x | X |
| 13 | 2 | u16 | y | Y |
| 15 | 2 | i16 | z | Z |
| 17 | 2 | u16 | graphic | graphic id of the static/object (0 = terrain) |

Notes:

Cancellation: x = 0xFFFF, y = 0xFFFF and serial = 0 (the reference client also sends z = 0 and graphic = 0). ServUO validates cursorId against the pending target.
For a position target, graphic = 0 indicates terrain; non-zero indicates a static at that coordinate.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`TargetResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingTargetingPackets.cs` (`TargetResponse`)
- Source-X: `src/network/receive.cpp` (`PacketTarget`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TargetObject`, `Send_TargetXYZ`, `Send_TargetCancel`)
