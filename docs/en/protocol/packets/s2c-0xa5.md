# 0xA5 - Open Web Browser
Direction: server -> client. Size: variable (id + u16 length + payload; length is the total size of the packet).
Purpose: Opens a URL in the client's web browser.

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA5 |
| 1 | 2 | u16 | length | Total size of the packet, including the id and this field |
| 3 | 1+ | ascii | url | URL terminated by NUL |

## Implementation reference
- ServUO: `Server/Network/Packets.cs` (`LaunchBrowser`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketWebPage`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenUrl`)
