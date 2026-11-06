# 0xBF / 0x1B - Spellbook content (new)
Direction: server -> client. Size: fixed 23 bytes total.
Purpose: informs which spells a spellbook contains.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 23 |
| 3 | 2 | u16 | subcommand | 0x001B |
| 5 | 2 | u16 | command | 1 |
| 7 | 4 | u32 | serial | Spellbook |
| 11 | 2 | u16 | graphic | Book graphic |
| 13 | 2 | u16 | offset | Number of the first spell represented by bit 0 (ServUO sends `BookOffset + 1`) |
| 15 | 8 | u8[8] | content | 64-bit mask in little-endian byte order: byte `i` holds bits `8*i` to `8*i+7`; bit `n` set = spell `offset + n` present |

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
