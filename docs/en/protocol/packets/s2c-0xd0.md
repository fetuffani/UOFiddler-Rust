# 0xD0 - Configuration File
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Configuration file sent to the client.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD0 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |

Notes: Not specified: ClassicUO has an empty handler and no server implementation sends it.

## Implementation reference
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ConfigurationFileR`)
