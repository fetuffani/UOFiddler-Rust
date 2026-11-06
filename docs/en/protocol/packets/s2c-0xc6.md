# 0xC6 - Invalid Map Enable
Direction: server -> client. Size: 1 byte.
Purpose: Enables the invalid map; id only.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC6 |

Notes: No additional fields.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`InvalidMapEnable`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`InvalidMapEnable`)
