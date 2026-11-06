# 0xBE - Assist Version
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Requests the version of the client's Assist.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBE |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 4 | u32 | version | Assist version of the server |

Notes: Total size 7 bytes in ServUO. ClassicUO has an empty handler.

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`AssistVersionReq`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`AssistVersion`)
