# 0xE9 - Highlight UI Toggle
Direção: servidor -> cliente. Tamanho: 75 bytes.
Finalidade: Alterna o destaque da interface (tutorial KR).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE9 |
| 1 | 74 | u8[74] | data | Não especificado |

Notas: O Source-X só define o id (`XCMD_HighlightUIToggle`).

## Referência de implementação
- Source-X: `src/network/send.cpp` (`src/common/sphereproto.h`)
