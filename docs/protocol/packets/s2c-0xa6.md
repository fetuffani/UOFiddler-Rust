# 0xA6 - Tip/Notice
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Exibe uma janela de dica (tip) ou aviso (notice) com texto.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA6 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1 | u8 | type | 0 = tip, 1 = ignorado pelo ClassicUO, 2 = notice |
| 4 | 4 | u32 | tipNumber | Número da dica / contexto |
| 8 | 2 | u16 | textLength | Comprimento do texto em bytes |
| 10 | textLength | ascii | text | Texto; quebras de linha com 0x0D |

Notas: O Source-X separa as linhas do texto com 0x0D e acrescenta um NUL final fora de `textLength`. O ClassicUO converte '\r' em '\n'. Para `type = 1` o ClassicUO descarta o pacote; para 0 e 2 exibe a janela em posições diferentes.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ScrollMessage`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketOpenScroll`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`TipWindow`)
