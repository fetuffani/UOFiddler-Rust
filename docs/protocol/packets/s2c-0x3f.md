# 0x3F - Update Statics (God client; UltimaLive command)

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Pacote do protocolo UltimaLive: atualiza statics de um bloco ou executa um comando UltimaLive.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3F |
| 1 | 2 | u16 | length | Tamanho total. |
| 3 | 4 | u32 | blockNumber | Número do bloco (x * alturaEmBlocos + y). |
| 7 | 4 | u32 | count | Número de unidades de 7 bytes no payload. |
| 11 | 2 | u16 | sequence | Sequência/segurança (0 nas implementações). |
| 13 | 1 | u8 | command | Comando UltimaLive (tabela). |
| 14 | 1 | u8 | mapId | Número do mapa. |
| 15 | n | u8[] | data | Payload do comando. |

Notas:

| command | Significado | Payload |
|---|---|---|
| 0x00 | Update Statics | `count` statics de 7 bytes |
| 0x01 | Update Map Definitions | entradas de 9 bytes, completadas com zeros até múltiplo de 7 |
| 0x02 | Login Confirmation | 28 bytes ASCII com o identificador do shard, completados com 0x00 |
| 0x03 | Refresh Client | vazio (15 bytes no total) |
| 0xFF | Block Query | vazio (15 bytes); bloco central da consulta de 25 blocos |

Static (7 bytes): `artId` u16, `x` u8 (offset no bloco), `y` u8, `z` i8, `hue` u16.
Map definition (9 bytes): `fileIndex` u8, `width` u16, `height` u16, `wrapWidth` u16, `wrapHeight` u16.
Login Confirmation total: 43 bytes. Block Query e Refresh: 15 bytes.

## Referência de implementação

- ModernUO: `refs/modernuo/OutgoingUltimaLivePackets.cs` (UltimaLive)
- Praxiiz UltimaLive: `UltimaLive/Protocol.txt`, `UltimaLive/Network/NetworkManager.cpp` (byte de comando no offset 13)
- Source-X: `src/network/send.cpp`, `PacketQueryClient`
