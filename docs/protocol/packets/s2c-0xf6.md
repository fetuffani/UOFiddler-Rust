# 0xF6 - Boat Moving
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Move um barco e os objetos a bordo em um único pacote.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF6 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do barco |
| 7 | 1 | u8 | speed | Velocidade |
| 8 | 1 | u8 | moveDir | Direção do movimento |
| 9 | 1 | u8 | facing | Direção do barco |
| 10 | 2 | u16 | x | X do barco |
| 12 | 2 | u16 | y | Y do barco |
| 14 | 2 | u16 | z | Z do barco |
| 16 | 2 | u16 | count | Quantidade de objetos a bordo |
| 18 | 10 x count | entry[] | objects | Entrada: u32 serial, u16 x, u16 y, u16 z |

Notas: Tamanho total: `18 + 10*count`. O barco em si não aparece na lista. O ClassicUO usa os três bytes de direção/velocidade para o movimento suave.

## Referência de implementação
- ServUO: `Scripts/Multis/Boats/BaseBoat.cs` (`MoveBoatHS`)
- ModernUO: `Projects/UOContent/Multis/Boats/BoatPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMoveShip`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BoatMoving`)
- Documento do projeto: `docs/protocol/boats-smooth-sailing.md`
