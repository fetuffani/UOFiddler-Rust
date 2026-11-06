# 0x5B - Time

Direction: server -> client. Size: 4 bytes.

Purpose: Informs the game time.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x5B |
| 1 | 1 | u8 | hour |  |
| 2 | 1 | u8 | minute |  |
| 3 | 1 | u8 | second |  |

Notes:

ClassicUO does not process this packet.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `CurrentTime`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendCurrentTime`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketGameTime`
