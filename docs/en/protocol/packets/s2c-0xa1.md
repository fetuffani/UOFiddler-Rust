# 0xA1 - Update Current HP
Direction: server -> client. Size: 9 bytes.
Purpose: Updates the current and maximum Hits value of a mobile.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA1 |
| 1 | 4 | u32 | serial | Mobile serial |
| 5 | 2 | i16 | max | Maximum value |
| 7 | 2 | i16 | current | Current value |

Notes: When normalization is enabled on the server (ServUO `AttributeNormalizer`, default), values for mobiles other than the player itself are sent scaled: `max = 25` and `current = current*25/maximum`. Without normalization, the real values are sent.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`MobileHits`/`MobileHitsN`, `MobileMana`/`MobileManaN`, `MobileStam`/`MobileStamN`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketHealthUpdate`, `PacketManaUpdate`, `PacketStaminaUpdate`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateHitpoints`, `UpdateMana`, `UpdateStamina`)
