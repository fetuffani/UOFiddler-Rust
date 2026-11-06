# 0xAE - Unicode Speech
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Entrega uma fala (mensagem) Unicode de um objeto ou do sistema.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAE |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do falante; 0xFFFFFFFF = sistema |
| 7 | 2 | u16 | graphic | Corpo/graphic do falante; 0xFFFF se não é mobile |
| 9 | 1 | u8 | type | Tipo da mensagem (tabela) |
| 10 | 2 | u16 | hue | Cor; ServUO usa 0x03B2 quando 0 |
| 12 | 2 | u16 | font | Fonte |
| 14 | 4 | ascii[4] | language | Código do idioma, ex. "ENU" |
| 18 | 30 | ascii[30] | name | Nome do falante, preenchido com 0x00 |
| 48 | 2*N+2 | utf16be | text | Texto em UTF-16 big-endian terminado por 0x0000 |

Notas: Tipos de mensagem (`MessageType`):

| Valor | Nome |
|---|---|
| 0x00 | Regular |
| 0x01 | System |
| 0x02 | Emote |
| 0x06 | Label |
| 0x07 | Focus |
| 0x08 | Whisper |
| 0x09 | Yell |
| 0x0A | Spell |
| 0x0D | Guild |
| 0x0E | Alliance |
| 0x0F | Command |
| 0xC0 | Encoded |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`UnicodeMessage`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageUNICODE`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UnicodeTalk`)
