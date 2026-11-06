# 0xB2 - Chat Message
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Message from the chat system (old chat channel).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB2 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 2 | u16 | command | Command/message code (table) |
| 5 | 4 | ascii[4] | language | Language, e.g. "ENU"; zeros if absent |
| 9 | 2*N+2 | utf16be | param1 | First parameter, terminated by 0x0000 |
| ... | 2*M+2 | utf16be | param2 | Second parameter, terminated by 0x0000 |

Notes: Servers send `command = number - 20`. Commands handled by ClassicUO:

| command | Meaning |
|---|---|
| 0x03E8 | Create conference (param1 = channel name; u16 password 0x31 indicates protected) |
| 0x03E9 | Destroy conference |
| 0x03EB | Request user name |
| 0x03EC | Close chat |
| 0x03ED | User name accepted |
| 0x03EE | Add user (u16 type + name) |
| 0x03EF | Remove user |
| 0x03F0 | Clear all players |
| 0x03F1 | Joined conference |
| 0x03F4 | Left conference |
| 0x0025 to 0x0027 | Chat message (u16 type, name, text) |
| 0x0001 to 0x0024, 0x0028 to 0x002C | Default messages with `%1` (and `%2`) replaced by param1 |

## Implementation reference
- ServUO: `Scripts/Services/Chat/Packets.cs` (`ChatMessagePacket`)
- ModernUO: `Projects/UOContent/Engines/Chat/ChatPackets.cs` (`SendChatMessage`)
- Source-X: `src/network/send.cpp` (`PacketChatMessage`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ChatMessage`)
