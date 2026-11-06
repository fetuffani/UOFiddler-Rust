# 0xBB - Message/Account ID
Direction: both. Size: fixed 9 bytes total.
Purpose: Reserved id; the implementations diverge on its use.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBB |
| 1 | 4 | u32 | serial1 | First serial |
| 5 | 4 | u32 | serial2 | Second serial |

Notes: Source-X reads two serials (mail message event, "mail message"); ModernUO accepts the packet as "Account ID" before login; ClassicUO registers an empty handler (Ultima Messenger). The content beyond the 9 bytes of size is not specified.


Implementation reference:
- Source-X: `src/network/receive.cpp` (`PacketMailMessage`)
- ModernUO: `Projects/Server/Network/Packets/IncomingPackets.cs`
- ClassicUO: `PacketHandlers.cs` (`UltimaMessengerR`)
