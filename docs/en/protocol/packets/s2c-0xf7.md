# 0xF7 - Packet List (multi-packet)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Groups several complete 0xF3 packets into a single send.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF7 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 2 | u16 | count | Number of grouped packets |
| 5 | 26 x count | packet[] | packets | Each entry is a complete 0xF3 packet, including the 0xF3 byte |

Notes: Id outside the project's size table (packets.md covers up to 0xF6). ClassicUO only accepts 0xF3 inside 0xF7 and stops reading on any other id.

## Implementation reference
- ServUO: `Scripts/Multis/Boats/BaseBoat.cs` (`PacketContainer`)
- Source-X: `src/network/send.cpp` (`PacketContainer`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`PacketList`)
