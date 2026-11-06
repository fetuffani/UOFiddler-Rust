# 0xF8 - CreateCharacter70160

Direction: client -> server. Size: 106 bytes (fixed). Available: before login (character selection).

Purpose: Creates a character (clients 7.0.16.0 or later, 4 skills).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF8 |
| 1 | 4 | u32 | constant | 0xEDEDEDED |
| 5 | 4 | u32 | constant | 0xFFFFFFFF |
| 9 | 1 | u8 | reserved | 0x00 |
| 10 | 30 | char[30] | name | ASCII |
| 40 | 2 | u8[2] | reserved | zeros |
| 42 | 4 | u32 | flags | client protocol flags |
| 46 | 8 | u8[8] | reserved | the reference client sends 0x00000001 and 0x00000000 |
| 54 | 1 | u8 | profession | profession |
| 55 | 15 | u8[15] | reserved | zeros |
| 70 | 1 | u8 | genderRace | 2/3 human M/F; 4/5 elf M/F; 6/7 gargoyle M/F |
| 71 | 1 | u8 | str | strength |
| 72 | 1 | u8 | dex | dexterity |
| 73 | 1 | u8 | int | intelligence |
| 74 | 8 | 4 x (u8, u8) | skills | four (index, value) pairs |
| 82 | 2 | u16 | skinHue | skin color |
| 84 | 2 | u16 | hairId | hair |
| 86 | 2 | u16 | hairHue | hair color |
| 88 | 2 | u16 | facialHairId | facial hair |
| 90 | 2 | u16 | facialHairHue | facial hair color |
| 92 | 1 | u8 | reserved | zero |
| 93 | 1 | u8 | cityIndex | starting city |
| 94 | 4 | u32 | slot | character slot |
| 98 | 4 | u32 | clientIp | client IPv4 |
| 102 | 2 | u16 | shirtHue | shirt color |
| 104 | 2 | u16 | pantsHue | pants color |

Notes:

Same layout as 0x00 with a fourth skill pair; all fields from offset 82 are shifted by 2 bytes. The packet table does not fix a size for this id; servers register it with 106 bytes.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`CreateCharacter70160`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`CreateCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCreateHS`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CreateCharacter`)
