# 0xB0 - Gump Menu Dialog (gump)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Abre um gump genérico descrito por um layout de texto e uma tabela de textos (formato sem compressão).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB0 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do dono do gump |
| 7 | 4 | u32 | gumpId | Tipo/id do gump, devolvido no 0xB1 |
| 11 | 4 | i32 | x | Posição X na tela |
| 15 | 4 | i32 | y | Posição Y na tela |
| 19 | 2 | u16 | layoutLength | Comprimento em bytes do layout (ServUO `DisplayGump` inclui o NUL final) |
| 21 | layoutLength | ascii | layout | Comandos do gump no formato `{ comando args }`; ver gump-layout.md |
| 21+layoutLength | 2 | u16 | textCount | Quantidade de linhas de texto |
| ... | variável | line[] | lines | `textCount` linhas |

Notas: Linha: `u16 charCount` seguido de `charCount` caracteres UTF-16 big-endian (sem NUL). A versão comprimida com zlib é o 0xDD. O ClassicUO lê o layout como ASCII e cada linha com `ReadUnicodeBE`.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`DisplayGump`, `DisplayGumpFast`)
- ModernUO: `Projects/UOContent/Gumps/Base/`
- Source-X: `src/network/send.cpp` (`PacketGumpDialog`, `writeStandardControls`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenGump`)
