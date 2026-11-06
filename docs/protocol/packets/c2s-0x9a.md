# 0x9A - AsciiPromptResponse

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Resposta a um prompt de texto ASCII.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9A |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 4 | u32 | serial | serial do emissor do prompt |
| 7 | 4 | u32 | promptId | id do prompt |
| 11 | 4 | u32 | type | 0 = cancelado; diferente de 0 = confirmado |
| 15 | n | char[] | text | ASCII terminado por NUL |

Notas:

O ServUO descarta texto com mais de 128 caracteres.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`AsciiPromptResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`AsciiPromptResponse`)
- Source-X: `src/network/receive.cpp` (`PacketPromptResponse`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ASCIIPromptResponse`)
