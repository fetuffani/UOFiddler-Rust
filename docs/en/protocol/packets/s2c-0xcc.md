# 0xCC - Localized Message Affix
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Cliloc message with additional text (prefix or suffix).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCC |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the speaker |
| 7 | 2 | u16 | graphic | Graphic |
| 9 | 1 | u8 | type | Message type |
| 10 | 2 | u16 | hue | Color |
| 12 | 2 | u16 | font | Font |
| 14 | 4 | u32 | cliloc | Cliloc number |
| 18 | 1 | u8 | affixType | Flags (table) |
| 19 | 30 | ascii[30] | name | Name |
| 49 | N+1 | ascii | affix | Additional text terminated by NUL |
| ... | 2*M+2 | utf16be | arguments | Arguments terminated by 0x0000 (big-endian) |

Notes: `affixType`: 0x00 appends to the end, 0x01 places it at the beginning, 0x02 marks the message as a system message. Unlike 0xC1, the arguments are big-endian (ServUO uses little-endian for Enhanced clients).

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`MessageLocalizedAffix`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageLocalisedEx`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayClilocString`)
