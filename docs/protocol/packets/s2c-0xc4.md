# 0xC4 - Semivisible
Direção: servidor -> cliente. Tamanho: 6 bytes.
Finalidade: Id reservado.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC4 |
| 1 | 5 | u8[5] | data | Não especificado |

Notas: O ClassicUO tem handler vazio; nenhuma implementação envia o pacote.

## Referência de implementação
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Semivisible`)
