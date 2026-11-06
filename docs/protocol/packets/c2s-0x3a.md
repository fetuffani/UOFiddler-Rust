# 0x3A - Set Skill Lock

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Altera o estado de bloqueio de habilidades.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x3A |
| 1 | 2 | u16 | length | 3 + 3 x n |
| 3 | 2 | u16 | skillIndex | índice da habilidade (entrada 1) |
| 5 | 1 | u8 | lock | 0 = subir, 1 = descer, 2 = travada |

Notas:

O trecho de 3 bytes (skillIndex, lock) pode se repetir; o Source-X lê todas as entradas (n <= 100, length - 3 múltiplo de 3), o ServUO lê apenas a primeira. O cliente de referência envia uma entrada por pacote (length = 6).

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ChangeSkillLock`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`ChangeSkillLock`)
- Source-X: `src/network/receive.cpp` (`PacketSkillLockChange`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_SkillsStatusRequest`, `Send_SkillStatusChangeRequest`)
