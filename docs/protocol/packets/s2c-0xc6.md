# 0xC6 - Invalid Map Enable
Direção: servidor -> cliente. Tamanho: 1 byte.
Finalidade: Habilita o mapa inválido; somente o id.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC6 |

Notas: Sem campos adicionais.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`InvalidMapEnable`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`InvalidMapEnable`)
