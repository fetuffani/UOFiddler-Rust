# 0xFB - PublicHouseContent

Direção: cliente -> servidor. Tamanho: 2 bytes (fixo). Disponível: antes do login e em jogo.

Finalidade: Mostra ou oculta o conteúdo de casas públicas.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xFB |
| 1 | 1 | u8 | show | 0 = ocultar; 1 = mostrar |

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`PublicHouseContent`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingHousePackets.cs` (`ShowPublicHouseContent`)
- Source-X: `src/network/receive.cpp` (`PacketPublicHouseContent`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ShowPublicHouseContent`)
