# 0x6C - Target Cursor

Direction: server -> client. Size: 19 bytes.

Purpose: Asks the client to show a target cursor (or cancels it).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6C |
| 1 | 1 | u8 | targetType | 0 = object, 1 = ground position. |
| 2 | 4 | u32 | cursorId | Identifier returned in the response. |
| 6 | 1 | u8 | cursorType | 0 = neutral, 1 = harmful, 2 = beneficial, 3 = cancel. |
| 7 | 12 | u8[12] | padding | Zeros (x, y, z and graphic are only used in the client's response). |

Notes:

To cancel: targetType 0, cursorId 0 and cursorType 3.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `TargetReq`, `CancelTarget`
- ModernUO: `Projects/Server/Network/Packets/OutgoingTargetPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketAddTarget`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `TargetCursor`
