# 0xE6 - Remove Waypoint
Direction: server -> client. Size: 5 bytes.
Purpose: Removes a waypoint from the radar map.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE6 |
| 1 | 4 | u32 | serial | Serial of the waypoint |

## Implementation reference
- ServUO: `Scripts/Misc/Waypoints.cs` (`RemoveWaypoint`)
- Source-X: `src/network/send.cpp` (`PacketWaypointRemove`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`RemoveWaypoint`)
