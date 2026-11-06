# 0x9F - Sell Reply

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Confirms the sale of items to a vendor.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9F |
| 1 | 2 | u16 | length | 9 + 6 x count |
| 3 | 4 | u32 | vendor | serial of the vendor |
| 7 | 2 | u16 | count | number of entries |

**Repeated entry (6 bytes, starting at offset 9)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 4 | u32 | serial | serial of the item |
| +4 | 2 | u16 | amount | quantity |

Notes:

ServUO requires count < 100 and a consistent length. Response: 0x3B End Vendor Sell and the inventory update. count = 0 closes the window.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`VendorSellReply`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingVendorPackets.cs` (`VendorSellReply`)
- Source-X: `src/network/receive.cpp` (`PacketVendorSellReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SellRequest`)
