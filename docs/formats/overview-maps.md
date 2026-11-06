# Mapas de visão geral (Multimap.rle e facetNN.mul)

Arquivos de imagem do mapa-múndi usados pelo cliente para exibir o mapa em escala reduzida. Little-endian.

| Arquivo | Conteúdo |
|---|---|
| `Multimap.rle` | imagem de 1 bit por pixel (terra ou fundo), com RLE |
| `facet00.mul` a `facet05.mul` | imagem colorida por facet, com RLE por linha |

## Multimap.rle

### Layout

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u32 | width | largura em pixels |
| 4 | u32 | height | altura em pixels |
| 8 | u8[] | runs | trechos, até o fim do arquivo |

Cada byte de `runs`:

| Bits | Significado |
|---|---|
| 7 | 1 = terra (pixel ligado), 0 = fundo |
| 0 a 6 | comprimento do trecho (1 a 127) |

Os pixels são gerados em ordem de linhas, da esquerda para a direita e de cima para baixo, em uma única sequência contínua (um trecho pode atravessar o fim de uma linha). A soma dos comprimentos é exatamente `width * height`.

### Arquivo de exemplo

- Dimensões: 2560 x 2048 (5242880 pixels), 320056 bytes.
- Soma dos comprimentos dos trechos: 5242880.
- Nenhum byte tem comprimento 0.

### Notas

- Trechos maiores que 127 pixels são divididos em vários bytes consecutivos com o mesmo bit 7.
- O Fiddler rejeita imagens com `width` ou `height` igual a 0 ou maior que 16384.

Leitor no Fiddler: `uo_core::multimap` (`decode_multimap` com `header32 = true`).

## facetNN.mul

### Layout

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | i16 | width | largura em pixels |
| 2 | i16 | height | altura em pixels |
| 4 | linha[height] | rows | uma linha por `y`, de cima para baixo |

Cada linha:

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u32 | byte_length | tamanho em bytes dos registros da linha (múltiplo de 3) |
| 4 | registro[byte_length / 3] | records | registros da linha |

Cada registro (3 bytes):

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u8 | run | número de pixels consecutivos com esta cor (1 a 255) |
| 1 | u16 | color | cor 5:5:5 |

Regras:

- A soma dos `run` de cada linha é exatamente `width`.
- O arquivo termina logo após a última linha (sem bytes extras).
- `color == 0` indica área sem mapeamento (fora do mapa).

### Conversão de cor

`color` é composto por bits `A RRRRR GGGGG BBBBB`:

| Bits | Canal |
|---|---|
| 10 a 14 | R (0 a 31) |
| 5 a 9 | G (0 a 31) |
| 0 a 4 | B (0 a 31) |
| 15 | ver abaixo |

Expansão de 5 para 8 bits: `v8 = (v5 << 3) | (v5 >> 2)`, aplicada a cada canal. O Fiddler pinta `color == 0` como preto e opaco.

### Bit 15

Nos arquivos de exemplo o bit 15 só aparece em `facet02.mul` (114005 registros, incluindo registros com `color == 0x8000`). Nos demais facets ele nunca está ligado. O significado não foi estabelecido; o Fiddler ignora o bit ao converter a cor.

### Tabela de facets do exemplo

| Arquivo | Bytes | width | height | Pixels sem cor (color 0) | Mapa (id) | Dimensões do `map{N}.mul` |
|---|---:|---:|---:|---:|---|---|
| facet00.mul | 12035882 | 7168 | 4096 | 4425088 | 0 (Felucca) | 7168 x 4096 |
| facet01.mul | 12049697 | 7168 | 4096 | 4535048 | 1 (Trammel) | 7168 x 4096 |
| facet02.mul | 2396882 | 2304 | 1600 | 870822 | 2 (Ilshenar) | 2304 x 1600 |
| facet03.mul | 1536927 | 2560 | 2048 | 789772 | 3 (Malas) | 2560 x 2048 |
| facet04.mul | 821754 | 1448 | 1448 | 19 | 4 (Tokuno) | 1448 x 1448 |
| facet05.mul | 1305131 | 1280 | 4096 | 3441554 | 5 (Ter Mur) | 1280 x 4096 |

O número do facet é o id do mapa. As dimensões de cada facet são iguais às do `map{N}.mul` com o mesmo número: `map0.mul` e `map1.mul` têm 89915392 bytes (896 x 512 blocos de 196 bytes, 7168 x 4096 células), `map2.mul` tem 11289600 bytes (288 x 200 blocos), `map4.mul` tem 6421156 bytes (181 x 181 blocos) e `map3.mul` e `map5.mul` têm 16056320 bytes (320 x 256 e 160 x 512 blocos). O tamanho em bytes de `map3.mul` e `map5.mul` é igual; quem distingue 2560 x 2048 de 1280 x 4096 é o `width x height` do facet correspondente.

Cada facet tem escala 1:1 com as células do mapa (um pixel por célula).

### Notas

- O arquivo de exemplo `Multimap.rle` tem 2560 x 2048, as mesmas dimensões de `facet03.mul`.
- O Fiddler exige que cada linha some exatamente `width` e que o arquivo termine na última linha; caso contrário a imagem é rejeitada.

Leitor no Fiddler: `uo_core::multimap` (`decode_facet`).
