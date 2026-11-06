# 0xAB - Gump Text Entry Dialog
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Opens a text or number entry window.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAB |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Context object serial |
| 7 | 1 | u8 | parentId | Parent dialog id |
| 8 | 1 | u8 | buttonId | Button id |
| 9 | 2 | u16 | textLength | Length of the main text |
| 11 | textLength | ascii | text | Main text |
| ... | 1 | u8 | cancel | 1 if a cancel button exists |
| ... | 1 | u8 | style | 0 = no editing, 1 = text, 2 = numeric (Source-X values) |
| ... | 4 | u32 | maxLength | Maximum length or maximum value |
| ... | 2 | u16 | descLength | Length of the description |
| ... | descLength | ascii | description | Description (Source-X includes the NUL) |

Notes: Source-X sends `textLength` including the trailing NUL and writes the `parentId` + `buttonId` pair as a constant u16 (cursor mode value). The response is 0xAC (client to server).

## Implementation reference
- Source-X: `src/network/send.cpp` (`PacketGumpValueInput`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`TextEntryDialog`)
