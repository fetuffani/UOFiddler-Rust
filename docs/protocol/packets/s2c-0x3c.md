# 0x3C - Add Multiple Items To Container (conteúdo da bag)

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Envia todo o conteúdo de um container (ou os itens de uma loja de vendedor).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3C |
| 1 | 2 | u16 | length | Tamanho total (5 + count * entrySize). |
| 3 | 2 | u16 | count | Número de entradas. |
| 5+i*size | 19 | entrada | items | Repetido `count` vezes (formato abaixo). |

**Entrada**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 4 | u32 | serial | Item. |
| +4 | 2 | u16 | graphic | ItemID. |
| +6 | 1 | u8 | graphicOffset | 0. |
| +7 | 2 | u16 | amount | Quantidade. |
| +9 | 2 | u16 | x |  |
| +11 | 2 | u16 | y |  |
| +13 | 1 | u8 | gridIndex | Somente 20 bytes (6.0.1.7+). |
| +13/+14 | 4 | u32 | containerSerial | Container pai. |
| +17/+18 | 2 | u16 | hue | Cor. |

Notas:

- `entrySize` = 19 bytes; 20 bytes a partir do cliente 6.0.1.7 (com `gridIndex`).
- Para vendedores, o ServUO envia as entradas em ordem inversa com x = índice + 1 e y = 1; o cliente ordena por x/y.
- O primeiro item da lista faz o cliente limpar o conteúdo anterior do container.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `ContainerContent`, `ContainerContent6017`, `VendorBuyContent`, `VendorBuyContent6017`
- ModernUO: `Projects/Server/Network/Packets/OutgoingContainerPackets.cs`, `OutgoingVendorBuyPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemContents`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateContainedItems`
