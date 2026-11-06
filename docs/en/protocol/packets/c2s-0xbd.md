# 0xBD - Client Version

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Reports the client version.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBD |
| 1 | 2 | u16 | length | total size |
| 3 | n | char[] | version | ASCII terminated by NUL, for example "7.0.10.3" |

Notes:

This is normally the response to a 0xBD sent by the server. Source-X reads at most 20 characters.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ClientVersion`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`ClientVersion`)
- Source-X: `src/network/receive.cpp` (`PacketClientVersion`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ClientVersion`)
