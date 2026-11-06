# 0x7C - Open Dialog Box (menu)

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Opens a selection menu (list of items or questions).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7C |
| 1 | 2 | u16 | length | Total size. |
| 3 | 4 | u32 | serial | Owner/source of the menu. |
| 7 | 2 | u16 | menuId | Menu identifier. |
| 9 | 1 | u8 | questionLength | Length of the text. |
| 10 | n | ASCII | question | Title, without terminator. |
| + | 1 | u8 | count | Number of options. |
| + | n | option[] | options | `count` options. |

**Option**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 2 | u16 | graphic | ItemID; 0 in text menus. |
| +2 | 2 | u16 | hue | Color. |
| +4 | 1 | u8 | textLength |  |
| +5 | n | ASCII | text | Without terminator. |

Notes:

- Item menu: each option has `graphic` != 0. Question menu (gray): `graphic` = 0 and `hue` = 0 (ServUO writes a u32 of 0); the client detects the type by reading the `graphic` of the first option.
- The client numbers the responses starting from 1.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `DisplayItemListMenu`, `DisplayQuestionMenu`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMenuPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketDisplayMenu`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `OpenMenu`
