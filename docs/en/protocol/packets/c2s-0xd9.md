# 0xD9 - Spy on Client (hardware)

Direction: client -> server. Size: 268 bytes (fixed). Available: before login and in game.

Purpose: Sends the client's hardware and system data.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD9 |
| 1 | 1 | u8 | clientType | client type |
| 2 | 4 | u32 | instanceId | instance id |
| 6 | 4 | u32 | osMajor | operating system major version |
| 10 | 4 | u32 | osMinor | operating system minor version |
| 14 | 4 | u32 | osRevision | revision |
| 18 | 1 | u8 | cpuManufacturer | CPU manufacturer |
| 19 | 4 | u32 | cpuFamily | family |
| 23 | 4 | u32 | cpuModel | model |
| 27 | 4 | u32 | cpuClock | clock |
| 31 | 1 | u8 | cpuQuantity | number of CPUs |
| 32 | 4 | u32 | memory | physical memory |
| 36 | 4 | u32 | screenWidth | screen width |
| 40 | 4 | u32 | screenHeight | screen height |
| 44 | 4 | u32 | screenDepth | color depth |
| 48 | 2 | u16 | directXMajor | DirectX major |
| 50 | 2 | u16 | directXMinor | DirectX minor |
| 52 | 128 | char16[64] | videoCard | video card description, UTF-16 |
| 180 | 4 | u32 | videoVendorId | vendor id |
| 184 | 4 | u32 | videoDeviceId | device id |
| 188 | 4 | u32 | videoMemory | video memory |
| 192 | 1 | u8 | distribution | distribution |
| 193 | 1 | u8 | clientsRunning | clients running |
| 194 | 1 | u8 | clientsInstalled | clients installed |
| 195 | 1 | u8 | clientsPartial | clients partially installed |
| 196 | 8 | char16[4] | language | language, UTF-16 |
| 204 | 64 | char16[32] | unknown | Not specified |

Notes:

The byte order of the numeric fields and the UTF-16 strings is not specified by the reference servers (which only ignore the content). Client 7.0.10.3 only sends this packet while respecting a minimum interval between sends.

Implementation reference:

- Source-X: `src/network/receive.cpp` (`PacketHardwareInfo`)
- ClassicUO size table: `src/ClassicUO.Client/Network/PacketsTable.cs` (268 bytes)
