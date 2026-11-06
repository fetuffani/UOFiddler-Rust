# 0xA5 - Open Web Browser
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Abre uma URL no navegador do cliente.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA5 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1+ | ascii | url | URL terminada em NUL |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`LaunchBrowser`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketWebPage`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenUrl`)
