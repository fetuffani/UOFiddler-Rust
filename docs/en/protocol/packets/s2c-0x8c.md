# 0x8C - Connect To Game Server
Direction: server -> client. Size: 11 bytes.
Purpose: Tells the client the address of the game server and the authentication key; the client reconnects and uses the key as seed.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8C |
| 1 | 4 | u8[4] | ip | IPv4 address in natural order: first byte = first octet |
| 5 | 2 | u16 | port | Game server port |
| 7 | 4 | u32 | authId | Authentication key (seed for the new connection) |

Notes: ClassicUO reads the IP as u32 little-endian (equivalent to the natural octet order), the port as u16 big-endian and the key as u32 big-endian; when reconnecting it sends the key as the first 4 bytes of the new connection and then the login (0x91). The client starts using game encryption/compression on the new connection.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`PlayServerAck`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendPlayServerAck`)
- Source-X: `src/network/send.cpp` (`PacketServerRelay`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ReceiveServerRelay`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`HandleRelayServerPacket`)
