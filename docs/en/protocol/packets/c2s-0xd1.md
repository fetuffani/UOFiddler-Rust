# 0xD1 - LogoutReq

Direction: client -> server. Size: 2 bytes (fixed). Available: in game.

Purpose: Notifies that the player is leaving the game.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD1 |
| 1 | 1 | u8 | value | 0x00 |

Notes:

Response: 0xD1 Logout Status (2 bytes).

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`LogoutReq`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`LogoutReq`)
- Source-X: `src/network/receive.cpp` (`PacketLogout`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_LogoutNotification`)
