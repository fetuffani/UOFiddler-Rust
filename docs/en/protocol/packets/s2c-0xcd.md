# 0xCD - Undocumented (1 byte)
Direction: server -> client. Size: 1 byte.
Purpose: Id without a known layout (client 7.0.10.3 discards it).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCD |

Notes: Not specified: there are no fields beyond the id.

## Implementation reference
- No reference implementation.
