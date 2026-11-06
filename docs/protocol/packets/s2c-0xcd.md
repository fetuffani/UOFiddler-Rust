# 0xCD - Não documentado (1 byte)
Direção: servidor -> cliente. Tamanho: 1 byte.
Finalidade: Id sem layout conhecido (o cliente 7.0.10.3 descarta).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCD |

Notas: Não especificado: não há campos além do id.

## Referência de implementação
- Nenhuma implementação de referência.
