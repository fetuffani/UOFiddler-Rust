# 0xF7 - Packet List (multi-pacote)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Agrupa vários pacotes 0xF3 completos em um único envio.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF7 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 2 | u16 | count | Quantidade de pacotes agrupados |
| 5 | 26 x count | packet[] | packets | Cada entrada é um pacote 0xF3 completo, incluindo o byte 0xF3 |

Notas: Id fora da tabela de tamanhos do projeto (packets.md cobre até 0xF6). O ClassicUO só aceita 0xF3 dentro do 0xF7 e interrompe a leitura em qualquer outro id.

## Referência de implementação
- ServUO: `Scripts/Multis/Boats/BaseBoat.cs` (`PacketContainer`)
- Source-X: `src/network/send.cpp` (`PacketContainer`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`PacketList`)
