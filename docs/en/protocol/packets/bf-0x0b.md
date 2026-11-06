# 0xBF / 0x0B - Client language
Direction: client -> server. Size: fixed 9 bytes total.
Purpose: reports the client's language code (for example `ENU`).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 9 |
| 3 | 2 | u16 | subcommand | 0x000B |
| 5 | 3 | ASCII | language | 3-letter code |
| 8 | 1 | u8 | terminator | 0x00 |

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
