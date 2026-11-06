# 0x6D - Play Music (MIDI)

Direction: server -> client. Size: 3 bytes.

Purpose: Plays a music track.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6D |
| 1 | 2 | u16 | musicId | Music index. |

Notes:

ModernUO sends the music with the bytes `1F FF`; ClassicUO interprets those two bytes as the stop command.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `PlayMusic`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendPlayMusic`, `SendStopMusic`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketPlayMusic`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PlayMusic`
