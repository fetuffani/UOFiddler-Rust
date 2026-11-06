# 0xB4 - Target Object List
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Shows ghost items that follow the target cursor.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB4 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1 | u8 | allowGround | 1 if the ground can be targeted |
| 4 | 4 | u32 | serial | Cursor serial/id |
| 8 | 2 | u16 | x | X |
| 10 | 2 | u16 | y | Y |
| 12 | 2 | u16 | z | Z |
| 14 | 2 | u16 | count | Number of entries |
| 16 | 10 x count | entry[] | items | Entry: 5 x u16 (itemId, hue, xOffset, yOffset, zOffset) |

Notes: Layout as documented in the project's packet table (packets.md); the click is answered with 0x6C. Not specified in the reference implementations.

## Implementation reference
- Project table: `docs/en/protocol/packets.md` (section 0xB4)
