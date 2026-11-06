# 0xD0 - ConfigurationFile

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Envio de arquivo de configuração.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD0 |
| 1 | 2 | u16 | length | tamanho total |
| 3 | n | u8[] | data | Não especificado |

Notas:

Os servidores de referência ignoram o conteúdo.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ConfigurationFile`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`ConfigurationFile`)
