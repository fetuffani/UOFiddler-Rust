# 0x7B - Sequence

Direction: server -> client. Size: 2 bytes.

Purpose: Reports a sequence number.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7B |
| 1 | 1 | u8 | sequence | Sequence number. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `Sequence`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendSequence`)
