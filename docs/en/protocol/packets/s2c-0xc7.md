# 0xC7 - 3D Particle Effect
Direction: server -> client. Size: 49 bytes.
Purpose: 3D particle effect.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC7 |
| 1 | 1 | u8 | type | 0 Moving, 1 Lightning, 2 FixedXYZ, 3 FixedFrom |
| 2 | 4 | u32 | source | Source serial |
| 6 | 4 | u32 | target | Target serial |
| 10 | 2 | u16 | itemId | Graphic |
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
| 27 | 1 | u8 | explode | 1 = explodes |
| 28 | 4 | u32 | hue | Hue |
| 32 | 4 | u32 | renderMode | Render mode |
| 36 | 2 | u16 | effect | Particle effect id |
| 38 | 2 | u16 | explodeEffect | Explosion effect id |
| 40 | 2 | u16 | explodeSound | Explosion sound |
| 42 | 4 | u32 | serial | Serial of the linked object |
| 46 | 1 | u8 | layer | Linked layer |
| 47 | 2 | u16 | unknown | Not specified |

Notes: The first 36 bytes match 0xC0.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ParticleEffect`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEffectPackets.cs`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GraphicEffect`)
