# 0x3F - Update Statics (God client; UltimaLive command)

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: UltimaLive protocol packet: updates the statics of a block or executes an UltimaLive command.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3F |
| 1 | 2 | u16 | length | Total size. |
| 3 | 4 | u32 | blockNumber | Block number (x * heightInBlocks + y). |
| 7 | 4 | u32 | count | Number of 7-byte units in the payload. |
| 11 | 2 | u16 | sequence | Sequence/security (0 in the implementations). |
| 13 | 1 | u8 | command | UltimaLive command (table). |
| 14 | 1 | u8 | mapId | Map number. |
| 15 | n | u8[] | data | Command payload. |

Notes:

| command | Meaning | Payload |
|---|---|---|
| 0x00 | Update Statics | `count` statics of 7 bytes |
| 0x01 | Update Map Definitions | 9-byte entries, padded with zeros to a multiple of 7 |
| 0x02 | Login Confirmation | 28 ASCII bytes with the shard identifier, padded with 0x00 |
| 0x03 | Refresh Client | empty (15 bytes in total) |
| 0xFF | Block Query | empty (15 bytes); central block of the 25-block query |

Static (7 bytes): `artId` u16, `x` u8 (offset within the block), `y` u8, `z` i8, `hue` u16.
Map definition (9 bytes): `fileIndex` u8, `width` u16, `height` u16, `wrapWidth` u16, `wrapHeight` u16.
Login Confirmation total: 43 bytes. Block Query and Refresh: 15 bytes.

## Implementation reference

- ModernUO: `refs/modernuo/OutgoingUltimaLivePackets.cs` (UltimaLive)
- Praxiiz UltimaLive: `UltimaLive/Protocol.txt`, `UltimaLive/Network/NetworkManager.cpp` (command byte at offset 13)
- Source-X: `src/network/send.cpp`, `PacketQueryClient`
