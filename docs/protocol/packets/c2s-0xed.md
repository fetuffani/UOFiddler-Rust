# 0xED - UnequipMacro

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: antes do login e em jogo.

Finalidade: Macro (KR) que desequipa camadas.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xED |
| 1 | 2 | u16 | length | 4 + 2 x count |
| 3 | 1 | u8 | count | número de camadas |
| 4 | 2 x count | u16[] | layers | camadas a desequipar |

Notas:

O Source-X processa no máximo 3 camadas.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnequipMacro`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingItemPackets.cs` (`UnequipMacro`)
- Source-X: `src/network/receive.cpp` (`PacketUnEquipItemMacro`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnequipMacroKR`)
