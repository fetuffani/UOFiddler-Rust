# 0xC0 - Hued Effect
Direction: server -> client. Size: 36 bytes.
Purpose: Graphical effect with a hue and a render mode.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC0 |
| 1 | 1 | u8 | type | 0 Moving, 1 Lightning, 2 FixedXYZ, 3 FixedFrom |
| 2 | 4 | u32 | source | Source serial |
| 6 | 4 | u32 | target | Target serial |
| 10 | 2 | u16 | itemId | Graphic of the effect |
| 12 | 2 | u16 | srcX | Source X |
| 14 | 2 | u16 | srcY | Source Y |
| 16 | 1 | i8 | srcZ | Source Z |
| 17 | 2 | u16 | dstX | Target X |
| 19 | 2 | u16 | dstY | Target Y |
| 21 | 1 | i8 | dstZ | Target Z |
| 22 | 1 | u8 | speed | Speed |
| 23 | 1 | u8 | duration | Duration |
| 24 | 2 | u16 | reserved | 0 |
| 26 | 1 | u8 | fixedDirection | 1 = fixed direction |
| 27 | 1 | u8 | explode | 1 = explodes at the target |
| 28 | 4 | u32 | hue | Hue |
| 32 | 4 | u32 | renderMode | Blend mode (ClassicUO uses the value mod 7) |

Notes: Same layout as 0x70 (28 bytes) with `hue` and `renderMode` added. 0xC7 adds particle fields (see s2c-0xc7).

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`HuedEffect`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEffectPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketEffect`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GraphicEffect`)
