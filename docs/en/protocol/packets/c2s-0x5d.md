# 0x5D - Play Character

Direction: client -> server. Size: 73 bytes (fixed). Available: before login (character selection).

Purpose: Selects the character to enter the world with.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x5D |
| 1 | 4 | u32 | pattern | 0xEDEDEDED |
| 5 | 30 | char[30] | name | character name (ASCII) |
| 35 | 2 | u8[2] | reserved | zeros |
| 37 | 4 | u32 | flags | client protocol flags |
| 41 | 24 | u8[24] | reserved | zeros in official clients; see notes |
| 65 | 4 | u32 | slot | character slot in the account |
| 69 | 4 | u32 | clientIp | client IPv4 |

Notes:

The 24 bytes at offset 41 include, in Source-X, 4 bytes of additional flags, 4 bytes of login count, and 16 bytes of unspecified use (0xFF000000 in Enhanced Client). Third-party tools may use them to carry their own data; ServUO optionally reads 8 bytes of feature flags and 16 bytes of an authentication key from this region.
Response: 0x1B Login Confirm and the world entry sequence. If the slot is invalid, the server closes the connection.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`PlayCharacter`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`PlayCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCharPlay`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SelectCharacter`)
