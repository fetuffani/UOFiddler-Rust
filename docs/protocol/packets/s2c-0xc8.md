# 0xC8 - Update Range
Direção: servidor -> cliente. Tamanho: 2 bytes.
Finalidade: Define o alcance de visão do cliente.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC8 |
| 1 | 1 | u8 | range | Alcance em casas |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ChangeUpdateRange`)
- Source-X: `src/network/send.cpp` (`PacketVisualRange`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ClientViewRange`)
