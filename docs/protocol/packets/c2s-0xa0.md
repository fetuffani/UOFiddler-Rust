# 0xA0 - Select Server

Direção: cliente -> servidor. Tamanho: 3 bytes (fixo). Disponível: antes do login.

Finalidade: Seleciona o servidor de jogo na lista recebida em 0xA8.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA0 |
| 1 | 2 | u16 | index | índice do servidor na lista 0xA8 |

Notas:

Resposta: 0x8C Connect To Game Server (endereço, porta e chave de autenticação). O cliente então abre nova conexão e envia 0x91.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`PlayServer`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`PlayServer`)
- Source-X: `src/network/receive.cpp` (`PacketServerSelect`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SelectServer`)
