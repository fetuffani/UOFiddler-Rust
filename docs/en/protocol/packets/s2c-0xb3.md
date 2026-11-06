# 0xB3 - Chat Text
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Text from the old chat system.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB3 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |

Notes: Not specified: no reference implementation sends this packet to the client. In the client-to-server direction Source-X reads `u16 length`, `ascii[4] language` and UTF-16 big-endian text.

## Implementation reference
- Source-X: `src/network/send.cpp` (`PacketChatCommand`, reception)
