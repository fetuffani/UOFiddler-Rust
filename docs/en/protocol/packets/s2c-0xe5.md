# 0xE5 - Display Waypoint
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Adds a waypoint to the radar map (KR/SA clients).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE5 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the object |
| 7 | 2 | u16 | x | X |
| 9 | 2 | u16 | y | Y |
| 11 | 1 | i8 | z | Z |
| 12 | 1 | u8 | map | Map id |
| 13 | 2 | u16 | type | Type (table) |
| 15 | 2 | u16 | ignoreObject | 1 = ignore the object |
| 17 | 4 | u32 | cliloc | 1046414 (corpse) or 1062613 in ServUO; Source-X: 1028198 or 1062613 |
| 21 | 2*N+2 | utf16le | name | Name terminated by 0x0000 (ServUO appends one more u16 0) |

Notes: Types: 0x01 Corpse, 0x02 PartyMember, 0x03 RallyPoint, 0x04 QuestGiver, 0x05 QuestDestination, 0x06 Resurrection, 0x07 PointOfInterest, 0x08 Landmark, 0x09 Town, 0x0A Dungeon, 0x0B Moongate, 0x0C Shop, 0x0D Player.

## Implementation reference
- ServUO: `Scripts/Misc/Waypoints.cs` (`DisplayWaypoint`)
- Source-X: `src/network/send.cpp` (`PacketWaypointAdd`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayWaypoint`)
