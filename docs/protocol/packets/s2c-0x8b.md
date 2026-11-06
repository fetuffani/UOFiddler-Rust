# 0x8B - Display Sign Gump
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Exibe a janela de placa (sign) com um texto.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8B |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial da placa |
| 7 | 2 | u16 | gumpId | Id do gump da placa |
| 9 | 2 | u16 | text1Length | Comprimento do primeiro texto (inclui o NUL final no ModernUO) |
| 11 | text1Length | ascii | text1 | Primeiro texto (parâmetro "unknown" nas implementações) |
| ... | 2 | u16 | captionLength | Comprimento da legenda, incluindo o NUL final |
| ... | captionLength | ascii | caption | Texto da placa, termina em NUL |

Notas: ServUO escreve o primeiro texto sem NUL (comprimento = número de caracteres); ModernUO escreve com NUL e soma 1 ao comprimento. O ClassicUO não possui handler para 0x8B.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`DisplaySignGump`)
- ModernUO: `Projects/UOContent/Gumps/Base/OutgoingGumpPackets.cs` (`SendDisplaySignGump`)
- Source-X: `src/network/send.cpp` (`PacketSignGump`)
