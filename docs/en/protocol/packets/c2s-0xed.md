# 0xED - UnequipMacro

Direction: client -> server. Size: variable (id + u16 length + payload). Available: before login and in game.

Purpose: Macro (KR) that unequips layers.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xED |
| 1 | 2 | u16 | length | 4 + 2 x count |
| 3 | 1 | u8 | count | number of layers |
| 4 | 2 x count | u16[] | layers | layers to unequip |

Notes:

Source-X processes at most 3 layers.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnequipMacro`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingItemPackets.cs` (`UnequipMacro`)
- Source-X: `src/network/receive.cpp` (`PacketUnEquipItemMacro`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnequipMacroKR`)
