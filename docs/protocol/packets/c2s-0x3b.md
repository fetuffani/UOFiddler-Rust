# 0x3B - VendorBuyReply

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Confirma a compra de itens de um vendedor.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3B |
| 1 | 2 | u16 | length | 8 + 7 x n |
| 3 | 4 | u32 | vendor | serial do vendedor |
| 7 | 1 | u8 | flag | 0 = nada comprado/cancelar; 2 = lista de compra presente |

**Entrada repetida (7 bytes, a partir do offset 8, apenas se flag = 2)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 1 | u8 | layer | camada do item no vendedor (o cliente de referência envia 0x1A) |
| +1 | 4 | u32 | serial | serial do item |
| +5 | 2 | u16 | amount | quantidade |

Notas:

n = (length - 8) / 7. O ServUO rejeita mais de 100 entradas. Resposta: 0x3B End Vendor Buy (8 bytes, servidor para cliente) e as mudanças de inventário.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`VendorBuyReply`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingVendorPackets.cs` (`VendorBuyReply`)
- Source-X: `src/network/receive.cpp` (`PacketVendorBuyReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_BuyRequest`)
