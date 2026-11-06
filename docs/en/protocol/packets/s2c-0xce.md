# 0xCE - Undocumented
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet) (as per the table).
Purpose: Id without a known layout.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

Notes: Not specified.

## Implementation reference
- No reference implementation.
