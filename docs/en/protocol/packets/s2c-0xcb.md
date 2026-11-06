# 0xCB - Global Queue Count
Direction: server -> client. Size: 7 bytes.
Purpose: Reports the number of calls available in the global queue.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCB |
| 1 | 2 | i16 | unknown | Auxiliary value |
| 3 | 4 | i32 | count | Number of calls |

Notes: The client displays "There are currently [count] available calls in the global queue.".

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`GQCount`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GlobalQueCount`)
