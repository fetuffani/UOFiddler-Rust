# 0x3B - End Vendor Buy

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Fecha a janela de compra/venda de um vendedor.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3B |
| 1 | 2 | u16 | length | 8. |
| 3 | 4 | u32 | vendorSerial | Vendedor. |
| 7 | 1 | u8 | flag | 0. |

Notas:

O ServUO usa o mesmo layout para o fim de compra e de venda. O cliente só lê o serial e fecha o gump do vendedor.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `EndVendorBuy`, `EndVendorSell`
- ModernUO: `Projects/Server/Network/Packets/OutgoingVendorBuyPackets.cs`, `OutgoingVendorSellPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketCloseVendor`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `CloseVendorInterface`
