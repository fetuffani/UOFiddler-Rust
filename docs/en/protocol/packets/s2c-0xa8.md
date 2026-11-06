# 0xA8 - Game Server List
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Lists the available game servers after the account login.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA8 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1 | u8 | flags | ServUO/ModernUO: 0x5D; Source-X: 0xFF. The client does not use the value for the layout |
| 4 | 2 | u16 | count | Number of servers |
| 6 | 40 x count | entry[] | servers | See the entry table |

Notes: Entry (40 bytes):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 2 | u16 | index | Server index (ServUO starts at 0; Source-X at 1); returned in 0xA0 |
| 2 | 32 | ascii[32] | name | Name, padded with 0x00 |
| 34 | 1 | u8 | percentFull | Load as a percentage |
| 35 | 1 | i8 | timezone | Time zone in hours |
| 36 | 4 | u8[4] | ip | IPv4 address |

IP order: clients from 4.0.0 onward receive the octets in natural order (first byte = first octet), as Source-X sends and ClassicUO interprets; clients before 4.0.0 expect the reversed order. ServUO writes the address as a big-endian integer with a little-endian value (reversed order). The client uses the address only for display/ping, since the actual connection comes from 0x8C. Total size: `6 + 40*count`.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`AccountLoginAck`, `ServerInfo`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendAccountLoginAck`)
- Source-X: `src/network/send.cpp` (`PacketServerList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ServerListReceived`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`ServerListEntry.Create`)
