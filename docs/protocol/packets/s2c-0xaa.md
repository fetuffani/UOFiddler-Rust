# 0xAA - Attack Target (current)
Direção: servidor -> cliente. Tamanho: 5 bytes.
Finalidade: Define o alvo de ataque atual do jogador.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAA |
| 1 | 4 | u32 | serial | Serial do alvo; 0 = pedido de ataque recusado |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ChangeCombatant`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingCombatPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketAttack`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`AttackCharacter`)
