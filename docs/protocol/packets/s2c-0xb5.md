# 0xB5 - Open Chat Window
Direção: servidor -> cliente. Tamanho: 64 bytes.
Finalidade: Abre a janela do chat antigo.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB5 |
| 1 | 63 | u8[63] | data | Não especificado |

Notas: Tamanho fixo de 64 bytes. O conteúdo não é especificado pelas implementações; o Source-X recebe o mesmo id (tamanho 64) do cliente quando o botão de chat é pressionado.

## Referência de implementação
- Source-X: `src/network/send.cpp` (`PacketChatButton`, recepção)
