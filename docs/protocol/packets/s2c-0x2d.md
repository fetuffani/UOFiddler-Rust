# 0x2D - Mobile Attributes

Direção: servidor -> cliente. Tamanho: 17 bytes.

Finalidade: Atualiza vida, mana e stamina (atual e máximo) de um mobile.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x2D |
| 1 | 4 | u32 | serial | Mobile. |
| 5 | 2 | u16 | hitsMax |  |
| 7 | 2 | u16 | hits |  |
| 9 | 2 | u16 | manaMax |  |
| 11 | 2 | u16 | mana |  |
| 13 | 2 | u16 | stamMax |  |
| 15 | 2 | u16 | stam |  |

Notas:

Os valores podem ser normalizados (escala fixa) pelo servidor quando a opção de normalização está ativa.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `MobileAttributes`, `MobileAttributesN`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMobilePackets.cs`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `MobileAttributes`
