# 0xB5 - Open Chat Window
Direction: server -> client. Size: 64 bytes.
Purpose: Opens the old chat window.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB5 |
| 1 | 63 | u8[63] | data | Not specified |

Notes: Fixed size of 64 bytes. The content is not specified by the implementations; Source-X receives the same id (size 64) from the client when the chat button is pressed.

## Implementation reference
- Source-X: `src/network/send.cpp` (`PacketChatButton`, reception)
