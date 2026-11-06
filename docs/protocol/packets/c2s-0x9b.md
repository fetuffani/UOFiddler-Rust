# 0x9B - Help Request

Direção: cliente -> servidor. Tamanho: 258 bytes (fixo). Disponível: em jogo.

Finalidade: Pede ajuda a um GM (botão Help).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9B |
| 1 | 257 | u8[257] | padding | zeros; o servidor não lê o conteúdo |

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`HelpRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`HelpRequest`)
- Source-X: `src/network/receive.cpp` (`PacketHelpPageReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_HelpRequest`)
