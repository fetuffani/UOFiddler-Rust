# 0xA9 - Characters + Starting Locations
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Delivers the character list, the starting cities and the client's feature flags.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA9 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1 | u8 | charCount | Number of character slots (1, 5, 6 or 7) |
| 4 | 60 x charCount | entry[] | characters | Slot: ASCII[30] name + 30 zeroed bytes; free slot = 60 zeros |
| 4+60*charCount | 1 | u8 | cityCount | Number of starting cities |
| ... | cityCount x (63 or 89) | city[] | cities | Format depends on the client version (below) |
| ... | 4 | u32 | flags | Character list flags (table below) |
| ... | 2 | i16 | lastSlot | Only clients 7.0.13.0+ / Enhanced: slot of the last character (ServUO sends -1) |

City, client from 7.0.13.0 (89 bytes):

| Offset | Size | Type | Field |
|---|---|---|---|
| 0 | 1 | u8 | index |
| 1 | 32 | ascii[32] | city |
| 33 | 32 | ascii[32] | building |
| 65 | 4 | i32 | x |
| 69 | 4 | i32 | y |
| 73 | 4 | i32 | z |
| 77 | 4 | i32 | mapId |
| 81 | 4 | u32 | description cliloc |
| 85 | 4 | u32 | 0 |

City, earlier clients (63 bytes): `u8 index`, `ascii[31] city`, `ascii[31] building`. The older client reads the position and description from local files (`citytext.enu`).

Flags (`CharacterListFlags`):

| Value | Name |
|---|---|
| 0x0001 | Unk1 |
| 0x0002 | OverwriteConfigButton |
| 0x0004 | OneCharacterSlot |
| 0x0008 | ContextMenus |
| 0x0010 | SlotLimit |
| 0x0020 | AOS |
| 0x0040 | SixthCharacterSlot |
| 0x0080 | SE |
| 0x0100 | ML |
| 0x0200 | Unk2 |
| 0x0400 | UO3DClientType |
| 0x0600 | KR (combination 0x200 + 0x400) |
| 0x0800 | Unk3 |
| 0x1000 | SeventhCharacterSlot |
| 0x2000 | Unk4 |
| 0x4000 | NewMovementSystem |
| 0x8000 | NewFeluccaAreas |

Notes: ClassicUO chooses the 89-byte format when the client version is >= 7.0.13.0. ServUO and ModernUO use `charCount = max(highest slot + 1, limit, 5)`. With third-party features disabled, ServUO overwrites bytes of the slots with a feature mask and an MD5 hash (assistant extension, outside the base protocol). Source-X appends `flags` and `lastSlot` only when the reported version is greater than 1.26.0. Total size for the new format: `11 + 60*charCount + 89*cityCount` (plus 2 if `lastSlot` is sent, depending on the implementation).

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`CharacterList`, `CharacterListOld`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendCharacterList`)
- ServUO: `Server/ExpansionInfo.cs` (`CharacterListFlags`)
- Source-X: `src/network/send.cpp` (`PacketCharacterList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ReceiveCharacterList`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`ParseCharacterList`, `ParseCities`)
