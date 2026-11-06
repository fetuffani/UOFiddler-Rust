# 0xA7 - Request Tip

Direction: client -> server. Size: 4 bytes (fixed). Available: in game.

Purpose: Requests the previous or next tip of the day.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA7 |
| 1 | 2 | u16 | lastTip | index of the displayed tip |
| 3 | 1 | u8 | direction | 0 = previous; 1 = next |

Notes:

ServUO reads the fields and does not respond; Source-X responds with the corresponding tip.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`RequestScrollWindow`)
- Source-X: `src/network/receive.cpp` (`PacketTipReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TipRequest`)
