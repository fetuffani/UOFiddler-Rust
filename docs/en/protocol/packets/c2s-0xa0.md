# 0xA0 - Select Server

Direction: client -> server. Size: 3 bytes (fixed). Available: before login.

Purpose: Selects the game server from the list received in 0xA8.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA0 |
| 1 | 2 | u16 | index | index of the server in the 0xA8 list |

Notes:

Response: 0x8C Connect To Game Server (address, port and authentication key). The client then opens a new connection and sends 0x91.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`PlayServer`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`PlayServer`)
- Source-X: `src/network/receive.cpp` (`PacketServerSelect`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SelectServer`)
