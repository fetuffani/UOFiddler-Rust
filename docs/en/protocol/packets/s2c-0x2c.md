# 0x2C - Resurrection Menu / Death

Direction: server -> client. Size: 2 bytes.

Purpose: Controls the death screen and the resurrection menu.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2C |
| 1 | 1 | u8 | action | Action (table). |

Notes:

| Value | Meaning |
|---|---|
| 0 | Character died (death screen; value sent by the server in ServUO and Source-X) |
| 1 | Resurrection menu (the client does not start the death screen) |
| 2 | Alive (ModernUO sends this value in `SendDeathStatus`; it is also the value the client sends) |

ClassicUO starts the death screen for any value other than 1.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `DeathStatus`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendDeathStatus`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketDeathMenu`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `DeathScreen`
