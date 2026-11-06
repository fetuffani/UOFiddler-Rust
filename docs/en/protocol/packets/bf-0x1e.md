# 0xBF / 0x1E - Request house design details
Direction: client -> server. Size: fixed 9 bytes total.
Purpose: requests the full content (0xD8) of a house.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 9 |
| 3 | 2 | u16 | subcommand | 0x001E |
| 5 | 4 | u32 | serial | House (multi) |

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
