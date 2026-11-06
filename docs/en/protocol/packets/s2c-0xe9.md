# 0xE9 - Highlight UI Toggle
Direction: server -> client. Size: 75 bytes.
Purpose: Toggles the interface highlight (KR tutorial).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE9 |
| 1 | 74 | u8[74] | data | Not specified |

Notes: Source-X only defines the id (`XCMD_HighlightUIToggle`).

## Implementation reference
- Source-X: `src/network/send.cpp` (`src/common/sphereproto.h`)
