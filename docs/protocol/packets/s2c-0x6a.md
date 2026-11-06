# 0x6A - Não documentado

Direção: servidor -> cliente. Tamanho: 3 bytes.

Finalidade: Não documentado.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6A |
| 1 | 2 | u8[2] | payload | Não documentado. |

Notas:

Apenas o tamanho (3) é conhecido, pela tabela do ClassicUO.

## Referência de implementação

- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PacketsTable.cs`
