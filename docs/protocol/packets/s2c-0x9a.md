# 0x9A - Console Entry Prompt
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Solicita ao cliente uma linha de texto ASCII (prompt de console).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9A |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial de contexto (objeto que pede) |
| 7 | 4 | u32 | promptId | Id do prompt, devolvido na resposta |
| 11 | 4 | u32 | type | 0 |
| 15 | 1+ | ascii | text | Texto, terminado em NUL (normalmente vazio: apenas 0x00) |

Notas: O Source-X envia `serial`, `promptId`, `0` e um NUL. O ClassicUO guarda os dois u32 iniciais como identificação do prompt. A versão Unicode é o 0xC2.

## Referência de implementação
- Source-X: `src/network/send.cpp` (`PacketAddPrompt`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ASCIIPrompt`)
