# 0xC2 - UnicodePromptResponse

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Response to a Unicode text prompt.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC2 |
| 1 | 2 | u16 | length | total size |
| 3 | 4 | u32 | serial | serial of the prompt's sender |
| 7 | 4 | u32 | promptId | prompt id |
| 11 | 4 | u32 | type | 0 = canceled; non-zero = confirmed |
| 15 | 4 | char[4] | language | ASCII language, for example "ENU" + NUL |
| 19 | n | char16[] | text | UTF-16 little-endian terminated by NUL |

Notes:

Unlike the other Unicode texts in the protocol, this one uses little-endian. ServUO discards text longer than 128 characters.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnicodePromptResponse`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`UnicodePromptResponse`)
- Source-X: `src/network/receive.cpp` (`PacketPromptResponseUnicode`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnicodePromptResponse`)
