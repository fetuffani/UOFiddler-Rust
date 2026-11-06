# 0x2E - Equip/Worn Item

Direction: server -> client. Size: 15 bytes.

Purpose: Equips an item on a mobile.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2E |
| 1 | 4 | u32 | serial | Item. |
| 5 | 2 | u16 | graphic | ItemID. |
| 7 | 1 | i8 | graphicOffset | 0; added to graphic. |
| 8 | 1 | u8 | layer | Layer. |
| 9 | 4 | u32 | mobileSerial | Owner. |
| 13 | 2 | u16 | hue | Color. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `EquipUpdate`
- ModernUO: `Projects/Server/Network/Packets/OutgoingEquipmentPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemEquipped`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `EquipItem`
