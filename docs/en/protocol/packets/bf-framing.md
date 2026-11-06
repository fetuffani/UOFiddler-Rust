# 0xBF - General Information: envelope and dispatch

Conventions: all integers are big-endian (network order), except where indicated (`UTF-16LE` in the arguments of 0xD6 and the spell mask of 0xBF/0x1B). Offsets count from byte 0 (the id byte). `S->C` = server to client, `C->S` = client to server. Null-terminated `UTF-16BE` strings occupy 2 bytes per character plus 2 bytes for the terminator.

- `0xBF` (General Information) has variable size: `[0xBF][u16 length][u16 subcommand][payload]`. `length` is the total size of the packet, including the id and the field itself.
- Dispatch is done by the `subcommand` (u16). Servers index a table from 0x00 to 0xFF; subcommands without a handler are ignored (with trace). Some subcommands are only accepted once the player is already attached to a character.

Common header (all subcommands):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | Total size of the packet |
| 3 | 2 | u16 | subcommand | See tables below |
| 5 | ... | ... | payload | Depends on the subcommand |

Documented subcommands (one file per subcommand, `bf-0xNN.md`): 0x01, 0x02, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0e, 0x0f, 0x10, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x20, 0x21, 0x22, 0x24, 0x25, 0x26, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x30, 0x31, 0x32, 0x33.

## Unspecified

Subcommands mentioned in third-party tables but without a layout in the implementations consulted: 0x03, 0x0D, 0x11 (ClassicUO treats it as a no-op), 0x12, 0x1F, 0x23, 0x27, 0x28, 0x29, 0x2F. ClassicUO uses subcommand 0xBEEF internally for plugins; it does not travel over the network.

Implementation reference:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` and `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` and derivatives), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
