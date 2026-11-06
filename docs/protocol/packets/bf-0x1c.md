# 0xBF / 0x1C - Selecionar/conjurar feitiço
Direção: cliente -> servidor. Tamanho: 9 ou 13 bytes total.
Finalidade: conjura um feitiço a partir de uma magia escolhida.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 9 (sem livro) ou 13 (com livro) |
| 3 | 2 | u16 | subcommand | 0x001C |
| 5 | 2 | u16 | hasBook | 1 = um serial de spellbook segue; 2 = sem livro (valor enviado pelo ClassicUO) |
| 7 | 4 | u32 | spellbook | Presente somente se `hasBook = 1` |
| 7 ou 11 | 2 | u16 | spellId | Número do feitiço + 1 (o servidor subtrai 1) |

Notas: o Source-X ignora o campo `hasBook` e lê diretamente o `spellId` no offset 7 (não trata `hasBook = 1`). Clientes anteriores à versão 6.0.14.2 usam o pacote 0x12 com texto ASCII, fora do escopo deste documento.

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` e `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` e derivados), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
