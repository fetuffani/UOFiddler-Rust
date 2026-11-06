# 0xFB - PublicHouseContent

Direction: client -> server. Size: 2 bytes (fixed). Available: before login and in game.

Purpose: Shows or hides the content of public houses.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xFB |
| 1 | 1 | u8 | show | 0 = hide; 1 = show |

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`PublicHouseContent`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingHousePackets.cs` (`ShowPublicHouseContent`)
- Source-X: `src/network/receive.cpp` (`PacketPublicHouseContent`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ShowPublicHouseContent`)
