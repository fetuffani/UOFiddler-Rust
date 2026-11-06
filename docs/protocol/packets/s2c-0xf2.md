# 0xF2 - Time Sync Response
Direção: servidor -> cliente. Tamanho: 25 bytes.
Finalidade: Resposta de sincronização de tempo.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF2 |
| 1 | 8 | i64 | time1 | Tempo do servidor |
| 9 | 8 | i64 | time2 | Tempo do servidor |
| 17 | 8 | i64 | time3 | Tempo do servidor |

Notas: O ModernUO escreve três vezes `Core.TickCount`; o Source-X escreve três vezes o horário Unix em milissegundos. O significado individual de cada campo não é especificado.

## Referência de implementação
- ModernUO: `Projects/Server/Network/Packets/OutgoingMovementPackets.cs` (`SendTimeSyncResponse`)
- Source-X: `src/network/send.cpp` (`PacketTimeSyncResponse`)
