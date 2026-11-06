# 0xDE - Update Mobile Status
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Reports the combat status of a mobile.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDE |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the mobile |
| 7 | 1 | u8 | status | Status code; 1 includes the following field |
| 8 | 4 | u32 | attacker | Serial of the attacker (only if status = 1) |

Notes: Layout according to the ClassicUO parser; no reference server sends it.

## Implementation reference
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateMobileStatus`)
