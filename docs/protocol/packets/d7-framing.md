# 0xD7 - Encoded Command: envelope e despacho

Convenções: inteiros big-endian; `enc32` = `u8 0x00` + `u32` (5 bytes).

- `0xD7` (Encoded Command, C->S) tem tamanho variável: `[0xD7][u16 length][u32 serial do jogador][u16 subcommand][payload]`. O payload começa no offset 9. O cliente oficial termina o pacote com um byte de fechamento (`0x0A`, ou `0x00` em alguns botões); os servidores não leem esse byte.
- Valores do payload do 0xD7 são "codificados": 1 byte de tipo seguido do dado. Tipo `0x00` = inteiro `u32`; tipo `0x02` = string (u16 comprimento em caracteres + UTF-16BE); tipo `0x03` = ponto 3D (i16 x, i16 y, u8 z). Neste documento, `enc32` designa o par `u8 0x00` + `u32` (5 bytes).
- Pacotes com `length` fixo no 0xBF/0xD7 não existem: o campo `length` sempre está presente.
- Identificadores de subcomando não listados aqui não têm layout especificado nas implementações consultadas (ver seção "Não especificados").

Cabeçalho comum:

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD7 |
| 1 | 2 | u16 | length | Tamanho total |
| 3 | 4 | u32 | playerSerial | Serial do personagem do jogador (o Source-X descarta se for diferente) |
| 7 | 2 | u16 | subcommand | Ver tabelas abaixo |
| 9 | ... | ... | payload | Valores codificados (`enc32` = `u8 0x00` + `u32`) |
| fim | 1 | u8 | terminator | `0x0A` (ou `0x00` no botão de quest); não lido pelos servidores |

Não há contraparte S->C: nenhum servidor consultado envia 0xD7 e o ClassicUO registra o id com um handler vazio. O ModernUO desconecta o cliente se o subcomando for maior que 0xFF.

Subcomandos documentados (`d7-0xNN.md`): 0x02, 0x03, 0x04, 0x05, 0x06, 0x0a, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x12, 0x13, 0x14, 0x19, 0x1a, 0x1e, 0x28, 0x32.

Referência de implementação:
- ServUO: `Server/Network/PacketHandlers.cs` (`EncodedCommand`, `RegisterEncoded`), `Server/Network/EncodedReader.cs`, `Scripts/Multis/HouseFoundation.cs` (`Designer_*`), `Scripts/Misc/EquipLastWeapon.cs`, `Scripts/Services/MondainsLegacyQuests/Helpers/PacketOverrides.cs`.
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`EncodedCommand`), `Projects/UOContent/Multis/Houses/HouseFoundation.cs`, `Projects/UOContent/Items/Weapons/Abilities/WeaponAbilityPackets.cs`.
- Source-X: `src/network/receive.cpp` (`PacketEncodedCommand`, `PacketHouseDesign*`, `PacketSpecialMove`, `PacketEquipLastWeapon`, `PacketGuildButton`, `PacketQuestButton`), `src/common/sphereproto.h` (`EXTAOS_TYPE`).
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CustomHouse*`, `Send_UseCombatAbility`, `Send_EquipLastWeapon`, `Send_GuildMenuRequest`, `Send_QuestMenuRequest`).
