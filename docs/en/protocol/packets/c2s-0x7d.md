# 0x7D - MenuResponse

Direction: client -> server. Size: 13 bytes (fixed). Available: in game.

Purpose: Response to a selection menu (the server's 0x7C).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7D |
| 1 | 4 | u32 | serial | menu/object serial |
| 5 | 2 | u16 | menuId | menu id |
| 7 | 2 | u16 | index | chosen entry (1-based); 0 = cancelled |
| 9 | 2 | u16 | itemId | graphic of the entry |
| 11 | 2 | u16 | hue | color of the entry |

Notes:

On cancellation the reference client sends index = 0, itemId = 0 and hue = 0. ServUO ignores menuId.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`MenuResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`MenuResponse`)
- Source-X: `src/network/receive.cpp` (`PacketMenuChoice`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MenuResponse`, `Send_GrayMenuResponse`)
