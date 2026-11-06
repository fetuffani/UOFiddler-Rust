# 0x3B - End Vendor Buy

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Closes the buy/sell window of a vendor.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3B |
| 1 | 2 | u16 | length | 8. |
| 3 | 4 | u32 | vendorSerial | Vendor. |
| 7 | 1 | u8 | flag | 0. |

Notes:

ServUO uses the same layout for the end of buying and the end of selling. The client only reads the serial and closes the vendor's gump.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `EndVendorBuy`, `EndVendorSell`
- ModernUO: `Projects/Server/Network/Packets/OutgoingVendorBuyPackets.cs`, `OutgoingVendorSellPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketCloseVendor`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `CloseVendorInterface`
