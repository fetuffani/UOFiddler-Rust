# 0xAA - Attack Target (current)
Direction: server -> client. Size: 5 bytes.
Purpose: Sets the player's current attack target.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAA |
| 1 | 4 | u32 | serial | Target serial; 0 = attack request refused |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ChangeCombatant`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingCombatPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketAttack`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`AttackCharacter`)
