# 0xAC - Gump Text Entry Dialog Reply

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Response to the text entry dialog (server 0xAB).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAC |
| 1 | 2 | u16 | length | total size |
| 3 | 4 | u32 | serial | serial of the dialog/object |
| 7 | 1 | u8 | parentId | id of the parent dialog |
| 8 | 1 | u8 | buttonId | button pressed |
| 9 | 1 | u8 | ok | 1 = OK; 0 = canceled |
| 10 | 2 | u16 | textLength | length of the text (the reference client includes the NUL) |
| 12 | n | char[] | text | ASCII |

Notes:

ServUO does not register this id; Source-X reads parentId and buttonId together as a u16 context.

Implementation reference:

- Source-X: `src/network/receive.cpp` (`PacketGumpValueInputResponse`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TextEntryDialogResponse`)
