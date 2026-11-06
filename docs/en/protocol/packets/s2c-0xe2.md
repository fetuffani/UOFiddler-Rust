# 0xE2 - New Character Animation
Direction: server -> client. Size: 10 bytes.
Purpose: Plays an animation on a mobile (clients 7.0.0.0+).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE2 |
| 1 | 4 | u32 | serial | Serial of the mobile |
| 5 | 2 | u16 | type | Animation type |
| 7 | 2 | u16 | action | Action |
| 9 | 1 | u8 | mode | Mode/delay between frames (ServUO: delay) |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`NewMobileAnimation`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketActionBasic`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`NewCharacterAnimation`)
