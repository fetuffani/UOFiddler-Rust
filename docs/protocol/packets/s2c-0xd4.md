# 0xD4 - Book Header (novo)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Abre um livro com título e autor de comprimento variável.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD4 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do livro |
| 7 | 1 | u8 | flag | 1 |
| 8 | 1 | u8 | writable | 1 se editável |
| 9 | 2 | u16 | pages | Quantidade de páginas |
| 11 | 2 | u16 | titleLength | Comprimento do título em bytes, incluindo o NUL |
| 13 | titleLength | utf8 | title | Título em UTF-8, terminado em NUL |
| ... | 2 | u16 | authorLength | Comprimento do autor, incluindo o NUL |
| ... | authorLength | utf8 | author | Autor em UTF-8, terminado em NUL |

Notas: Tamanho total no ModernUO: `17 + bytes(title) + bytes(author)`. Em dados mais antigos o 0x93 usa campos fixos.

## Referência de implementação
- ServUO: `Scripts/Items/Books/BaseBook.cs`
- ModernUO: `Projects/UOContent/Items/Books/BookPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketDisplayBookNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenBook`)
