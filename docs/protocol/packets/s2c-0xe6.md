# 0xE6 - Remove Waypoint
Direção: servidor -> cliente. Tamanho: 5 bytes.
Finalidade: Remove um waypoint do mapa de radar.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE6 |
| 1 | 4 | u32 | serial | Serial do waypoint |

## Referência de implementação
- ServUO: `Scripts/Misc/Waypoints.cs` (`RemoveWaypoint`)
- Source-X: `src/network/send.cpp` (`PacketWaypointRemove`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`RemoveWaypoint`)
