# 0xAB - Gump Text Entry Dialog
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Abre uma janela de entrada de texto ou número.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAB |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do objeto de contexto |
| 7 | 1 | u8 | parentId | Id do diálogo pai |
| 8 | 1 | u8 | buttonId | Id do botão |
| 9 | 2 | u16 | textLength | Comprimento do texto principal |
| 11 | textLength | ascii | text | Texto principal |
| ... | 1 | u8 | cancel | 1 se existe botão cancelar |
| ... | 1 | u8 | style | 0 = sem edição, 1 = texto, 2 = numérico (valores do Source-X) |
| ... | 4 | u32 | maxLength | Comprimento máximo ou valor máximo |
| ... | 2 | u16 | descLength | Comprimento da descrição |
| ... | descLength | ascii | description | Descrição (Source-X inclui o NUL) |

Notas: O Source-X envia `textLength` incluindo o NUL final e escreve o par `parentId`+`buttonId` como um u16 constante (valor do modo de cursor). A resposta é o 0xAC (cliente para servidor).

## Referência de implementação
- Source-X: `src/network/send.cpp` (`PacketGumpValueInput`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`TextEntryDialog`)
