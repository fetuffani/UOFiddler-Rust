# 0xBE - AssistVersion

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Informa a versão de um assistente de terceiros.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBE |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 4 | u32 | unknown | Não especificado |
| 7 | n | char[] | version | ASCII terminado por NUL |

Notas:

Os servidores de referência descartam o conteúdo.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`AssistVersion`)
