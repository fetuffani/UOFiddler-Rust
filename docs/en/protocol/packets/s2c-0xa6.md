# 0xA6 - Tip/Notice
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Displays a tip or notice window with text.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA6 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1 | u8 | type | 0 = tip, 1 = ignored by ClassicUO, 2 = notice |
| 4 | 4 | u32 | tipNumber | Tip number / context |
| 8 | 2 | u16 | textLength | Length of the text in bytes |
| 10 | textLength | ascii | text | Text; line breaks with 0x0D |

Notes: Source-X separates the lines of the text with 0x0D and appends a trailing NUL outside of `textLength`. ClassicUO converts '\r' to '\n'. For `type = 1` ClassicUO discards the packet; for 0 and 2 it displays the window in different positions.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ScrollMessage`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketOpenScroll`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`TipWindow`)
