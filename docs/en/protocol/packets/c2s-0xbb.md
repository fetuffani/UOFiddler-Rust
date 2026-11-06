# 0xBB - AccountID

Direction: client -> server. Size: 9 bytes (fixed). Available: before login.

Purpose: Ultima Messenger message (two serials).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBB |
| 1 | 4 | u32 | serial1 | Not specified |
| 5 | 4 | u32 | serial2 | Not specified |

Notes:

ServUO ignores the packet; Source-X handles it as a mail message between two serials.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`AccountID`)
- Source-X: `src/network/receive.cpp` (`PacketMailMessage`)
