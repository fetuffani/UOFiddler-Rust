# 0xB8 - Character Profile
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Delivers the profile of a character.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the character (0 if hidden) |
| 7 | N+1 | ascii | header | Name/header, terminated by NUL |
| ... | 2*M+2 | utf16be | footer | Title/footer, terminated by 0x0000 |
| ... | 2*K+2 | utf16be | body | Profile text, terminated by 0x0000 |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`DisplayProfile`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketProfile`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CharacterProfile`)
