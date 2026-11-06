# 0xAF - Display Death Action
Direction: server -> client. Size: 13 bytes.
Purpose: Shows the death animation of a mobile and links the corpse.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAF |
| 1 | 4 | u32 | serial | Dead mobile serial |
| 5 | 4 | u32 | corpse | Corpse serial (0 if none) |
| 9 | 4 | u32 | flag | ServUO: 0. Source-X: 1 if it fell forward. ClassicUO reads the field and does not use it |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`DeathAnimation`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketDeath`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayDeath`)
