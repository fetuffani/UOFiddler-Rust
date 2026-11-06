# 0x9F - Sell Reply

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Confirma a venda de itens a um vendedor.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9F |
| 1 | 2 | u16 | length | 9 + 6 x count |
| 3 | 4 | u32 | vendor | serial do vendedor |
| 7 | 2 | u16 | count | número de entradas |

**Entrada repetida (6 bytes, a partir do offset 9)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 4 | u32 | serial | serial do item |
| +4 | 2 | u16 | amount | quantidade |

Notas:

O ServUO exige count < 100 e length coerente. Resposta: 0x3B End Vendor Sell e a atualização de inventário. count = 0 fecha a janela.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`VendorSellReply`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingVendorPackets.cs` (`VendorSellReply`)
- Source-X: `src/network/receive.cpp` (`PacketVendorSellReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SellRequest`)
