# 0x3A - Set Skill Lock

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Changes the lock state of skills.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3A |
| 1 | 2 | u16 | length | 3 + 3 x n |
| 3 | 2 | u16 | skillIndex | skill index (entry 1) |
| 5 | 1 | u8 | lock | 0 = up, 1 = down, 2 = locked |

Notes:

The 3-byte segment (skillIndex, lock) may repeat; Source-X reads all entries (n <= 100, length - 3 is a multiple of 3), while ServUO reads only the first one. The reference client sends one entry per packet (length = 6).

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`ChangeSkillLock`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`ChangeSkillLock`)
- Source-X: `src/network/receive.cpp` (`PacketSkillLockChange`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SkillsStatusRequest`, `Send_SkillStatusChangeRequest`)
