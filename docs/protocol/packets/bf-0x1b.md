# 0xBF / 0x1B - Conteúdo do spellbook (novo)
Direção: servidor -> cliente. Tamanho: fixo 23 bytes total.
Finalidade: informa quais feitiços um spellbook contém.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 23 |
| 3 | 2 | u16 | subcommand | 0x001B |
| 5 | 2 | u16 | command | 1 |
| 7 | 4 | u32 | serial | Spellbook |
| 11 | 2 | u16 | graphic | Gráfico do livro |
| 13 | 2 | u16 | offset | Número do primeiro feitiço representado pelo bit 0 (ServUO envia `BookOffset + 1`) |
| 15 | 8 | u8[8] | content | Máscara de 64 bits em ordem little-endian de bytes: o byte `i` contém os bits `8*i` a `8*i+7`; bit `n` ligado = feitiço `offset + n` presente |

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` e `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` e derivados), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
