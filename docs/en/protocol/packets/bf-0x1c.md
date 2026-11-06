# 0xBF / 0x1C - Select/cast spell
Direction: client -> server. Size: 9 or 13 bytes total.
Purpose: casts a spell chosen from a spellbook.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 9 (no book) or 13 (with book) |
| 3 | 2 | u16 | subcommand | 0x001C |
| 5 | 2 | u16 | hasBook | 1 = a spellbook serial follows; 2 = no book (value sent by ClassicUO) |
| 7 | 4 | u32 | spellbook | Present only if `hasBook = 1` |
| 7 or 11 | 2 | u16 | spellId | Spell number + 1 (the server subtracts 1) |

Notes: Source-X ignores the `hasBook` field and reads `spellId` directly at offset 7 (it does not handle `hasBook = 1`). Clients prior to version 6.0.14.2 use packet 0x12 with ASCII text, which is outside the scope of this document.

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
