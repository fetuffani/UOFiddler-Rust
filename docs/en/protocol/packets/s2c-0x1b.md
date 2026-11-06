# 0x1B - Login Confirm (character entered)

Direction: server -> client. Size: 37 bytes.

Purpose: Confirms entry into the world and informs the character's serial, body, position and map size.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1B |
| 1 | 4 | u32 | serial | Character serial. |
| 5 | 4 | u32 | zero | 0. |
| 9 | 2 | u16 | body | Body. |
| 11 | 2 | u16 | x | X position. |
| 13 | 2 | u16 | y | Y position. |
| 15 | 2 | u16 | z | Z position (the client reads it as i8 from the value). |
| 17 | 1 | u8 | direction | Direction; the client uses the low 3 bits. |
| 18 | 1 | u8 | zero | 0. |
| 19 | 4 | i32 | minusOne | 0xFFFFFFFF. |
| 23 | 4 | u32 | zero | 0 (ServUO writes two u16 zeros, ModernUO one u32 zero; equivalent). |
| 27 | 2 | u16 | mapWidth | Map width (default 6144). |
| 29 | 2 | u16 | mapHeight | Map height (default 4096). |
| 31 | 6 | u8[6] | padding | Zeros. |

Notes:

Total: 37 bytes, same as the value in the packets.md table.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `LoginConfirm`
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendLoginConfirmation`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketPlayerStart`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `EnterWorld`
