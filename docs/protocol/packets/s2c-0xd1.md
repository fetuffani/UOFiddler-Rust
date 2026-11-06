# 0xD1 - Logout Status
Direção: servidor -> cliente. Tamanho: 2 bytes.
Finalidade: Resposta ao pedido de logout.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD1 |
| 1 | 1 | u8 | allow | 1 = o cliente pode sair; 0 = recusado |

Notas: O ClassicUO só desconecta se havia pedido de logout pendente e a flag OverwriteConfigButton está ativa.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`LogoutAck`)
- Source-X: `src/network/send.cpp` (`PacketLogoutAck`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Logout`)
