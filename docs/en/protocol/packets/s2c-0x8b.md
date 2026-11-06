# 0x8B - Display Sign Gump
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Displays the sign window with a text.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8B |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Sign serial |
| 7 | 2 | u16 | gumpId | Sign gump id |
| 9 | 2 | u16 | text1Length | Length of the first text (includes the trailing NUL in ModernUO) |
| 11 | text1Length | ascii | text1 | First text ("unknown" parameter in the implementations) |
| ... | 2 | u16 | captionLength | Length of the caption, including the trailing NUL |
| ... | captionLength | ascii | caption | Sign text, ends in NUL |

Notes: ServUO writes the first text without NUL (length = number of characters); ModernUO writes it with NUL and adds 1 to the length. ClassicUO has no handler for 0x8B.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`DisplaySignGump`)
- ModernUO: `Projects/UOContent/Gumps/Base/OutgoingGumpPackets.cs` (`SendDisplaySignGump`)
- Source-X: `src/network/send.cpp` (`PacketSignGump`)
