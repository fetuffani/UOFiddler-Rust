# 0xDF - Buff/Debuff
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Adds or removes a buff/debuff icon.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDF |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the mobile |
| 7 | 2 | u16 | iconId | Icon id (BuffIcon) |
| 9 | 2 | u16 | count | 0 = remove the icon; 1 = add (one entry follows) |
| 11 | variable | entry | entry | Present only if count > 0 |

Notes: Removal: `count = 0` followed by 4 zeroed bytes (ServUO: 13 bytes; ModernUO: 15 bytes with length).

Add entry (count = 1):

| Size | Type | Field |
|---|---|---|
| 2 | u16 | sourceType (0) |
| 2 | u16 | reserved (0) |
| 2 | u16 | iconId (repeated) |
| 2 | u16 | queueIndex (ServUO sends 1) |
| 4 | u32 | reserved (0) |
| 2 | u16 | duration in seconds (0 = no timer) |
| 3 | u8[3] | reserved (0) |
| 4 | u32 | titleCliloc |
| 4 | u32 | secondaryCliloc (description) |
| 4 | u32 | third cliloc (0) |
| 2+ | u16 + utf16le | arguments 1: length, UTF-16LE text terminated by NUL |
| 2+ | u16 + utf16le | arguments 2 |
| 2+ | u16 + utf16le | arguments 3 |

Without arguments, the server writes 6 zeroed bytes after the third cliloc (3 lengths of 0). With arguments, ServUO and ModernUO write: length `0x0001`, `0x0000`, `'\t'` followed by the UTF-16LE text with NUL, then `0x0001` and `0x0000`. The exact meaning of the length fields is not specified. ClassicUO converts the id: ids >= 0x466 and < 0x3E9 follow distinct tables.

## Implementation reference
- ServUO: `Scripts/Misc/BuffIcons.cs` (`AddBuffPacket`, `RemoveBuffPacket`)
- ModernUO: `Projects/UOContent/Engines/BuffIcons/BuffIconPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketBuff`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BuffDebuff`)
