# 0x2F - Fight Occurring

Direção: servidor -> cliente. Tamanho: 10 bytes.

Finalidade: Indica um golpe entre atacante e defensor.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2F |
| 1 | 1 | u8 | flag | 0 em ServUO; ignorado pelo cliente. |
| 2 | 4 | u32 | attacker | Atacante. |
| 6 | 4 | u32 | defender | Defensor. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `Swing`
- ModernUO: `Projects/Server/Network/Packets/OutgoingCombatPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketSwing`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Swing`
