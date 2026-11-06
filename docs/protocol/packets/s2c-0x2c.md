# 0x2C - Resurrection Menu / Death

Direção: servidor -> cliente. Tamanho: 2 bytes.

Finalidade: Controla a tela de morte e o menu de ressurreição.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2C |
| 1 | 1 | u8 | action | Ação (tabela). |

Notas:

| Valor | Significado |
|---|---|
| 0 | Personagem morreu (tela de morte; valor enviado pelo servidor em ServUO e Source-X) |
| 1 | Menu de ressurreição (o cliente não inicia a tela de morte) |
| 2 | Vivo (ModernUO envia este valor em `SendDeathStatus`; também é o valor que o cliente envia) |

O ClassicUO inicia a tela de morte para qualquer valor diferente de 1.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `DeathStatus`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendDeathStatus`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketDeathMenu`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `DeathScreen`
