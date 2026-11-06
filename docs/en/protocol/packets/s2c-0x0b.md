# 0x0B - Damage

Direction: server -> client. Size: 7 bytes.

Purpose: Informs the client of the amount of damage taken by an entity (displayed as a floating number).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x0B |
| 1 | 4 | u32 | serial | Entity that took damage. |
| 5 | 2 | u16 | damage | Damage; 0 displays nothing. Limited to 0..0xFFFF. |

Notes:

Old variant: subcommand 0x22 of packet 0xBF (serial u32 + damage u8, limited to 255); see packet 0xBF.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `DamagePacket`, `DamagePacketOld`
- ModernUO: `Projects/Server/Network/Packets/OutgoingDamagePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketCombatDamage`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Damage`
