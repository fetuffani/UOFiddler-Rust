# 0xBD - Client Version

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Informa a versão do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBD |
| 1 | 2 | u16 | length | tamanho total |
| 3 | n | char[] | version | ASCII terminado por NUL, por exemplo "7.0.10.3" |

Notas:

Normalmente é a resposta a um 0xBD enviado pelo servidor. O Source-X lê no máximo 20 caracteres.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ClientVersion`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`ClientVersion`)
- Source-X: `src/network/receive.cpp` (`PacketClientVersion`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ClientVersion`)
