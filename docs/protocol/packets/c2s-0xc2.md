# 0xC2 - UnicodePromptResponse

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Resposta a um prompt de texto Unicode.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC2 |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 4 | u32 | serial | serial do emissor do prompt |
| 7 | 4 | u32 | promptId | id do prompt |
| 11 | 4 | u32 | type | 0 = cancelado; diferente de 0 = confirmado |
| 15 | 4 | char[4] | language | idioma ASCII, por exemplo "ENU" + NUL |
| 19 | n | char16[] | text | UTF-16 little-endian terminado por NUL |

Notas:

Diferente dos demais textos Unicode do protocolo, este usa little-endian. O ServUO descarta texto com mais de 128 caracteres.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnicodePromptResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`UnicodePromptResponse`)
- Source-X: `src/network/receive.cpp` (`PacketPromptResponseUnicode`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnicodePromptResponse`)
