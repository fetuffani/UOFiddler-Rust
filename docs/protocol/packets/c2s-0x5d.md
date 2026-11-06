# 0x5D - Play Character

Direção: cliente -> servidor. Tamanho: 73 bytes (fixo). Disponível: antes do login (seleção de personagem).

Finalidade: Seleciona o personagem com que entrar no mundo.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x5D |
| 1 | 4 | u32 | pattern | 0xEDEDEDED |
| 5 | 30 | char[30] | name | nome do personagem (ASCII) |
| 35 | 2 | u8[2] | reservado | zeros |
| 37 | 4 | u32 | flags | flags de protocolo do cliente |
| 41 | 24 | u8[24] | reservado | zeros nos clientes oficiais; ver notas |
| 65 | 4 | u32 | slot | slot do personagem na conta |
| 69 | 4 | u32 | clientIp | IPv4 do cliente |

Notas:

Os 24 bytes do offset 41 incluem, no Source-X, 4 bytes de flags adicionais, 4 bytes de contagem de logins e 16 bytes de uso não especificado (0xFF000000 no Enhanced Client). Assistentes de terceiros podem usá-los para transportar dados próprios; o ServUO opcionalmente lê 8 bytes de flags de recursos e 16 bytes de chave de autenticação nessa região.
Resposta: 0x1B Login Confirm e a sequência de entrada no mundo. Se o slot for inválido, o servidor encerra a conexão.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`PlayCharacter`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`PlayCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCharPlay`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SelectCharacter`)
