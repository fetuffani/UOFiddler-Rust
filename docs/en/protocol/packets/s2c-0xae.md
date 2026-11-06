# 0xAE - Unicode Speech
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Delivers a Unicode speech (message) from an object or from the system.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAE |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Speaker serial; 0xFFFFFFFF = system |
| 7 | 2 | u16 | graphic | Speaker's body/graphic; 0xFFFF if not a mobile |
| 9 | 1 | u8 | type | Message type (table) |
| 10 | 2 | u16 | hue | Color; ServUO uses 0x03B2 when 0 |
| 12 | 2 | u16 | font | Font |
| 14 | 4 | ascii[4] | language | Language code, e.g. "ENU" |
| 18 | 30 | ascii[30] | name | Speaker's name, padded with 0x00 |
| 48 | 2*N+2 | utf16be | text | Text in UTF-16 big-endian terminated by 0x0000 |

Notes: Message types (`MessageType`):

| Value | Name |
|---|---|
| 0x00 | Regular |
| 0x01 | System |
| 0x02 | Emote |
| 0x06 | Label |
| 0x07 | Focus |
| 0x08 | Whisper |
| 0x09 | Yell |
| 0x0A | Spell |
| 0x0D | Guild |
| 0x0E | Alliance |
| 0x0F | Command |
| 0xC0 | Encoded |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`UnicodeMessage`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageUNICODE`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UnicodeTalk`)
