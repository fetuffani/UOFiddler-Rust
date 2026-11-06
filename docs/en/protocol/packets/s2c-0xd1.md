# 0xD1 - Logout Status
Direction: server -> client. Size: 2 bytes.
Purpose: Response to the logout request.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD1 |
| 1 | 1 | u8 | allow | 1 = the client may log out; 0 = refused |

Notes: ClassicUO only disconnects if a logout request was pending and the OverwriteConfigButton flag is active.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`LogoutAck`)
- Source-X: `src/network/send.cpp` (`PacketLogoutAck`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Logout`)
