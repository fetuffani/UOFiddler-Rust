# 0x2E - Equip/Worn Item

Direção: servidor -> cliente. Tamanho: 15 bytes.

Finalidade: Equipa um item em um mobile.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2E |
| 1 | 4 | u32 | serial | Item. |
| 5 | 2 | u16 | graphic | ItemID. |
| 7 | 1 | i8 | graphicOffset | 0; somado ao graphic. |
| 8 | 1 | u8 | layer | Camada. |
| 9 | 4 | u32 | mobileSerial | Dono. |
| 13 | 2 | u16 | hue | Cor. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `EquipUpdate`
- ModernUO: `Projects/Server/Network/Packets/OutgoingEquipmentPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemEquipped`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `EquipItem`
