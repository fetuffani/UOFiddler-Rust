# 0xBF / 0x2B - Animação por serial parcial
Direção: servidor -> cliente. Tamanho: fixo 9 bytes total.
Finalidade: ajusta animação/frame de um mobile identificado pelos 16 bits baixos do serial (tratado só pelo ClassicUO).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 9 |
| 3 | 2 | u16 | subcommand | 0x002B |
| 5 | 2 | u16 | serialLow | 16 bits baixos do serial |
| 7 | 1 | u8 | animation | Id da animação |
| 8 | 1 | u8 | frame | Quantidade/índice de frame |

Notas: este subcomando aparece somente no ClassicUO; ServUO, ModernUO e Source-X não o enviam.

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` e `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` e derivados), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
