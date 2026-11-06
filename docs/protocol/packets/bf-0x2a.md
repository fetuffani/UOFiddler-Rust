# 0xBF / 0x2A - Troca de raça
Direção: ambas. Tamanho: S->C 7 bytes; C->S 5 ou 15 bytes.
Finalidade: abre o diálogo de troca de raça e devolve a aparência escolhida.

S->C (7 bytes):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | 7 |
| 3 | 2 | u16 | subcommand | 0x002A |
| 5 | 1 | u8 | female | 1 = feminino |
| 6 | 1 | u8 | race | `RaceID + 1` no ModernUO; 0xFF = fechar o diálogo (neste caso `female = 0`) |

C->S (15 bytes; 5 bytes sem payload = cancelamento):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 5 | 2 | u16 | skinHue | Cor da pele |
| 7 | 2 | u16 | hairStyle | Estilo do cabelo |
| 9 | 2 | u16 | hairHue | Cor do cabelo |
| 11 | 2 | u16 | beardStyle | Estilo da barba |
| 13 | 2 | u16 | beardHue | Cor da barba |

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` e `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` e derivados), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
