# 0xF3 - Object Info (SA/new)
Direction: server -> client. Size: 26 bytes (HS clients) | 24 bytes (SA, without the final field).
Purpose: Displays an item, mobile or multi in the world (replaces 0x1A).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF3 |
| 1 | 2 | u16 | command | 0x0001 |
| 3 | 1 | u8 | dataType | 0 item, 1 mobile, 2 multi, 3 damageable item |
| 4 | 4 | u32 | serial | Serial |
| 8 | 2 | u16 | graphic | Graphic (item: 15 bits in SA, 16 bits in HS; multi: 14 bits) |
| 10 | 1 | u8 | graphicOffset | 0 (increment of the graphic) |
| 11 | 2 | u16 | amount | Amount |
| 13 | 2 | u16 | amount2 | Repeats the amount |
| 15 | 2 | u16 | x | X (15 valid bits) |
| 17 | 2 | u16 | y | Y (14 valid bits) |
| 19 | 1 | i8 | z | Z |
| 20 | 1 | u8 | light | Light level of the item / direction of the mobile |
| 21 | 2 | u16 | hue | Hue |
| 23 | 1 | u8 | flags | Flags (table) |
| 24 | 2 | u16 | extra | 0 (only in the 26-byte variant) |

Notes: Item flags: 0x20 movable, 0x80 invisible. Mobile flags: 0x01 paralyzed/frozen, 0x02 female, 0x04 flying, 0x08 yellow health bar/blessed, 0x10 ignores mobiles, 0x40 war mode, 0x80 hidden. ClassicUO always reads the final 2 bytes, so it assumes the 26-byte variant.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`WorldItemSA`, `WorldItemHS`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateWorldEntity`)
- Source-X: `src/network/send.cpp` (`PacketItemWorldNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateItemSA`)
