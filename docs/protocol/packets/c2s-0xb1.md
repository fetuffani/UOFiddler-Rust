# 0xB1 - Gump Menu Response

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Resposta a um gump (botão, interruptores e campos de texto).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB1 |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 4 | u32 | serial | serial do gump (objeto que o abriu) |
| 7 | 4 | u32 | gumpId | id do tipo do gump |
| 11 | 4 | u32 | buttonId | botão pressionado; 0 = fechar |
| 15 | 4 | u32 | switchCount | número de interruptores marcados |
| 19 | 4 x s | u32[] | switches | ids dos interruptores marcados |
| 19 + 4s | 4 | u32 | textCount | número de entradas de texto |

**Entrada de texto (repetida textCount vezes)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 2 | u16 | entryId | id do campo de texto |
| +2 | 2 | u16 | textLength | número de caracteres (não bytes); máximo 239 |
| +4 | 2 x textLength | char16[] | text | UTF-16 big-endian, sem NUL final |

Notas:

s = switchCount. Cada entrada de texto ocupa 4 + 2 x textLength bytes.

Gump de virtudes (gumpId 0x000001CD = 461): o cliente de referência envia apenas serial, gumpId e buttonId (length = 15); o ServUO ainda aceita, com buttonId = 1 e switchCount > 0, um serial adicional de mobile após switchCount.
Se o botão não existir no gump, o ServUO desconecta o cliente.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`DisplayGumpResponse`)
- Source-X: `src/network/receive.cpp` (`PacketGumpDialogRet`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_GumpResponse`, `Send_VirtueGumpResponse`)
