# 0x0A - Edit

Direção: cliente -> servidor. Tamanho: 11 bytes (fixo). Disponível: em jogo (somente cliente God).

Finalidade: Edição de mapa/estáticos pelo cliente God.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x0A |
| 1 | 1 | u8 | type | 10 = estático, 7 = NPC, 4 = dinâmico |
| 2 | 2 | i16 | x | posição X |
| 4 | 2 | i16 | y | posição Y |
| 6 | 2 | i16 | graphic | id do gráfico |
| 8 | 1 | i8 | z | posição Z |
| 9 | 2 | u16 | hue | cor |

Notas:

O ServUO apenas registra o conteúdo e desconecta contas sem privilégio de staff.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`Edit`)
