# 0x2C - DeathStatusResponse

Direction: client -> server. Size: 2 bytes (fixed). Available: in game.

Purpose: The player's response to the death screen.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2C |
| 1 | 1 | u8 | action | see notes |

Notes:

Values in Source-X: 0x01 resurrect, 0x02 continue as a ghost; the reference client sends 0x02. ServUO ignores the packet.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`DeathStatusResponse`)
- Source-X: `src/network/receive.cpp` (`PacketDeathStatus`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_DeathScreen`)
