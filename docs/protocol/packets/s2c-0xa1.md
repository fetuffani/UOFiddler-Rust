# 0xA1 - Update Current HP
Direção: servidor -> cliente. Tamanho: 9 bytes.
Finalidade: Atualiza o valor atual e máximo de Hits de um mobile.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA1 |
| 1 | 4 | u32 | serial | Serial do mobile |
| 5 | 2 | i16 | max | Valor máximo |
| 7 | 2 | i16 | current | Valor atual |

Notas: Quando a normalização está ativa no servidor (ServUO `AttributeNormalizer`, padrão), valores de mobiles que não são o próprio jogador são enviados em escala: `max = 25` e `current = atual*25/máximo`. Sem normalização, os valores reais são enviados.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`MobileHits`/`MobileHitsN`, `MobileMana`/`MobileManaN`, `MobileStam`/`MobileStamN`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketHealthUpdate`, `PacketManaUpdate`, `PacketStaminaUpdate`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateHitpoints`, `UpdateMana`, `UpdateStamina`)
