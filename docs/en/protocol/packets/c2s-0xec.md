# 0xEC - EquipMacro

Direction: client -> server. Size: variable (id + u16 length + payload). Available: before login and in game.

Purpose: Macro (KR) that equips a list of items.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEC |
| 1 | 2 | u16 | length | 4 + 4 x count |
| 3 | 1 | u8 | count | number of items |
| 4 | 4 x count | u32[] | serials | serials of the items |

Notes:

Source-X processes at most 3 items.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`EquipMacro`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingItemPackets.cs` (`EquipMacro`)
- Source-X: `src/network/receive.cpp` (`PacketEquipItemMacro`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_EquipMacroKR`)
