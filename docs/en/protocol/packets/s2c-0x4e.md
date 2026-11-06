# 0x4E - Personal Light Level

Direction: server -> client. Size: 6 bytes.

Purpose: Sets the personal light level of a mobile.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4E |
| 1 | 4 | u32 | serial | Mobile; the client only applies it if it is the player. |
| 5 | 1 | u8 | level | Light level; the client caps it at 0x1E. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `PersonalLightLevel`
- ModernUO: `Projects/Server/Network/Packets/OutgoingLightPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `XCMD_LightPoint`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PersonalLightLevel`
