# 0xF4 - CrashReport

Direction: client -> server. Size: variable (id + u16 length + payload). Available: before login and in game.

Purpose: Client crash report.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF4 |
| 1 | 2 | u16 | length | total size |
| 3 | 1 | u8 | major | major version |
| 4 | 1 | u8 | minor | minor version |
| 5 | 1 | u8 | revision | revision |
| 6 | 1 | u8 | patch | patch |
| 7 | 2 | u16 | x | character X |
| 9 | 2 | u16 | y | Y |
| 11 | 1 | i8 | z | Z |
| 12 | 1 | u8 | map | map |
| 13 | 32 | char[32] | account | account |
| 45 | 32 | char[32] | character | character |
| 77 | 15 | char[15] | ip | IP |
| 92 | 4 | u32 | unknown | Not specified |
| 96 | 4 | u32 | exception | exception code |
| 100 | 100 | char[100] | process | executable |
| 200 | 100 | char[100] | report | description |
| 300 | 1 | u8 | zero | 0x00 |
| 301 | 4 | u32 | offset | error offset |
| 305 | 1 | u8 | count | number of addresses (not always present) |
| 306 | 4 x count | u32[] | addresses | stack addresses |

Notes:

Source-X reads up to the error offset (field at 301); ServUO also reads the address list.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`CrashReport`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`CrashReport`)
- Source-X: `src/network/receive.cpp` (`PacketCrashReport`)
