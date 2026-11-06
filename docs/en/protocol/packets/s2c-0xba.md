# 0xBA - Quest Arrow
Direction: server -> client. Size: 10 bytes (HS) | 6 bytes (earlier).
Purpose: Shows or removes the quest arrow on screen.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBA |
| 1 | 1 | u8 | active | 1 = show, 0 = remove |
| 2 | 2 | u16 | x | Target X coordinate |
| 4 | 2 | u16 | y | Target Y coordinate |
| 6 | 4 | u32 | serial | Associated serial (only clients 7.0.9.0+; 10 bytes) |

Notes: The 6-byte variant ends at `y`. When removing the arrow, the old ServUO sends x = y = -1; the HS variant repeats the coordinates.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`SetArrow`, `CancelArrow`, `SetArrowHS`, `CancelArrowHS`)
- ModernUO: `Projects/UOContent/Skills/Tracking/OutgoingArrowPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketArrowQuest`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayQuestArrow`)
