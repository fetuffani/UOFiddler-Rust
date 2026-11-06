# 0xEA - Toggle Hotbar
Direction: server -> client. Size: 3 bytes.
Purpose: Turns the KR client's hotbar on or off.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEA |
| 1 | 2 | u16 | enable | 1 = enable, 0 = disable |

Notes: Sent only to KR clients.

## Implementation reference
- Source-X: `src/network/send.cpp` (`PacketToggleHotbar`)
