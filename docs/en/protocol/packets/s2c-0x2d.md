# 0x2D - Mobile Attributes

Direction: server -> client. Size: 17 bytes.

Purpose: Updates life, mana and stamina (current and maximum) of a mobile.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2D |
| 1 | 4 | u32 | serial | Mobile. |
| 5 | 2 | u16 | hitsMax |  |
| 7 | 2 | u16 | hits |  |
| 9 | 2 | u16 | manaMax |  |
| 11 | 2 | u16 | mana |  |
| 13 | 2 | u16 | stamMax |  |
| 15 | 2 | u16 | stam |  |

Notes:

The values may be normalized (fixed scale) by the server when the normalization option is active.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `MobileAttributes`, `MobileAttributesN`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `MobileAttributes`
