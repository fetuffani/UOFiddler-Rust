# 0xCB - Global Queue Count
Direção: servidor -> cliente. Tamanho: 7 bytes.
Finalidade: Informa a quantidade de chamadas disponíveis na fila global.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCB |
| 1 | 2 | i16 | unknown | Valor auxiliar |
| 3 | 4 | i32 | count | Quantidade de chamadas |

Notas: O cliente exibe "There are currently [count] available calls in the global queue.".

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`GQCount`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`GlobalQueCount`)
