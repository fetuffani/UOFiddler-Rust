# 0xA8 - Game Server List
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Lista os servidores de jogo disponíveis após o login da conta.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA8 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1 | u8 | flags | ServUO/ModernUO: 0x5D; Source-X: 0xFF. O cliente não usa o valor para o layout |
| 4 | 2 | u16 | count | Quantidade de servidores |
| 6 | 40 x count | entry[] | servers | Ver tabela da entrada |

Notas: Entrada (40 bytes):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 2 | u16 | index | Índice do servidor (ServUO começa em 0; Source-X em 1); devolvido em 0xA0 |
| 2 | 32 | ascii[32] | name | Nome, preenchido com 0x00 |
| 34 | 1 | u8 | percentFull | Lotação em porcentagem |
| 35 | 1 | i8 | timezone | Fuso em horas |
| 36 | 4 | u8[4] | ip | Endereço IPv4 |

Ordem do IP: clientes a partir de 4.0.0 recebem os octetos em ordem natural (primeiro byte = primeiro octeto), como o Source-X envia e o ClassicUO interpreta; clientes anteriores a 4.0.0 esperam a ordem invertida. O ServUO escreve o endereço como inteiro big-endian de valor little-endian (ordem invertida). O cliente usa o endereço apenas para exibição/ping, pois a conexão real vem do 0x8C. Tamanho total: `6 + 40*count`.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`AccountLoginAck`, `ServerInfo`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendAccountLoginAck`)
- Source-X: `src/network/send.cpp` (`PacketServerList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ServerListReceived`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`ServerListEntry.Create`)
