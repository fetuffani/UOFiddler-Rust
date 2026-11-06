# 0xA7 - Request Tip

Direção: cliente -> servidor. Tamanho: 4 bytes (fixo). Disponível: em jogo.

Finalidade: Pede a dica do dia anterior/seguinte.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA7 |
| 1 | 2 | u16 | lastTip | índice da dica exibida |
| 3 | 1 | u8 | direction | 0 = anterior; 1 = próxima |

Notas:

O ServUO lê os campos e não responde; o Source-X responde com a dica correspondente.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`RequestScrollWindow`)
- Source-X: `src/network/receive.cpp` (`PacketTipReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TipRequest`)
