# 0x3C - Add Multiple Items To Container (bag contents)

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Sends the entire content of a container (or the items of a vendor's shop).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3C |
| 1 | 2 | u16 | length | Total size (5 + count * entrySize). |
| 3 | 2 | u16 | count | Number of entries. |
| 5+i*size | 19 | entry | items | Repeated `count` times (format below). |

**Entry**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 4 | u32 | serial | Item. |
| +4 | 2 | u16 | graphic | ItemID. |
| +6 | 1 | u8 | graphicOffset | 0. |
| +7 | 2 | u16 | amount | Quantity. |
| +9 | 2 | u16 | x |  |
| +11 | 2 | u16 | y |  |
| +13 | 1 | u8 | gridIndex | Only 20-byte entries (6.0.1.7+). |
| +13/+14 | 4 | u32 | containerSerial | Parent container. |
| +17/+18 | 2 | u16 | hue | Color. |

Notes:

- `entrySize` = 19 bytes; 20 bytes from client 6.0.1.7 onward (with `gridIndex`).
- For vendors, ServUO sends the entries in reverse order with x = index + 1 and y = 1; the client sorts by x/y.
- The first item in the list makes the client clear the container's previous content.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `ContainerContent`, `ContainerContent6017`, `VendorBuyContent`, `VendorBuyContent6017`
- ModernUO: `Projects/Server/Network/Packets/OutgoingContainerPackets.cs`, `OutgoingVendorBuyPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemContents`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateContainedItems`
