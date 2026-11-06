# 0xBF / 0x2A - Race change
Direction: both. Size: S->C 7 bytes; C->S 5 or 15 bytes.
Purpose: opens the race change dialog and returns the chosen appearance.

S->C (7 bytes):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 7 |
| 3 | 2 | u16 | subcommand | 0x002A |
| 5 | 1 | u8 | female | 1 = female |
| 6 | 1 | u8 | race | `RaceID + 1` in ModernUO; 0xFF = close the dialog (in this case `female = 0`) |

C->S (15 bytes; 5 bytes with no payload = cancel):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 5 | 2 | u16 | skinHue | Skin color |
| 7 | 2 | u16 | hairStyle | Hair style |
| 9 | 2 | u16 | hairHue | Hair color |
| 11 | 2 | u16 | beardStyle | Beard style |
| 13 | 2 | u16 | beardHue | Beard color |

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
