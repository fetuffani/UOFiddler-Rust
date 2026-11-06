# 0xAC - Gump Text Entry Dialog Reply

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Resposta à caixa de entrada de texto (0xAB do servidor).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAC |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 4 | u32 | serial | serial do diálogo/objeto |
| 7 | 1 | u8 | parentId | id do diálogo pai |
| 8 | 1 | u8 | buttonId | botão pressionado |
| 9 | 1 | u8 | ok | 1 = OK; 0 = cancelado |
| 10 | 2 | u16 | textLength | tamanho do texto (o cliente de referência inclui o NUL) |
| 12 | n | char[] | text | ASCII |

Notas:

O ServUO não registra este id; o Source-X lê parentId e buttonId juntos como um contexto u16.

Referência de implementação:

- Source-X: `src/network/receive.cpp` (`PacketGumpValueInputResponse`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TextEntryDialogResponse`)
