# 0x1C - Speech ASCII

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Mensagem de fala/sistema em ASCII exibida sobre uma entidade ou no journal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1C |
| 1 | 2 | u16 | length | Tamanho total. |
| 3 | 4 | u32 | serial | Falante; 0xFFFFFFFF ou 0 para sistema. |
| 7 | 2 | u16 | graphic | Corpo do falante. |
| 9 | 1 | u8 | type | Tipo de mensagem (tabela). |
| 10 | 2 | u16 | hue | Cor; ServUO substitui 0 por 0x03B2. |
| 12 | 2 | u16 | font | Fonte. |
| 14 | 30 | ASCII[30] | name | Nome do falante, preenchido com 0x00. |
| 44 | n | ASCII\0 | text | Texto terminado em 0x00. |

Notas:

Valores de `type`:

| Valor | Significado |
|---|---|
| 0 | Normal |
| 1 | Sistema |
| 2 | Emote |
| 6 | Label |
| 7 | Focus |
| 8 | Sussurro |
| 9 | Grito |
| 10 | Spell |
| 13 | Guild |
| 14 | Alliance |
| 15 | Command |
| 16 | GM chat |

Length = 44 + len(text) + 1.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `AsciiMessage`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketMessageASCII`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Talk`
