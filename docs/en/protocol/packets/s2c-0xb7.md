# 0xB7 - Help/Tip Data
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Sends the help text of an object.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB7 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the object |
| 7 | 2*N+2 | utf16be | text | Text, terminated by 0x0000 |

Notes: ClassicUO registers an empty handler.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ObjectHelpResponse`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketTooltip`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Help`)
