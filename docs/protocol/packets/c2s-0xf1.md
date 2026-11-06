# 0xF1 - Time Sync Request

Direção: cliente -> servidor. Tamanho: 9 bytes (fixo). Disponível: em jogo.

Finalidade: Pede sincronização de tempo.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF1 |
| 1 | 8 | u64 | clientTime | tempo do cliente |

Notas:

Resposta: 0xF2 Time Sync Response (25 bytes: id + 3 x 8 bytes). O ModernUO mantém o tratamento desativado.

Referência de implementação:

- Source-X: `src/network/receive.cpp` (`PacketTimeSyncRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMovementPackets.cs` (`TimeSyncReq`, desativado)
