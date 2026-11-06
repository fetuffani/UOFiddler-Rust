# 0x6E - Character Animation

Direção: servidor -> cliente. Tamanho: 14 bytes.

Finalidade: Executa uma animação em um mobile.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6E |
| 1 | 4 | u32 | serial | Mobile. |
| 5 | 2 | u16 | action | Ação de animação. |
| 7 | 2 | u16 | frameCount | Número de quadros. |
| 9 | 2 | u16 | repeatCount | Repetições. |
| 11 | 1 | bool | reverse | 1 = toca ao contrário. |
| 12 | 1 | bool | repeat | 1 = repete. |
| 13 | 1 | u8 | delay | Atraso entre quadros. |

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `MobileAnimation`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketAction`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `CharacterAnimation`
