# 0x7E - GodviewQuery

Direção: cliente -> servidor. Tamanho: 2 bytes (fixo). Disponível: em jogo (somente cliente God).

Finalidade: Consulta do modo God view.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7E |
| 1 | 1 | u8 | value | Não especificado |

Notas:

O ServUO apenas registra o valor.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`GodviewQuery`)
