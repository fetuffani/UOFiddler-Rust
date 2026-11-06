# 0x9A - Console Entry Prompt
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Asks the client for a line of ASCII text (console prompt).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9A |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Context serial (object asking) |
| 7 | 4 | u32 | promptId | Prompt id, returned in the response |
| 11 | 4 | u32 | type | 0 |
| 15 | 1+ | ascii | text | Text, terminated by NUL (usually empty: just 0x00) |

Notes: Source-X sends `serial`, `promptId`, `0` and a NUL. ClassicUO stores the first two u32 values as the prompt identification. The Unicode version is 0xC2.

## Implementation reference
- Source-X: `src/network/send.cpp` (`PacketAddPrompt`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ASCIIPrompt`)
