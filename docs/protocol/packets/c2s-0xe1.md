# 0xE1 - ClientType

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: antes do login e em jogo.

Finalidade: Informa o tipo de cliente (2D, KR, Enhanced).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE1 |
| 1 | 2 | u16 | length | 9 |
| 3 | 2 | u16 | count | 1 |
| 5 | 4 | u32 | clientType | 2 = KR; 3 = Enhanced Client |

Notas:

Layout do ServUO e do Source-X. O ModernUO lê, após o u16 inicial, um u16 de tipo e uma cadeia ASCII de versão. Clientes anteriores à 5.0.9 usam tamanho fixo de 9 bytes.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ClientType`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`ClientType`)
- Source-X: `src/network/receive.cpp` (`PacketClientType`)
