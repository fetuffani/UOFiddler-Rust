# 0xDF - Buff/Debuff
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Adiciona ou remove um ícone de buff/debuff.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDF |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do mobile |
| 7 | 2 | u16 | iconId | Id do ícone (BuffIcon) |
| 9 | 2 | u16 | count | 0 = remover o ícone; 1 = adicionar (uma entrada a seguir) |
| 11 | variável | entry | entry | Presente somente se count > 0 |

Notas: Remoção: `count = 0` seguido de 4 bytes zerados (ServUO: 13 bytes; ModernUO: 15 bytes com length).

Entrada de adição (count = 1):

| Tamanho | Tipo | Campo |
|---|---|---|
| 2 | u16 | sourceType (0) |
| 2 | u16 | reservado (0) |
| 2 | u16 | iconId (repetido) |
| 2 | u16 | queueIndex (ServUO envia 1) |
| 4 | u32 | reservado (0) |
| 2 | u16 | duração em segundos (0 = sem timer) |
| 3 | u8[3] | reservado (0) |
| 4 | u32 | titleCliloc |
| 4 | u32 | secondaryCliloc (descrição) |
| 4 | u32 | terceiro cliloc (0) |
| 2+ | u16 + utf16le | argumentos 1: comprimento, texto UTF-16LE terminado por NUL |
| 2+ | u16 + utf16le | argumentos 2 |
| 2+ | u16 + utf16le | argumentos 3 |

Sem argumentos o servidor escreve após o terceiro cliloc 6 bytes zerados (3 comprimentos 0). Com argumentos o ServUO e o ModernUO escrevem: comprimento `0x0001`, `0x0000`, `'\t'` seguido do texto UTF-16LE com NUL, depois `0x0001` e `0x0000`. O significado exato dos campos de comprimento não é especificado. O ClassicUO converte o id: ids >= 0x466 e < 0x3E9 seguem tabelas distintas.

## Referência de implementação
- ServUO: `Scripts/Misc/BuffIcons.cs` (`AddBuffPacket`, `RemoveBuffPacket`)
- ModernUO: `Projects/UOContent/Engines/BuffIcons/BuffIconPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketBuff`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BuffDebuff`)
