# 0x9B - Help Request

Direction: client -> server. Size: 258 bytes (fixed). Available: in game.

Purpose: Requests help from a GM (Help button).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9B |
| 1 | 257 | u8[257] | padding | zeros; the server does not read the content |

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`HelpRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`HelpRequest`)
- Source-X: `src/network/receive.cpp` (`PacketHelpPageReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_HelpRequest`)
