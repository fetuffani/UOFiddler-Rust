# 0xAF - Display Death Action
Direção: servidor -> cliente. Tamanho: 13 bytes.
Finalidade: Mostra a animação de morte de um mobile e vincula o cadáver.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAF |
| 1 | 4 | u32 | serial | Serial do mobile morto |
| 5 | 4 | u32 | corpse | Serial do cadáver (0 se nenhum) |
| 9 | 4 | u32 | flag | ServUO: 0. Source-X: 1 se caiu de frente. O ClassicUO lê o campo e não o usa |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`DeathAnimation`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketDeath`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayDeath`)
