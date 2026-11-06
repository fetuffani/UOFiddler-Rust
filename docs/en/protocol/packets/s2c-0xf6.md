# 0xF6 - Boat Moving
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Moves a boat and the objects on board in a single packet.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF6 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the boat |
| 7 | 1 | u8 | speed | Speed |
| 8 | 1 | u8 | moveDir | Movement direction |
| 9 | 1 | u8 | facing | Facing direction of the boat |
| 10 | 2 | u16 | x | Boat X |
| 12 | 2 | u16 | y | Boat Y |
| 14 | 2 | u16 | z | Boat Z |
| 16 | 2 | u16 | count | Number of objects on board |
| 18 | 10 x count | entry[] | objects | Entry: u32 serial, u16 x, u16 y, u16 z |

Notes: Total size: `18 + 10*count`. The boat itself does not appear in the list. ClassicUO uses the three speed/direction bytes for smooth movement.

## Implementation reference
- ServUO: `Scripts/Multis/Boats/BaseBoat.cs` (`MoveBoatHS`)
- ModernUO: `Projects/UOContent/Multis/Boats/BoatPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMoveShip`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BoatMoving`)
- Project document: `docs/protocol/boats-smooth-sailing.md`
