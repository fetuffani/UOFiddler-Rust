# 0xD2 - Extended 0x20 (Mobile Update)
Direction: server -> client. Size: 25 bytes.
Purpose: Extended variant of the mobile update (0x20).

Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; u8/u16/u32 are unsigned, i8/i16/i32 are signed.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD2 |
| 1 | 16 | u8[16] | body | Same prefix as 0x77 according to ClassicUO: serial u32, graphic u16, x u16, y u16, z i8, direction u8, hue u16, flags u8, notoriety u8 |
| 17 | 8 | u8[8] | extra | Not specified |

Notes: ClassicUO handles 0xD2 with the same parser as 0x77; Source-X only defines the id. The final 8 bytes and their relation to the 0x20 layout are not specified in the code.

## Implementation reference
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateCharacter`)
- Source-X: `src/network/send.cpp` (`sphereproto.h`: `XCMD_CharMove2`)
