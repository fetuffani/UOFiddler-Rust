# 0xC8 - SetUpdateRange

Direção: cliente -> servidor. Tamanho: 2 bytes (fixo). Disponível: em jogo.

Finalidade: Informa o alcance de visão desejado.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC8 |
| 1 | 1 | u8 | range | alcance em tiles |

Notas:

O servidor limita o valor aos seus mínimos e máximos e responde com 0xC8 (2 bytes) contendo o alcance efetivo.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`SetUpdateRange`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`SetUpdateRange`)
- Source-X: `src/network/receive.cpp` (`PacketViewRange`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ClientViewRange`)
