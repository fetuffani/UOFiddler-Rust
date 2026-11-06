# 0x3A - Skills List/Update

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Sends the full skills list or the update of a single skill.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3A |
| 1 | 2 | u16 | length | Total size. |
| 3 | 1 | u8 | type | List format (table). |

**Entry (repeated from offset 4)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 2 | u16 | skillId | Base 1 for type 0x00/0x02; base 0 for the others. |
| +2 | 2 | u16 | value | Effective value x10. |
| +4 | 2 | u16 | base | Base value x10. |
| +6 | 1 | u8 | lock | Lock state. |
| +7 | 2 | u16 | cap | Only types with cap. |

**Terminator (type 0x00 and 0x02)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| end | 2 | u16 | zero | 0x0000. |

**Name entry (type 0xFE, after u16 count)**

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| +0 | 1 | bool | hasButton | 1 if the skill has a use button. |
| +1 | 1 | u8 | nameLength |  |
| +2 | n | ASCII | name | Name without terminator. |

Notes:

Values of `type`:

| type | Content |
|---|---|
| 0x00 | Full list, base 1 ids, no cap, terminated by id 0 |
| 0x01 | Full list, base 0 ids, with cap (opens the skills gump) |
| 0x02 | Full list, base 1 ids, with cap, terminated by id 0 (used by ServUO) |
| 0x03 | Full list, base 0 ids, with cap (opens the gump) |
| 0xDF | One skill, base 0 id, with cap (ServUO `SkillChange`) |
| 0xFF | One skill, base 0 id, no cap |
| 0xFE | Skill names (format below) |

Values in tenths (x10): 1000 = 100.0. `lock`: 0 = up, 1 = down, 2 = locked.
Full list packet size: 6 + 9 * N (ServUO). Size of one skill: 13 bytes with cap.
When the type has no cap, the client assumes 1000.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `SkillUpdate`, `SkillChange`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendSkillsUpdate`, `SendSkillChange`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketSkills`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateSkills`
