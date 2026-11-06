# 0x4F - Overall Light Level

Direction: server -> client. Size: 2 bytes.

Purpose: Sets the global light level.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4F |
| 1 | 1 | u8 | level | Light level; the client caps it at 0x1E. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `GlobalLightLevel`
- ModernUO: `Projects/Server/Network/Packets/OutgoingLightPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketGlobalLight`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `LightLevel`
