# 0xBE - AssistVersion

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Reports the version of a third-party assistant.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBE |
| 1 | 2 | u16 | length | total size |
| 3 | 4 | u32 | unknown | Not specified |
| 7 | n | char[] | version | ASCII terminated by NUL |

Notes:

Reference servers discard the content.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`AssistVersion`)
