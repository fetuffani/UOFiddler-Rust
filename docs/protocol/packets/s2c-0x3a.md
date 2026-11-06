# 0x3A - Skills List/Update

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Envia a lista completa de skills ou a atualização de uma skill.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3A |
| 1 | 2 | u16 | length | Tamanho total. |
| 3 | 1 | u8 | type | Formato da lista (tabela). |

**Entrada (repetida a partir do offset 4)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 2 | u16 | skillId | Base 1 para type 0x00/0x02; base 0 nos demais. |
| +2 | 2 | u16 | value | Valor efetivo x10. |
| +4 | 2 | u16 | base | Valor base x10. |
| +6 | 1 | u8 | lock | Estado do lock. |
| +7 | 2 | u16 | cap | Somente tipos com cap. |

**Terminador (type 0x00 e 0x02)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| fim | 2 | u16 | zero | 0x0000. |

**Entrada de nome (type 0xFE, após u16 count)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 1 | bool | hasButton | 1 se a skill tem botão de uso. |
| +1 | 1 | u8 | nameLength |  |
| +2 | n | ASCII | name | Nome sem terminador. |

Notas:

Valores de `type`:

| type | Conteúdo |
|---|---|
| 0x00 | Lista completa, ids base 1, sem cap, terminada por id 0 |
| 0x01 | Lista completa, ids base 0, com cap (abre o gump de skills) |
| 0x02 | Lista completa, ids base 1, com cap, terminada por id 0 (usada pelo ServUO) |
| 0x03 | Lista completa, ids base 0, com cap (abre o gump) |
| 0xDF | Uma skill, id base 0, com cap (ServUO `SkillChange`) |
| 0xFF | Uma skill, id base 0, sem cap |
| 0xFE | Nomes das skills (formato abaixo) |

Valores em décimos (x10): 1000 = 100.0. `lock`: 0 = subir, 1 = descer, 2 = travada.
Tamanho do pacote de lista completa: 6 + 9 * N (ServUO). Tamanho de uma skill: 13 bytes com cap.
Quando o tipo não tem cap, o cliente assume 1000.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `SkillUpdate`, `SkillChange`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendSkillsUpdate`, `SendSkillChange`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketSkills`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateSkills`
