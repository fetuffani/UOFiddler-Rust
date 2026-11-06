# 0x1D - Delete Object

Direction: server -> client. Size: 5 bytes.

Purpose: Removes an item or mobile from the client's view.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1D |
| 1 | 4 | u32 | serial | Removed entity. |

Notes:

The client ignores the removal of the player's own character.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `RemoveItem`, `RemoveMobile`
- ModernUO: `Projects/Server/Network/Packets/OutgoingEntityPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketRemoveObject`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `DeleteObject`
