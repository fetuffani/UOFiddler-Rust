# 0xFA - Unhandled (UO Store)

Direction: client -> server. Size: 1 byte (fixed). Available: in game.

Purpose: Opens the UO Store (store button).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xFA |

Notes:

No payload. ServUO leaves the handling to the UO Store module; the packet table does not fix a size for this id.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (registration comment for 0xFA)
- Source-X: `src/network/receive.cpp` (`PacketUltimaStoreButton`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_OpenUOStore`)
