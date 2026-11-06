# 0x6C - Target Response

Direção: cliente -> servidor. Tamanho: 19 bytes (fixo). Disponível: em jogo.

Finalidade: Responde a um cursor de alvo (0x6C do servidor): objeto, posição ou cancelamento.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6C |
| 1 | 1 | u8 | targetType | 0 = objeto; 1 = posição/estático |
| 2 | 4 | u32 | cursorId | id do cursor recebido no pedido de alvo |
| 6 | 1 | u8 | cursorType | 0 neutro, 1 prejudicial, 2 benéfico, 3 cancelar |
| 7 | 4 | u32 | serial | serial do objeto (0 se posição) |
| 11 | 2 | u16 | x | X |
| 13 | 2 | u16 | y | Y |
| 15 | 2 | i16 | z | Z |
| 17 | 2 | u16 | graphic | id do gráfico do estático/objeto (0 = terreno) |

Notas:

Cancelamento: x = 0xFFFF, y = 0xFFFF e serial = 0 (o cliente de referência também envia z = 0 e graphic = 0). O ServUO valida cursorId contra o alvo pendente.
Em alvo de posição, graphic = 0 indica terreno; diferente de 0 indica um estático naquela coordenada.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`TargetResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingTargetingPackets.cs` (`TargetResponse`)
- Source-X: `src/network/receive.cpp` (`PacketTarget`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TargetObject`, `Send_TargetXYZ`, `Send_TargetCancel`)
