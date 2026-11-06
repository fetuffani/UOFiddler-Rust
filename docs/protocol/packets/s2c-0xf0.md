# 0xF0 - Movimento em lote e pacote especial Krrios
Direção: cliente -> servidor (movimento) e ambas (especial Krrios). Tamanho: variável.
Finalidade: id compartilhado por duas funções.

Movimento em lote (C->S, clientes KR/SA com a feature habilitada):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF0 |
| 1 | 2 | u16 | length | `4 + 34 * steps` |
| 3 | 1 | u8 | steps | Número de passos |
| 4 | 34 por passo | step[] | steps | Ver abaixo |

Passo (34 bytes):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 8 | i64 | time1 | Carimbo de tempo (anti fast-walk; ignorado pelo Source-X) |
| 8 | 8 | i64 | time2 | Carimbo de tempo |
| 16 | 1 | u8 | sequence | Sequência do passo |
| 17 | 1 | u8 | direction | Direção |
| 18 | 4 | u32 | mode | 1 = andar, 2 = correr |
| 22 | 4 | u32 | x | Posição (não usada pelo servidor) |
| 26 | 4 | u32 | y | |
| 30 | 4 | u32 | z | |

Notas: o Source-X só descreve o formato; a validação de `length` contra o tamanho recebido distingue o movimento do pacote especial Krrios. ServUO, ModernUO e ClassicUO não processam este formato. O Source-X comenta que o campo exige habilitar a feature no login (0xA9).

Pacote especial Krrios/Razor (ClassicUO):

| Direção | Corpo após `length` | Descrição |
|---|---|---|
| S->C `type 0x00` | nenhum | Aceite do handshake; o cliente habilita o mapa de grupo/guild |
| S->C `type 0x01` | repetir `u32 serial, u16 x, u16 y, u8 map` até serial 0 | Posições dos membros do grupo |
| S->C `type 0x02` | `u8 locations` e, se verdadeiro, repetir `u32 serial, u16 x, u16 y, u8 map, u8 hits` até serial 0 | Posições da guild |
| S->C `type 0x03`, `0x04`, `0xF0` | ignorado | Runebook, guardline |
| S->C `type 0xFE` | nenhum | Pedido de ACK do Razor |
| C->S `0xFF` | nenhum | ACK do Razor (3 bytes) |
| C->S `0x00` | nenhum | Pede posições do grupo |
| C->S `0x01` + `u8 1` | | Pede posições da guild |


Referência de implementação:
- Source-X: `src/network/receive.cpp` (`PacketMovementReqNew`)
- ClassicUO: `PacketHandlers.cs` (`KrriosClientSpecial`), `OutgoingPackets.cs` (`Send_RazorACK`, `Send_QueryGuildPosition`, `Send_QueryPartyPosition`)
