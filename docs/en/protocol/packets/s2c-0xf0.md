# 0xF0 - Bulk movement and Krrios special packet
Direction: client -> server (movement) and both (Krrios special). Size: variable.
Purpose: Id shared by two functions.

Bulk movement (C->S, KR/SA clients with the feature enabled):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF0 |
| 1 | 2 | u16 | length | `4 + 34 * steps` |
| 3 | 1 | u8 | steps | Number of steps |
| 4 | 34 per step | step[] | steps | See below |

Step (34 bytes):

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 8 | i64 | time1 | Timestamp (anti fast-walk; ignored by Source-X) |
| 8 | 8 | i64 | time2 | Timestamp |
| 16 | 1 | u8 | sequence | Step sequence |
| 17 | 1 | u8 | direction | Direction |
| 18 | 4 | u32 | mode | 1 = walk, 2 = run |
| 22 | 4 | u32 | x | Position (not used by the server) |
| 26 | 4 | u32 | y | |
| 30 | 4 | u32 | z | |

Notes: Source-X only describes the format; validating `length` against the received size distinguishes the movement from the Krrios special packet. ServUO, ModernUO and ClassicUO do not process this format. Source-X notes that the field requires enabling the feature at login (0xA9).

Krrios/Razor special packet (ClassicUO):

| Direction | Body after `length` | Description |
|---|---|---|
| S->C `type 0x00` | none | Handshake acceptance; the client enables the group/guild map |
| S->C `type 0x01` | repeat `u32 serial, u16 x, u16 y, u8 map` until serial 0 | Positions of the group members |
| S->C `type 0x02` | `u8 locations` and, if true, repeat `u32 serial, u16 x, u16 y, u8 map, u8 hits` until serial 0 | Guild positions |
| S->C `type 0x03`, `0x04`, `0xF0` | ignored | Runebook, guardline |
| S->C `type 0xFE` | none | Razor ACK request |
| C->S `0xFF` | none | Razor ACK (3 bytes) |
| C->S `0x00` | none | Requests group positions |
| C->S `0x01` + `u8 1` | | Requests guild positions |


Implementation reference:
- Source-X: `src/network/receive.cpp` (`PacketMovementReqNew`)
- ClassicUO: `PacketHandlers.cs` (`KrriosClientSpecial`), `OutgoingPackets.cs` (`Send_RazorACK`, `Send_QueryGuildPosition`, `Send_QueryPartyPosition`)
