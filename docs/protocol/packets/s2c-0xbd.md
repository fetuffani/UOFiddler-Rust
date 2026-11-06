# 0xBD - Client Version request
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Pede ao cliente que informe sua versão.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBD |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |

Notas: Tamanho total 3 bytes (`BD 00 03`). O cliente responde com o 0xBD de cliente para servidor (string de versão).

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ClientVersionReq`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendClientVersionRequest`)
- Source-X: `src/network/send.cpp` (`PacketClientVersionReq`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ClientVersion`)
