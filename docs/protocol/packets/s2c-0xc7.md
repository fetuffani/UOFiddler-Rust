# 0xC7 - 3D Particle Effect
Direção: servidor -> cliente. Tamanho: 49 bytes.
Finalidade: Efeito de partículas 3D.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC7 |
| 1 | 1 | u8 | type | 0 Moving, 1 Lightning, 2 FixedXYZ, 3 FixedFrom |
| 2 | 4 | u32 | source | Serial de origem |
| 6 | 4 | u32 | target | Serial de destino |
| 10 | 2 | u16 | itemId | Graphic |
| 12 | 2 | u16 | srcX | X de origem |
| 14 | 2 | u16 | srcY | Y de origem |
| 16 | 1 | i8 | srcZ | Z de origem |
| 17 | 2 | u16 | dstX | X de destino |
| 19 | 2 | u16 | dstY | Y de destino |
| 21 | 1 | i8 | dstZ | Z de destino |
| 22 | 1 | u8 | speed | Velocidade |
| 23 | 1 | u8 | duration | Duração |
| 24 | 2 | u16 | reserved | 0 |
| 26 | 1 | u8 | fixedDirection | 1 = direção fixa |
| 27 | 1 | u8 | explode | 1 = explode |
| 28 | 4 | u32 | hue | Hue |
| 32 | 4 | u32 | renderMode | Modo de renderização |
| 36 | 2 | u16 | effect | Id do efeito de partícula |
| 38 | 2 | u16 | explodeEffect | Id do efeito da explosão |
| 40 | 2 | u16 | explodeSound | Som da explosão |
| 42 | 4 | u32 | serial | Serial do objeto vinculado |
| 46 | 1 | u8 | layer | Layer vinculado |
| 47 | 2 | u16 | unknown | Não especificado |

Notas: Os primeiros 36 bytes coincidem com o 0xC0.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ParticleEffect`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEffectPackets.cs`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GraphicEffect`)
