# 0x6E - Character Animation

Direction: server -> client. Size: 14 bytes.

Purpose: Plays an animation on a mobile.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6E |
| 1 | 4 | u32 | serial | Mobile. |
| 5 | 2 | u16 | action | Animation action. |
| 7 | 2 | u16 | frameCount | Number of frames. |
| 9 | 2 | u16 | repeatCount | Repetitions. |
| 11 | 1 | bool | reverse | 1 = plays in reverse. |
| 12 | 1 | bool | repeat | 1 = repeats. |
| 13 | 1 | u8 | delay | Delay between frames. |

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `MobileAnimation`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketAction`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `CharacterAnimation`
