# 0x9A - AsciiPromptResponse

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Response to an ASCII text prompt.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9A |
| 1 | 2 | u16 | length | total size |
| 3 | 4 | u32 | serial | serial of the prompt's sender |
| 7 | 4 | u32 | promptId | prompt id |
| 11 | 4 | u32 | type | 0 = canceled; non-zero = confirmed |
| 15 | n | char[] | text | ASCII, NUL-terminated |

Notes:

ServUO discards text longer than 128 characters.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`AsciiPromptResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`AsciiPromptResponse`)
- Source-X: `src/network/receive.cpp` (`PacketPromptResponse`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ASCIIPromptResponse`)
