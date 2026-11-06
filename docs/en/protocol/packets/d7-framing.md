# 0xD7 - Encoded Command: envelope and dispatch

Conventions: integers are big-endian; `enc32` = `u8 0x00` + `u32` (5 bytes).

- `0xD7` (Encoded Command, C->S) has variable size: `[0xD7][u16 length][u32 player serial][u16 subcommand][payload]`. The payload starts at offset 9. The official client ends the packet with a closing byte (`0x0A`, or `0x00` on some buttons); the servers do not read that byte.
- Payload values of 0xD7 are "encoded": 1 type byte followed by the data. Type `0x00` = `u32` integer; type `0x02` = string (u16 length in characters + UTF-16BE); type `0x03` = 3D point (i16 x, i16 y, u8 z). In this document, `enc32` designates the pair `u8 0x00` + `u32` (5 bytes).
- Packets with a fixed `length` in 0xBF/0xD7 do not exist: the `length` field is always present.
- Subcommand identifiers not listed here do not have a specified layout in the implementations consulted (see the "Not specified" section).

Common header:

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD7 |
| 1 | 2 | u16 | length | Total size |
| 3 | 4 | u32 | playerSerial | Serial of the player's character (Source-X discards it if different) |
| 7 | 2 | u16 | subcommand | See the tables below |
| 9 | ... | ... | payload | Encoded values (`enc32` = `u8 0x00` + `u32`) |
| end | 1 | u8 | terminator | `0x0A` (or `0x00` on the quest button); not read by the servers |

There is no S->C counterpart: no server consulted sends 0xD7, and ClassicUO registers the id with an empty handler. ModernUO disconnects the client if the subcommand is greater than 0xFF.

Documented subcommands (`d7-0xNN.md`): 0x02, 0x03, 0x04, 0x05, 0x06, 0x0a, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x12, 0x13, 0x14, 0x19, 0x1a, 0x1e, 0x28, 0x32.

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`EncodedCommand`, `RegisterEncoded`), `Server/Network/EncodedReader.cs`, `Scripts/Multis/HouseFoundation.cs` (`Designer_*`), `Scripts/Misc/EquipLastWeapon.cs`, `Scripts/Services/MondainsLegacyQuests/Helpers/PacketOverrides.cs`.
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`EncodedCommand`), `Projects/UOContent/Multis/Houses/HouseFoundation.cs`, `Projects/UOContent/Items/Weapons/Abilities/WeaponAbilityPackets.cs`.
- Source-X: `src/network/receive.cpp` (`PacketEncodedCommand`, `PacketHouseDesign*`, `PacketSpecialMove`, `PacketEquipLastWeapon`, `PacketGuildButton`, `PacketQuestButton`), `src/common/sphereproto.h` (`EXTAOS_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CustomHouse*`, `Send_UseCombatAbility`, `Send_EquipLastWeapon`, `Send_GuildMenuRequest`, `Send_QuestMenuRequest`).
