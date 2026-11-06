# 0x9E - Sell List
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Abre a janela de venda do vendedor com os itens que o jogador pode vender.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9E |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | vendor | Serial do vendedor |
| 7 | 2 | u16 | count | Quantidade de entradas |
| 9 | variável | entry[] | items | `count` entradas |

Notas: Entrada:

| Tamanho | Tipo | Campo |
|---|---|---|
| 4 | u32 | serial do item |
| 2 | u16 | itemId |
| 2 | u16 | hue |
| 2 | u16 | amount |
| 2 | u16 | price |
| 2 | u16 | nameLength |
| nameLength | ascii | name (sem NUL) |

Se `name` for numérico o ClassicUO o trata como número de cliloc; se vazio usa o nome do tiledata.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`VendorSellList`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingVendorSellPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketVendorSellList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`SellList`)
