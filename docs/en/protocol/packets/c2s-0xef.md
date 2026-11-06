# 0xEF - Login Seed

Direction: client -> server. Size: 21 bytes (fixed). Available: before login (first packet of the connection).

Purpose: Sends the encryption seed and the client version.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEF |
| 1 | 4 | u32 | seed | seed (zero is invalid) |
| 5 | 4 | u32 | major | client major version |
| 9 | 4 | u32 | minor | client minor version |
| 13 | 4 | u32 | revision | revision |
| 17 | 4 | u32 | prototype | patch |

Notes:

Clients prior to 6.0.5.0 send only the 4-byte seed, without id and without version. The packet table marks this id as outside the size table; servers register it with 21 bytes.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`LoginServerSeed`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`LoginServerSeed`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_Seed`, `Send_Seed_Old`)
