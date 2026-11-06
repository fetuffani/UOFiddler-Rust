# 0xC9 - Trip Time
Direção: servidor -> cliente. Tamanho: 6 bytes.
Finalidade: Resposta de medição de latência (ping).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC9 |
| 1 | 1 | u8 | unknown | Valor repetido do pedido |
| 2 | 4 | i32 | tick | Tick count do servidor |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`TripTimeResponse`)
