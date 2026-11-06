# 0xC1 - Localized Message (cliloc)
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Message from an object with text coming from the cliloc, with arguments.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC1 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | serial | Serial of the speaker; 0xFFFFFFFF = system |
| 7 | 2 | u16 | graphic | Graphic of the speaker; 0xFFFF if none |
| 9 | 1 | u8 | type | Message type (same table as 0xAE) |
| 10 | 2 | u16 | hue | Color; ServUO uses 0x03B2 when 0 |
| 12 | 2 | u16 | font | Font |
| 14 | 4 | u32 | cliloc | Cliloc number |
| 18 | 30 | ascii[30] | name | Name of the speaker |
| 48 | 2*N+2 | utf16le | arguments | Arguments terminated by 0x0000 (UTF-16 little-endian); separated by tab ('\t') |

Notes: Attention: the arguments of 0xC1 are UTF-16 little-endian; in 0xCC they are big-endian. Arguments that start with `#` reference another cliloc. ClassicUO reads the rest of the packet as arguments.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`MessageLocalized`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageLocalised`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayClilocString`)
