# 0xB3 - Chat Text
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Texto do sistema de chat antigo.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB3 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |

Notas: Não especificado: nenhuma implementação de referência envia este pacote ao cliente. Na direção cliente para servidor o Source-X lê `u16 length`, `ascii[4] language` e texto UTF-16 big-endian.

## Referência de implementação
- Source-X: `src/network/send.cpp` (`PacketChatCommand`, recepção)
