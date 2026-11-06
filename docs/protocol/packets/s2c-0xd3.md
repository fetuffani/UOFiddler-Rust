# 0xD3 - Extended 0x78 (Mobile Incoming)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Variante estendida do 0x78 (mobile incoming).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD3 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial |
| 7 | 2 | u16 | body | Body |
| 9 | 2 | u16 | x | X |
| 11 | 2 | u16 | y | Y |
| 13 | 1 | i8 | z | Z |
| 14 | 1 | u8 | direction | Direção |
| 15 | 2 | u16 | hue | Hue |
| 17 | 1 | u8 | flags | Flags do mobile |
| 18 | 1 | u8 | notoriety | Notoriedade |
| 19 | 6 | u8[6] | extra | 3 x u16 (zeros no ServUO) |
| 25 | variável | item[] | equipment | Lista de itens equipados como no 0x78, terminada por serial 0 |

Notas: O ServUO envia apenas o cabeçalho acima seguido de i32 0 (sem equipamento); o ClassicUO lê, após os 6 bytes extras, a lista do 0x78: `u32 serial`, `u16 graphic`, `u8 layer`, `u16 hue` (clientes 7.0.33.1+; antes o hue só existe se `graphic & 0x8000`), até o serial 0.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`UnkD3`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateObject`)
