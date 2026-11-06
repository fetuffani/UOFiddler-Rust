# 0x4E - Personal Light Level

Direção: servidor -> cliente. Tamanho: 6 bytes.

Finalidade: Define o nível de luz pessoal de um mobile.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4E |
| 1 | 4 | u32 | serial | Mobile; o cliente só aplica se for o jogador. |
| 5 | 1 | u8 | level | Nível de luz; o cliente limita a 0x1E. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `PersonalLightLevel`
- ModernUO: `Projects/Server/Network/Packets/OutgoingLightPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `XCMD_LightPoint`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PersonalLightLevel`
