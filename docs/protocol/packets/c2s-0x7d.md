# 0x7D - MenuResponse

Direção: cliente -> servidor. Tamanho: 13 bytes (fixo). Disponível: em jogo.

Finalidade: Resposta a um menu de seleção (0x7C do servidor).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7D |
| 1 | 4 | u32 | serial | serial do menu/objeto |
| 5 | 2 | u16 | menuId | id do menu |
| 7 | 2 | u16 | index | entrada escolhida (base 1); 0 = cancelado |
| 9 | 2 | u16 | itemId | gráfico da entrada |
| 11 | 2 | u16 | hue | cor da entrada |

Notas:

Em cancelamento o cliente de referência envia index = 0, itemId = 0 e hue = 0. O ServUO ignora menuId.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`MenuResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`MenuResponse`)
- Source-X: `src/network/receive.cpp` (`PacketMenuChoice`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MenuResponse`, `Send_GrayMenuResponse`)
