# 0xEA - Toggle Hotbar
Direção: servidor -> cliente. Tamanho: 3 bytes.
Finalidade: Liga ou desliga a hotbar do cliente KR.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEA |
| 1 | 2 | u16 | enable | 1 = ativar, 0 = desativar |

Notas: Enviado somente a clientes KR.

## Referência de implementação
- Source-X: `src/network/send.cpp` (`PacketToggleHotbar`)
