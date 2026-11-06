# 0x1C - Speech ASCII

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Speech/system message in ASCII displayed over an entity or in the journal.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1C |
| 1 | 2 | u16 | length | Total size. |
| 3 | 4 | u32 | serial | Speaker; 0xFFFFFFFF or 0 for system. |
| 7 | 2 | u16 | graphic | Speaker's body. |
| 9 | 1 | u8 | type | Message type (table). |
| 10 | 2 | u16 | hue | Color; ServUO replaces 0 with 0x03B2. |
| 12 | 2 | u16 | font | Font. |
| 14 | 30 | ASCII[30] | name | Speaker name, padded with 0x00. |
| 44 | n | ASCII\0 | text | Text terminated with 0x00. |

Notes:

Values of `type`:

| Value | Meaning |
|---|---|
| 0 | Normal |
| 1 | System |
| 2 | Emote |
| 6 | Label |
| 7 | Focus |
| 8 | Whisper |
| 9 | Yell |
| 10 | Spell |
| 13 | Guild |
| 14 | Alliance |
| 15 | Command |
| 16 | GM chat |

Length = 44 + len(text) + 1.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `AsciiMessage`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketMessageASCII`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Talk`
