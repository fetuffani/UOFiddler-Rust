# 0xD1 - LogoutReq

Direção: cliente -> servidor. Tamanho: 2 bytes (fixo). Disponível: em jogo.

Finalidade: Notifica que o jogador está saindo do jogo.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD1 |
| 1 | 1 | u8 | value | 0x00 |

Notas:

Resposta: 0xD1 Logout Status (2 bytes).

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`LogoutReq`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`LogoutReq`)
- Source-X: `src/network/receive.cpp` (`PacketLogout`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_LogoutNotification`)
