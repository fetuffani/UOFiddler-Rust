# 0x2F - Fight Occurring

Direction: server -> client. Size: 10 bytes.

Purpose: Indicates a swing between attacker and defender.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2F |
| 1 | 1 | u8 | flag | 0 in ServUO; ignored by the client. |
| 2 | 4 | u32 | attacker | Attacker. |
| 6 | 4 | u32 | defender | Defender. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `Swing`
- ModernUO: `Projects/Server/Network/Packets/OutgoingCombatPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketSwing`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Swing`
