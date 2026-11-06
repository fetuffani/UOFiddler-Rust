# 0xBA - Quest Arrow
Direção: servidor -> cliente. Tamanho: 10 bytes (HS) | 6 bytes (anterior).
Finalidade: Mostra ou remove a seta de missão na tela.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBA |
| 1 | 1 | u8 | active | 1 = exibir, 0 = remover |
| 2 | 2 | u16 | x | Coordenada X alvo |
| 4 | 2 | u16 | y | Coordenada Y alvo |
| 6 | 4 | u32 | serial | Serial associado (apenas clientes 7.0.9.0+; 10 bytes) |

Notas: A variante de 6 bytes termina em `y`. Ao remover a seta o ServUO antigo envia x = y = -1; a variante HS repete as coordenadas.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`SetArrow`, `CancelArrow`, `SetArrowHS`, `CancelArrowHS`)
- ModernUO: `Projects/UOContent/Skills/Tracking/OutgoingArrowPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketArrowQuest`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayQuestArrow`)
