# 0xC2 - Unicode Prompt
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Solicita ao cliente uma linha de texto Unicode.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC2 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial de contexto |
| 7 | 4 | u32 | promptId | Id do prompt |
| 11 | 4 | u32 | type | 0 |
| 15 | 4 | ascii[4] | language | Idioma; ServUO envia 0 |
| 19 | 2 | utf16 | text | Texto inicial; vazio = 0x0000 |

Notas: Tamanho total típico: 21 bytes. A resposta é o 0xC2 de cliente para servidor.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`UnicodePrompt`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketAddPrompt`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UnicodePrompt`)
