# 0xBF - General Information: envelope e despacho

Convenções: todos os inteiros são big-endian (rede), exceto onde indicado (`UTF-16LE` nos argumentos do 0xD6 e máscara de feitiços do 0xBF/0x1B). Offsets contam a partir do byte 0 (o byte de id). `S->C` = servidor para cliente, `C->S` = cliente para servidor. Strings `UTF-16BE` terminadas em nulo ocupam 2 bytes por caractere mais 2 bytes de terminador.

- `0xBF` (General Information) tem tamanho variável: `[0xBF][u16 length][u16 subcommand][payload]`. `length` é o tamanho total do pacote, incluindo o id e o próprio campo.
- O despacho é feito pelo `subcommand` (u16). Os servidores indexam uma tabela de 0x00 a 0xFF; subcomandos sem handler são ignorados (com trace). Alguns subcomandos só são aceitos com o jogador já anexado a um personagem.

Cabeçalho comum (todos os subcomandos):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBF |
| 1 | 2 | u16 | length | Tamanho total do pacote |
| 3 | 2 | u16 | subcommand | Ver tabelas abaixo |
| 5 | ... | ... | payload | Depende do subcomando |

Subcomandos documentados (um arquivo por subcomando, `bf-0xNN.md`): 0x01, 0x02, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0e, 0x0f, 0x10, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x20, 0x21, 0x22, 0x24, 0x25, 0x26, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x30, 0x31, 0x32, 0x33.

## Não especificados

Subcomandos citados em tabelas de terceiros mas sem layout nas implementações consultadas: 0x03, 0x0D, 0x11 (o ClassicUO trata como no-op), 0x12, 0x1F, 0x23, 0x27, 0x28, 0x29, 0x2F. O ClassicUO usa internamente o subcomando 0xBEEF para plugins; ele não trafega pela rede.

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`ExtendedCommand`, `RegisterExtended`), `Server/Network/Packets.cs` (`MapPatches`, `StatLockInfo`, `DisplayEquipmentInfo`, `SpeedControl`, `BondedStatus`, `ToggleSpecialAbility`, `DisplayContextMenu`, `DisplayContextMenuOld`, `CloseGump`, `NewSpellbookContent`, `NullFastwalkStack`, `MapChange`, `ClearWeaponAbility`, `DamagePacketOld`).
- ModernUO: `Projects/UOContent/Network/Packets/IncomingExtendedCommandPackets.cs`, `Projects/Server/Network/Packets/Outgoing*Packets.cs`, `Projects/UOContent/Context Menus/ContextMenuSystem.cs`.
- Source-X: `src/network/receive.cpp` (`PacketExtendedCommand` e `PacketScreenSize`, `PacketPartyMessage`, ...), `src/network/send.cpp` (`PacketExtended` e derivados), `src/common/sphereproto.h` (`EXTDATA_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ExtendedCommand`), `src/ClassicUO.Client/Network/OutgoingPackets.cs`.
