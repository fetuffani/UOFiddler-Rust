# 0x8D - ECCreateCharacter

Direction: client -> server. Size: variable (id + u16 length + payload); 146 bytes in Enhanced clients. Available: before login (character selection).

Purpose: Creates a character through the Enhanced Client (KR/SA).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8D |
| 1 | 2 | u16 | length | total size (146) |
| 3 | 4 | u32 | pattern | client pattern |
| 7 | 4 | u32 | slot | character slot |
| 11 | 30 | char[30] | name | ASCII |
| 41 | 30 | char[30] | unknown | Not specified |
| 71 | 1 | u8 | profession | profession |
| 72 | 1 | u8 | cityIndex | starting city |
| 73 | 1 | u8 | gender | 0 = male; non-zero = female |
| 74 | 1 | u8 | race | race (see notes) |
| 75 | 1 | u8 | str | strength |
| 76 | 1 | u8 | dex | dexterity |
| 77 | 1 | u8 | int | intelligence |
| 78 | 2 | u16 | skinHue | skin color |
| 80 | 8 | u8[8] | reserved | zeros |
| 88 | 1 | u8 | skill1 | index of skill 1 |
| 89 | 1 | u8 | skill1Value | value |
| 90 | 1 | u8 | skill2 | skill 2 |
| 91 | 1 | u8 | skill2Value | value |
| 92 | 1 | u8 | skill3 | skill 3 |
| 93 | 1 | u8 | skill3Value | value |
| 94 | 1 | u8 | skill4 | skill 4 |
| 95 | 1 | u8 | skill4Value | value |
| 96 | 26 | u8[26] | reserved | zeros |
| 122 | 2 | u16 | hairHue | hair color |
| 124 | 2 | u16 | hairId | hair style |
| 126 | 6 | u8[6] | reserved | Not specified |
| 132 | 2 | u16 | shirtHue | shirt color |
| 134 | 2 | u16 | shirtId | shirt id |
| 136 | 1 | u8 | reserved | Not specified |
| 137 | 2 | u16 | faceHue | face color |
| 139 | 2 | u16 | faceId | face id |
| 141 | 1 | u8 | reserved | Not specified |
| 142 | 2 | u16 | beardHue | beard color |
| 144 | 2 | u16 | beardId | beard style |

Notes:

`race`: ServUO uses (race - 1) as the race index (SA clients send a value above KR); Source-X subtracts 1 only for KR clients. The resulting index values follow the server's race numbering (0 human, 1 elf, 2 gargoyle).
When a profession is chosen, the packet does not carry consistent attributes and skills; Source-X derives stats and skills from the profession.
ServUO registers this id with variable size; Source-X expects 146 bytes. ModernUO does not register this id.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ECCreateCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCreateNew`)
