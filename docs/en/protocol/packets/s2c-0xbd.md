# 0xBD - Client Version request
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Asks the client to report its version.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBD |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |

Notes: Total size 3 bytes (`BD 00 03`). The client replies with the client-to-server 0xBD (version string).

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`ClientVersionReq`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendClientVersionRequest`)
- Source-X: `src/network/send.cpp` (`PacketClientVersionReq`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ClientVersion`)
