# 0xC2 - Unicode Prompt
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Asks the client for a line of Unicode text.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC2 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Context serial |
| 7 | 4 | u32 | promptId | Prompt id |
| 11 | 4 | u32 | type | 0 |
| 15 | 4 | ascii[4] | language | Language; ServUO sends 0 |
| 19 | 2 | utf16 | text | Initial text; empty = 0x0000 |

Notes: Typical total size: 21 bytes. The reply is the client-to-server 0xC2.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`UnicodePrompt`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketAddPrompt`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UnicodePrompt`)
