# 0xC4 - Semivisible
Direction: server -> client. Size: 6 bytes.
Purpose: Reserved id.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC4 |
| 1 | 5 | u8[5] | data | Not specified |

Notes: ClassicUO has an empty handler; no implementation sends the packet.

## Implementation reference
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Semivisible`)
