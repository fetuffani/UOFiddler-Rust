# 0x1D - Delete Object

Direção: servidor -> cliente. Tamanho: 5 bytes.

Finalidade: Remove um item ou mobile da visão do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1D |
| 1 | 4 | u32 | serial | Entidade removida. |

Notas:

O cliente ignora a remoção do próprio jogador.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `RemoveItem`, `RemoveMobile`
- ModernUO: `Projects/Server/Network/Packets/OutgoingEntityPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketRemoveObject`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `DeleteObject`
