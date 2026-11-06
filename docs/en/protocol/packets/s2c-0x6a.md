# 0x6A - Not documented

Direction: server -> client. Size: 3 bytes.

Purpose: Not documented.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6A |
| 1 | 2 | u8[2] | payload | Not documented. |

Notes:

Only the size (3) is known, from the ClassicUO table.

## Implementation reference

- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PacketsTable.cs`
