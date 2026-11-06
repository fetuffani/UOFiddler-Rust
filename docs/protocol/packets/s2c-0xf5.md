# 0xF5 - New Map Message
Direção: servidor -> cliente. Tamanho: 21 bytes.
Finalidade: Abre um gump de mapa com id da facet (substitui o 0x90).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF5 |
| 1 | 4 | u32 | serial | Serial do item mapa |
| 5 | 2 | u16 | gumpId | 0x139D |
| 7 | 2 | u16 | startX | Canto superior esquerdo X |
| 9 | 2 | u16 | startY | Canto superior esquerdo Y |
| 11 | 2 | u16 | endX | Canto inferior direito X |
| 13 | 2 | u16 | endY | Canto inferior direito Y |
| 15 | 2 | u16 | width | Largura |
| 17 | 2 | u16 | height | Altura |
| 19 | 2 | u16 | facet | Id da facet (0 Felucca, 1 Trammel, 2 Ilshenar, 3 Malas, 4 Tokuno, 5 Ter Mur) |

Notas: Divergência: o ServUO e o ClassicUO tratam o par como largura seguida de altura; o Source-X escreve altura primeiro e depois largura.

## Referência de implementação
- ServUO: `Scripts/Items/Tools/MapItem.cs` (`NewMapDetails`)
- Source-X: `src/network/send.cpp` (`PacketDisplayMapNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayMap`)
