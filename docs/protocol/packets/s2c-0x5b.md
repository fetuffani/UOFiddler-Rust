# 0x5B - Time

Direção: servidor -> cliente. Tamanho: 4 bytes.

Finalidade: Informa a hora do jogo.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x5B |
| 1 | 1 | u8 | hour |  |
| 2 | 1 | u8 | minute |  |
| 3 | 1 | u8 | second |  |

Notas:

O ClassicUO não processa este pacote.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `CurrentTime`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendCurrentTime`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketGameTime`
