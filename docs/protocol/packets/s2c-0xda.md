# 0xDA - Mahjong (game packets)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Pacotes do jogo de Mahjong; um byte de subcomando seleciona o formato.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDA |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do jogo |
| 7 | 1 | u8 | reserved | 0 |
| 8 | 1 | u8 | subcommand | Subcomando (tabela) |
| 9 | variável | u8[] | payload | Depende do subcomando |

Notas: Subcomandos enviados pelo servidor:

| Sub | Nome | Payload (a partir do offset 9) |
|---|---|---|
| 0x02 | Players info | `u8 0`, `u8 seats`, depois `seats` entradas de 45 bytes (abaixo) |
| 0x03 | Tile info | 1 tile de 9 bytes (abaixo) |
| 0x04 | Tiles info | `u16 count`, depois `count` tiles de 9 bytes |
| 0x05 | General info | `u16 0`, `u8 0`, `u8 flags` (bit 0x01 mostrar placar, 0x02 visão de espectador), `u8 dado1`, `u8 dado2`, `u8 dealerWind`, `u16 dealerY`, `u16 dealerX`, `u8 dealerDirection`, `u16 wallBreakY`, `u16 wallBreakX` |
| 0x19 | Join game | sem payload (tamanho total 9) |
| 0x1A | Relieve | sem payload (tamanho total 9) |

Entrada de jogador (45 bytes): `u32 serial` (0 para assento vazio), `u8 papel` (1 dealer, 2 demais), `u8 assento`, `i32 pontuação`, `u16 0`, `u8 0`, `u8 público`, `ascii[30] nome`, `u8 espectador` (1 se não está jogando).

Tile (9 bytes): `u8 número`, `u8 valor` (0 se oculto), `u16 y`, `u16 x`, `u8 nível de pilha`, `u8 direção`, `u8 flags` (0x10 = virada).

O ServUO aplica a ordem Y antes de X nas posições.

## Referência de implementação
- ServUO: `Scripts/Items/Containers/Mahjong/Packets.cs`
- ModernUO: `Projects/UOContent/Items/Games/Mahjong/MahjongPackets.cs`
