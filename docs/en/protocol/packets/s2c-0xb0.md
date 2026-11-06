# 0xB0 - Gump Menu Dialog (gump)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Opens a generic gump described by a text layout and a table of texts (uncompressed format).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB0 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Gump owner serial |
| 7 | 4 | u32 | gumpId | Gump type/id, returned in 0xB1 |
| 11 | 4 | i32 | x | Screen X position |
| 15 | 4 | i32 | y | Screen Y position |
| 19 | 2 | u16 | layoutLength | Length in bytes of the layout (ServUO `DisplayGump` includes the trailing NUL) |
| 21 | layoutLength | ascii | layout | Gump commands in the `{ command args }` format; see gump-layout.md |
| 21+layoutLength | 2 | u16 | textCount | Number of text lines |
| ... | variable | line[] | lines | `textCount` lines |

Notes: Line: `u16 charCount` followed by `charCount` UTF-16 big-endian characters (no NUL). The zlib-compressed version is 0xDD. ClassicUO reads the layout as ASCII and each line with `ReadUnicodeBE`.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`DisplayGump`, `DisplayGumpFast`)
- ModernUO: `Projects/UOContent/Gumps/Base/`
- Source-X: `src/network/send.cpp` (`PacketGumpDialog`, `writeStandardControls`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenGump`)
