# 0xE1 - ClientType

Direction: client -> server. Size: variable (id + u16 length + payload). Available: before login and in game.

Purpose: Reports the client type (2D, KR, Enhanced).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE1 |
| 1 | 2 | u16 | length | 9 |
| 3 | 2 | u16 | count | 1 |
| 5 | 4 | u32 | clientType | 2 = KR; 3 = Enhanced Client |

Notes:

Layout used by ServUO and Source-X. ModernUO reads, after the initial u16, a u16 type and an ASCII version string. Clients prior to 5.0.9 use a fixed size of 9 bytes.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ClientType`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`ClientType`)
- Source-X: `src/network/receive.cpp` (`PacketClientType`)
