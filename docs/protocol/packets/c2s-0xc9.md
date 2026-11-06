# 0xC9 - TripTime

Direção: cliente -> servidor. Tamanho: 6 bytes (fixo). Disponível: em jogo.

Finalidade: Medição de tempo de viagem (ping de rede).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC9 |
| 1 | 1 | u8 | value | valor a ser devolvido |
| 2 | 4 | u32 | unknown | Não especificado |

Notas:

Resposta: pacote 0xC9 (6 bytes) com o valor do offset 1.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`TripTime`)
