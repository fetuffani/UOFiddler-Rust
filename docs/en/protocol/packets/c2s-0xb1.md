# 0xB1 - Gump Menu Response

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Response to a gump (button, switches and text fields).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB1 |
| 1 | 2 | u16 | length | total size |
| 3 | 4 | u32 | serial | serial of the gump (object that opened it) |
| 7 | 4 | u32 | gumpId | gump type id |
| 11 | 4 | u32 | buttonId | button pressed; 0 = close |
| 15 | 4 | u32 | switchCount | number of switches set |
| 19 | 4 x s | u32[] | switches | ids of the switches set |
| 19 + 4s | 4 | u32 | textCount | number of text entries |

**Text entry (repeated textCount times)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 2 | u16 | entryId | id of the text field |
| +2 | 2 | u16 | textLength | number of characters (not bytes); maximum 239 |
| +4 | 2 x textLength | char16[] | text | UTF-16 big-endian, without trailing NUL |

Notes:

s = switchCount. Each text entry occupies 4 + 2 x textLength bytes.

Virtues gump (gumpId 0x000001CD = 461): the reference client sends only serial, gumpId and buttonId (length = 15); ServUO still accepts, with buttonId = 1 and switchCount > 0, an additional mobile serial after switchCount.
If the button does not exist in the gump, ServUO disconnects the client.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`DisplayGumpResponse`)
- Source-X: `src/network/receive.cpp` (`PacketGumpDialogRet`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_GumpResponse`, `Send_VirtueGumpResponse`)
