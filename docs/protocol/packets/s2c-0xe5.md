# 0xE5 - Display Waypoint
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Adiciona um waypoint no mapa de radar (clientes KR/SA).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE5 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do objeto |
| 7 | 2 | u16 | x | X |
| 9 | 2 | u16 | y | Y |
| 11 | 1 | i8 | z | Z |
| 12 | 1 | u8 | map | Id do mapa |
| 13 | 2 | u16 | type | Tipo (tabela) |
| 15 | 2 | u16 | ignoreObject | 1 = ignora o objeto |
| 17 | 4 | u32 | cliloc | 1046414 (cadáver) ou 1062613 no ServUO; Source-X: 1028198 ou 1062613 |
| 21 | 2*N+2 | utf16le | name | Nome terminado por 0x0000 (o ServUO acrescenta mais um u16 0) |

Notas: Tipos: 0x01 Corpse, 0x02 PartyMember, 0x03 RallyPoint, 0x04 QuestGiver, 0x05 QuestDestination, 0x06 Resurrection, 0x07 PointOfInterest, 0x08 Landmark, 0x09 Town, 0x0A Dungeon, 0x0B Moongate, 0x0C Shop, 0x0D Player.

## Referência de implementação
- ServUO: `Scripts/Misc/Waypoints.cs` (`DisplayWaypoint`)
- Source-X: `src/network/send.cpp` (`PacketWaypointAdd`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayWaypoint`)
