# 0x8C - Connect To Game Server
Direção: servidor -> cliente. Tamanho: 11 bytes.
Finalidade: Informa ao cliente o endereço do servidor de jogo e a chave de autenticação; o cliente reconecta e usa a chave como seed.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8C |
| 1 | 4 | u8[4] | ip | Endereço IPv4 em ordem natural: primeiro byte = primeiro octeto |
| 5 | 2 | u16 | port | Porta do servidor de jogo |
| 7 | 4 | u32 | authId | Chave de autenticação (seed da nova conexão) |

Notas: O ClassicUO lê o IP como u32 little-endian (equivale à ordem natural dos octetos), a porta como u16 big-endian e a chave como u32 big-endian; ao reconectar envia a chave como primeiros 4 bytes da nova conexão e depois o login (0x91). O cliente passa a usar criptografia/compressão de jogo na nova conexão.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`PlayServerAck`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendPlayServerAck`)
- Source-X: `src/network/send.cpp` (`PacketServerRelay`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ReceiveServerRelay`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`HandleRelayServerPacket`)
