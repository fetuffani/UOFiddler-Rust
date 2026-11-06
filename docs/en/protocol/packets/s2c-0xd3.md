# 0xD3 - Extended 0x78 (Mobile Incoming)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Extended variant of 0x78 (mobile incoming).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD3 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial |
| 7 | 2 | u16 | body | Body |
| 9 | 2 | u16 | x | X |
| 11 | 2 | u16 | y | Y |
| 13 | 1 | i8 | z | Z |
| 14 | 1 | u8 | direction | Direction |
| 15 | 2 | u16 | hue | Hue |
| 17 | 1 | u8 | flags | Mobile flags |
| 18 | 1 | u8 | notoriety | Notoriety |
| 19 | 6 | u8[6] | extra | 3 x u16 (zeros in ServUO) |
| 25 | variable | item[] | equipment | List of equipped items as in 0x78, terminated by serial 0 |

Notes: ServUO sends only the header above followed by an i32 0 (no equipment); ClassicUO reads, after the 6 extra bytes, the 0x78 list: `u32 serial`, `u16 graphic`, `u8 layer`, `u16 hue` (clients 7.0.33.1+; before that the hue only exists if `graphic & 0x8000`), up to serial 0.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`UnkD3`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateObject`)
