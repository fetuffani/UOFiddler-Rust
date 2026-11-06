# 0xC3 - GQ Request
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Help request for the global queue (GQ) system.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC3 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | variable | u8[] | payload | Sequence of integers and a `region` string, according to the ServUO constructor |

Notes: Sequence written by ServUO after the header: 4 x i32, i32, i16, i16 (length 6), 6 ASCII bytes "region", i32 (call time in seconds), i16 (map), i32 x, i32 y, i32 z, i32 volume, i32 rank, i32 (-1), i32 type. The field names come from the comments in the constructor, with test values. ClassicUO does not register a handler.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`GQRequest`)
