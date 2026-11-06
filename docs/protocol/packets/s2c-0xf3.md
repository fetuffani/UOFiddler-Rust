# 0xF3 - Object Info (SA/novo)
Direção: servidor -> cliente. Tamanho: 26 bytes (clientes HS) | 24 bytes (SA, sem o campo final).
Finalidade: Apresenta um item, mobile ou multi no mundo (substitui o 0x1A).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF3 |
| 1 | 2 | u16 | command | 0x0001 |
| 3 | 1 | u8 | dataType | 0 item, 1 mobile, 2 multi, 3 item danificável |
| 4 | 4 | u32 | serial | Serial |
| 8 | 2 | u16 | graphic | Graphic (item: 15 bits no SA, 16 bits no HS; multi: 14 bits) |
| 10 | 1 | u8 | graphicOffset | 0 (incremento do graphic) |
| 11 | 2 | u16 | amount | Quantidade |
| 13 | 2 | u16 | amount2 | Repete a quantidade |
| 15 | 2 | u16 | x | X (15 bits válidos) |
| 17 | 2 | u16 | y | Y (14 bits válidos) |
| 19 | 1 | i8 | z | Z |
| 20 | 1 | u8 | light | Nível de luz do item / direção do mobile |
| 21 | 2 | u16 | hue | Hue |
| 23 | 1 | u8 | flags | Flags (tabela) |
| 24 | 2 | u16 | extra | 0 (apenas na variante de 26 bytes) |

Notas: Flags de item: 0x20 movable, 0x80 invisível. Flags de mobile: 0x01 paralisado/congelado, 0x02 feminino, 0x04 voando, 0x08 barra de vida amarela/blessed, 0x10 ignora mobiles, 0x40 war mode, 0x80 oculto. O ClassicUO sempre lê os 2 bytes finais, portanto assume a variante de 26 bytes.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`WorldItemSA`, `WorldItemHS`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateWorldEntity`)
- Source-X: `src/network/send.cpp` (`PacketItemWorldNew`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateItemSA`)
