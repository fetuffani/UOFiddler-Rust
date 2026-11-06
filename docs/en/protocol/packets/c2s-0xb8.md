# 0xB8 - ProfileReq

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Requests or changes the profile of a character.

**Profile request (length = 8)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | 8 |
| 3 | 1 | u8 | mode | 0 = display |
| 4 | 4 | u32 | serial | serial of the mobile |

**Profile edit**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | 12 + 2 x textLength |
| 3 | 1 | u8 | mode | 1 = edit |
| 4 | 4 | u32 | serial | serial of the mobile |
| 8 | 2 | u16 | command | the reference client sends 0x0001 |
| 10 | 2 | u16 | textLength | number of characters |
| 12 | 2 x textLength | char16[] | text | UTF-16 big-endian |

Notes:

ServUO rejects textLength > 511. Response to mode 0: 0xB8 Character Profile (server to client).

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ProfileReq`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMobilePackets.cs` (`ProfileReq`)
- Source-X: `src/network/receive.cpp` (`PacketProfileReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ProfileRequest`, `Send_ProfileUpdate`)
