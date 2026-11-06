# 0x2C - DeathStatusResponse

Direção: cliente -> servidor. Tamanho: 2 bytes (fixo). Disponível: em jogo.

Finalidade: Resposta do jogador à tela de morte.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2C |
| 1 | 1 | u8 | action | ver notas |

Notas:

Valores no Source-X: 0x01 ressuscitar, 0x02 continuar como fantasma; o cliente de referência envia 0x02. O ServUO ignora o pacote.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`DeathStatusResponse`)
- Source-X: `src/network/receive.cpp` (`PacketDeathStatus`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_DeathScreen`)
