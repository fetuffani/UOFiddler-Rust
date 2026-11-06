# 0xC3 - GQ Request
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Pedido de ajuda para o sistema de fila global (GQ).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC3 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | variável | u8[] | payload | Sequência de inteiros e uma string `region`, conforme o construtor do ServUO |

Notas: Sequência escrita pelo ServUO após o cabeçalho: 4 x i32, i32, i16, i16 (comprimento 6), 6 bytes ASCII "region", i32 (tempo de chamada em segundos), i16 (mapa), i32 x, i32 y, i32 z, i32 volume, i32 rank, i32 (-1), i32 tipo. Os nomes dos campos vêm dos comentários do construtor, com valores de teste. O ClassicUO não registra handler.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`GQRequest`)
