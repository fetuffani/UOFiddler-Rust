# 0xD8 - Custom House
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Entrega ao cliente a estrutura completa de uma casa customizada, comprimida por plano.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD8 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1 | u8 | compression | 0x03 (zlib por plano) |
| 4 | 1 | u8 | response | 0x00 ou 0x01; o cliente ignora |
| 5 | 4 | u32 | serial | Serial da casa |
| 9 | 4 | u32 | revision | Número de revisão |
| 13 | 2 | u16 | itemCount | Total de componentes |
| 15 | 2 | u16 | bufferLength | 1 + soma(4 + tamanho comprimido de cada bloco) |
| 17 | 1 | u8 | blockCount | Quantidade de blocos |
| 18 | variável | block[] | blocks | Blocos |

Notas: Bloco: `u8 index` (nibble alto = modo 2 grade, 0 lista de 5 bytes, 1 lista de 4 bytes; nibble baixo = plano), `u8 sizeLow`, `u8 compressedLow`, `u8 ((size >> 4) & 0xF0) | ((compressed >> 8) & 0x0F)`, depois `compressed` bytes zlib. Os tamanhos têm 12 bits. Planos, coordenadas, blocos de offsets e o pipeline de customização estão descritos em house-customization.md (seções 3.4 e 4).

## Referência de implementação
- ServUO: `Scripts/Multis/HouseFoundation.cs` (`DesignStateDetailed`)
- ModernUO: `Projects/UOContent/Multis/Houses/HousePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketHouseDesign`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CustomHouse`)
- Documento do projeto: `docs/protocol/house-customization.md`
