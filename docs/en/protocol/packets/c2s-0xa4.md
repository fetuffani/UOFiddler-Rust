# 0xA4 - System Info

Direction: client -> server. Size: 149 bytes (fixed). Available: before login and in game.

Purpose: Hardware information (legacy format).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA4 |
| 1 | 1 | u8 | v1 | Not specified |
| 2 | 2 | u16 | v2 | Not specified |
| 4 | 1 | u8 | v3 | Not specified |
| 5 | 32 | char[32] | s1 | ASCII |
| 37 | 32 | char[32] | s2 | ASCII |
| 69 | 32 | char[32] | s3 | ASCII |
| 101 | 32 | char[32] | s4 | ASCII |
| 133 | 2 | u16 | v4 | Not specified |
| 135 | 2 | u16 | v5 | Not specified |
| 137 | 4 | i32 | v6 | Not specified |
| 141 | 4 | i32 | v7 | Not specified |
| 145 | 4 | i32 | v8 | Not specified |

Notes:

The meaning of the fields is not specified; servers discard the packet.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`SystemInfo`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`SystemInfo`)
- Source-X: `src/network/receive.cpp` (`PacketSystemInfo`)
