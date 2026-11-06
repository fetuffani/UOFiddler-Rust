# 0xE2 - New Character Animation
Direção: servidor -> cliente. Tamanho: 10 bytes.
Finalidade: Executa uma animação em um mobile (clientes 7.0.0.0+).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE2 |
| 1 | 4 | u32 | serial | Serial do mobile |
| 5 | 2 | u16 | type | Tipo da animação |
| 7 | 2 | u16 | action | Ação |
| 9 | 1 | u8 | mode | Modo/atraso entre quadros (ServUO: delay) |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`NewMobileAnimation`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketActionBasic`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`NewCharacterAnimation`)
