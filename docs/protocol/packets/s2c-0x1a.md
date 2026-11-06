# 0x1A - Object Info (item no chão)

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Cria ou atualiza um item no mundo; campos opcionais são sinalizados por bits em serial, graphic, x e y.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1A |
| 1 | 2 | u16 | length | Tamanho total. |
| 3 | 4 | u32 | serial | Bit 31 (0x80000000) = o campo amount está presente; os demais 31 bits são o serial. |
| 7 | 2 | u16 | graphic | ItemID. Bit 15 (0x8000) = byte graphicOffset presente (ClassicUO). Valores >= 0x4000 indicam multi (ServUO envia ItemID | 0x4000). |
| opc. | 1 | u8 | graphicOffset | Presente se graphic & 0x8000; somado ao ItemID. |
| opc. | 2 | u16 | amount | Presente se bit 31 do serial; senão amount = 1. |
| - | 2 | u16 | x | Bit 15 = byte direction/light presente; coordenada = x & 0x7FFF. |
| - | 2 | u16 | y | Bit 15 = hue presente; bit 14 = flags presente; coordenada = y & 0x3FFF. |
| opc. | 1 | u8 | direction | Presente se x & 0x8000 (direção; ServUO envia o nível de luz do item). |
| - | 1 | i8 | z | Altitude. |
| opc. | 2 | u16 | hue | Presente se y & 0x8000. |
| opc. | 1 | u8 | flags | Presente se y & 0x4000. |

Notas:

- Ordem dos campos no fluxo: serial, graphic, [graphicOffset], [amount], x, y, [direction], z, [hue], [flags]. Os offsets fixos são 0, 1, 3 e 7; o resto depende dos bits.
- Tamanho mínimo: 14 bytes (sem opcionais). Máximo: 20 bytes.
- `flags` (item): 0x20 = movível, 0x80 = invisível.
- Clientes 7.0.0.0 e posteriores recebem o 0xF3 no lugar deste pacote.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `WorldItem`, `Item.GetPacketFlags`
- ModernUO: `Projects/Server/Network/Packets/OutgoingItemPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketItemWorld`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `UpdateItem`
