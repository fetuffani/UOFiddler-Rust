# 0xEC - EquipMacro

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: antes do login e em jogo.

Finalidade: Macro (KR) que equipa uma lista de itens.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEC |
| 1 | 2 | u16 | length | 4 + 4 x count |
| 3 | 1 | u8 | count | número de itens |
| 4 | 4 x count | u32[] | serials | serials dos itens |

Notas:

O Source-X processa no máximo 3 itens.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`EquipMacro`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingItemPackets.cs` (`EquipMacro`)
- Source-X: `src/network/receive.cpp` (`PacketEquipItemMacro`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_EquipMacroKR`)
