# 0x3B - VendorBuyReply

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Confirms the purchase of items from a vendor.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3B |
| 1 | 2 | u16 | length | 8 + 7 x n |
| 3 | 4 | u32 | vendor | vendor serial |
| 7 | 1 | u8 | flag | 0 = nothing bought / cancel; 2 = buy list present |

**Repeated entry (7 bytes, starting at offset 8, only if flag = 2)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 1 | u8 | layer | layer of the item in the vendor (the reference client sends 0x1A) |
| +1 | 4 | u32 | serial | item serial |
| +5 | 2 | u16 | amount | quantity |

Notes:

n = (length - 8) / 7. ServUO rejects more than 100 entries. Response: 0x3B End Vendor Buy (8 bytes, server to client) and the inventory changes.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`VendorBuyReply`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingVendorPackets.cs` (`VendorBuyReply`)
- Source-X: `src/network/receive.cpp` (`PacketVendorBuyReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_BuyRequest`)
