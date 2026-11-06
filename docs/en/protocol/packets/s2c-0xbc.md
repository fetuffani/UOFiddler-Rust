# 0xBC - Seasonal Info
Direction: server -> client. Size: fixed 3 bytes total.
Purpose: Changes the season displayed by the client.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBC |
| 1 | 1 | u8 | season | 0 spring, 1 summer, 2 fall, 3 winter, 4 desolation (values > 4 become 0) |
| 2 | 1 | u8 | playSound | Plays the season's music (1) or not (0); ClassicUO treats it as a music index |


Implementation reference:
- ServUO: `Server/Network/Packets.cs` (`SeasonChange`)
- Source-X: `src/network/send.cpp` (`PacketSeason`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Season`)
