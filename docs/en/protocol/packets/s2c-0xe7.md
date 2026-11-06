# 0xE7 - Highlight UI Continue
Direction: server -> client. Size: 12 bytes.
Purpose: Interface highlight control (KR tutorial).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xE7 |
| 1 | 11 | u8[11] | data | Not specified |

Notes: Source-X only defines the id (`XCMD_HighlightUIContinue`).

## Implementation reference
- Source-X: `src/network/send.cpp` (`src/common/sphereproto.h`)
