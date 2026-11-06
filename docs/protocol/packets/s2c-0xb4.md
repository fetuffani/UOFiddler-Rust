# 0xB4 - Target Object List
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Mostra itens fantasmas que acompanham o cursor de alvo.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB4 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1 | u8 | allowGround | 1 se o chão pode ser alvo |
| 4 | 4 | u32 | serial | Serial/id do cursor |
| 8 | 2 | u16 | x | X |
| 10 | 2 | u16 | y | Y |
| 12 | 2 | u16 | z | Z |
| 14 | 2 | u16 | count | Quantidade de entradas |
| 16 | 10 x count | entry[] | items | Entrada: 5 x u16 (itemId, hue, xOffset, yOffset, zOffset) |

Notas: Layout conforme documentado na tabela de pacotes do projeto (packets.md); o clique é respondido com 0x6C. Não especificado nas implementações de referência.

## Referência de implementação
- Tabela do projeto: `docs/protocol/packets.md` (seção 0xB4)
