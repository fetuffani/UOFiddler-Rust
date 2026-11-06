# 0x4F - Overall Light Level

Direção: servidor -> cliente. Tamanho: 2 bytes.

Finalidade: Define o nível de luz global.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4F |
| 1 | 1 | u8 | level | Nível de luz; o cliente limita a 0x1E. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `GlobalLightLevel`
- ModernUO: `Projects/Server/Network/Packets/OutgoingLightPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketGlobalLight`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `LightLevel`
