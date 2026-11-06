# 0xDA - Mahjong (game packets)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Mahjong game packets; a one-byte subcommand selects the format.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDA |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Game serial |
| 7 | 1 | u8 | reserved | 0 |
| 8 | 1 | u8 | subcommand | Subcommand (table) |
| 9 | variable | u8[] | payload | Depends on the subcommand |

Notes: Subcommands sent by the server:

| Sub | Name | Payload (from offset 9) |
|---|---|---|
| 0x02 | Players info | `u8 0`, `u8 seats`, then `seats` entries of 45 bytes (below) |
| 0x03 | Tile info | 1 tile of 9 bytes (below) |
| 0x04 | Tiles info | `u16 count`, then `count` tiles of 9 bytes |
| 0x05 | General info | `u16 0`, `u8 0`, `u8 flags` (bit 0x01 show scoreboard, 0x02 spectator view), `u8 data1`, `u8 data2`, `u8 dealerWind`, `u16 dealerY`, `u16 dealerX`, `u8 dealerDirection`, `u16 wallBreakY`, `u16 wallBreakX` |
| 0x19 | Join game | no payload (total size 9) |
| 0x1A | Relief | no payload (total size 9) |

Player entry (45 bytes): `u32 serial` (0 for an empty seat), `u8 role` (1 dealer, 2 others), `u8 seat`, `i32 score`, `u16 0`, `u8 0`, `u8 public`, `ascii[30] name`, `u8 spectator` (1 if not playing).

Tile (9 bytes): `u8 number`, `u8 value` (0 if hidden), `u16 y`, `u16 x`, `u8 stack level`, `u8 direction`, `u8 flags` (0x10 = flipped).

ServUO applies the Y order before X in the positions.

## Implementation reference
- ServUO: `Scripts/Items/Containers/Mahjong/Packets.cs`
- ModernUO: `Projects/UOContent/Items/Games/Mahjong/MahjongPackets.cs`
