# 0xC0 - Hued Effect
Direção: servidor -> cliente. Tamanho: 36 bytes.
Finalidade: Efeito gráfico com hue e modo de renderização.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC0 |
| 1 | 1 | u8 | type | 0 Moving, 1 Lightning, 2 FixedXYZ, 3 FixedFrom |
| 2 | 4 | u32 | source | Serial de origem |
| 6 | 4 | u32 | target | Serial de destino |
| 10 | 2 | u16 | itemId | Graphic do efeito |
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
| 27 | 1 | u8 | explode | 1 = explode no destino |
| 28 | 4 | u32 | hue | Hue |
| 32 | 4 | u32 | renderMode | Modo de blend (o ClassicUO usa valor mod 7) |

Notas: Mesmo layout do 0x70 (28 bytes) acrescido de `hue` e `renderMode`. O 0xC7 acrescenta campos de partículas (ver s2c-0xc7).

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`HuedEffect`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEffectPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketEffect`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GraphicEffect`)
