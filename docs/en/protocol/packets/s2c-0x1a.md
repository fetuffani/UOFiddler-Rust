# 0x1A - Object Info (item on the ground)

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Creates or updates an item in the world; optional fields are flagged by bits in serial, graphic, x and y.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1A |
| 1 | 2 | u16 | length | Total size. |
| 3 | 4 | u32 | serial | Bit 31 (0x80000000) = the amount field is present; the other 31 bits are the serial. |
| 7 | 2 | u16 | graphic | ItemID. Bit 15 (0x8000) = graphicOffset byte present (ClassicUO). Values >= 0x4000 indicate multi (ServUO sends ItemID | 0x4000). |
| opt. | 1 | u8 | graphicOffset | Present if graphic & 0x8000; added to the ItemID. |
| opt. | 2 | u16 | amount | Present if serial bit 31 is set; otherwise amount = 1. |
| - | 2 | u16 | x | Bit 15 = direction/light byte present; coordinate = x & 0x7FFF. |
| - | 2 | u16 | y | Bit 15 = hue present; bit 14 = flags present; coordinate = y & 0x3FFF. |
| opt. | 1 | u8 | direction | Present if x & 0x8000 (direction; ServUO sends the item's light level). |
| - | 1 | i8 | z | Altitude. |
| opt. | 2 | u16 | hue | Present if y & 0x8000. |
| opt. | 1 | u8 | flags | Present if y & 0x4000. |

Notes:

- Field order in the stream: serial, graphic, [graphicOffset], [amount], x, y, [direction], z, [hue], [flags]. The fixed offsets are 0, 1, 3 and 7; the rest depends on the bits.
- Minimum size: 14 bytes (without optional fields). Maximum: 20 bytes.
- `flags` (item): 0x20 = movable, 0x80 = invisible.
- Clients 7.0.0.0 and later receive 0xF3 in place of this packet.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `WorldItem`, `Item.GetPacketFlags`
- ModernUO: `Projects/Server/Network/Packets/OutgoingItemPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemWorld`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateItem`
