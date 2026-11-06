# 0x7B - Sequence

Direção: servidor -> cliente. Tamanho: 2 bytes.

Finalidade: Informa um número de sequência.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7B |
| 1 | 1 | u8 | sequence | Número de sequência. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `Sequence`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendSequence`)
