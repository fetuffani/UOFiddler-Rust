# 0x0B - Damage

Direção: servidor -> cliente. Tamanho: 7 bytes.

Finalidade: Informa ao cliente a quantidade de dano sofrida por uma entidade (exibida como número flutuante).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x0B |
| 1 | 4 | u32 | serial | Entidade que sofreu dano. |
| 5 | 2 | u16 | damage | Dano; 0 não exibe nada. Limitado a 0..0xFFFF. |

Notas:

Variante antiga: subcomando 0x22 do pacote 0xBF (serial u32 + damage u8, limitado a 255); ver pacote 0xBF.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `DamagePacket`, `DamagePacketOld`
- ModernUO: `Projects/Server/Network/Packets/OutgoingDamagePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketCombatDamage`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `Damage`
