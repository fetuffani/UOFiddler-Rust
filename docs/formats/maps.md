# Mapas (facets)

O mundo é dividido em facets numeradas (`mapN`). Cada facet é uma grade de blocos de 8 x 8 células. Três arquivos descrevem uma facet: o terreno (`mapN.mul`), o índice dos estáticos (`staidxN.mul`) e os estáticos (`staticsN.mul`). Arquivos de diferença (`mapdif`, `stadif`) substituem blocos inteiros. Little-endian em todos os campos.

## Facets e dimensões

| N | Nome | Largura (células) | Altura (células) | Blocos (L x A) | Total de blocos | Tamanho de `mapN.mul` |
|---|---|---|---|---|---|---|
| 0 | Felucca | 7168 | 4096 | 896 x 512 | 458752 | 89915392 |
| 1 | Trammel | 7168 | 4096 | 896 x 512 | 458752 | 89915392 |
| 2 | Ilshenar | 2304 | 1600 | 288 x 200 | 57600 | 11289600 |
| 3 | Malas | 2560 | 2048 | 320 x 256 | 81920 | 16056320 |
| 4 | Tokuno | 1448 | 1448 | 181 x 181 | 32761 | 6421156 |
| 5 | Ter Mur | 1280 | 4096 | 160 x 512 | 81920 | 16056320 |

O tamanho de `mapN.mul` é `blocos * 196`. A altura é fixa por facet; a largura em blocos é `tamanho_do_arquivo / 196 / (altura / 8)`. O leitor usa a altura da tabela e calcula a largura a partir do tamanho do arquivo.

Coordenadas: a célula `(x, y)` pertence ao bloco `(bx, by) = (x >> 3, y >> 3)`, e é a célula de índice `(y & 7) * 8 + (x & 7)` dentro dele (linha a linha).

Número do bloco (usado em `mapN.mul`, `staidxN.mul` e nos arquivos de lista dos diffs):

```
numero = bx * (altura / 8) + by
```

Ou seja, os blocos estão ordenados coluna a coluna (todos os `by` de um mesmo `bx` em sequência).

## mapN.mul (terreno)

Propósito: tile de terreno e altura de cada célula.

Arquivo: `map0.mul` a `map5.mul`. Sequência de blocos de 196 bytes, sem cabeçalho de arquivo; o bloco `numero` está em `numero * 196`.

Bloco (196 bytes):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 | `header` | nas cópias de referência é igual ao número do bloco; o leitor o ignora |
| 4 | célula[64] | `cells` | 64 células de 3 bytes, na ordem descrita acima |

Célula (3 bytes):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u16 | `tile_id` | id do terreno (0 a 0x3FFF; arte e flags em `art.mul`/`tiledata.mul`) |
| 2 | i8 | `z` | altura (-128 a 127) |

Leitor no Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::load_map`, `land_block`)

## staidxN.mul e staticsN.mul (estáticos)

Propósito: itens fixos do mundo (árvores, paredes, construções) por bloco.

`staidxN.mul` tem uma entrada de 12 bytes por bloco, na ordem do número do bloco (`numero * 12`). É o índice genérico descrito em [index-mul.md](index-mul.md):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | i32 | `lookup` | posição dos registros do bloco em `staticsN.mul`; `-1` = bloco sem estáticos |
| 4 | i32 | `length` | bytes ocupados (múltiplo de 7); `-1` = sem estáticos |
| 8 | i32 | `extra` | não usado (nas cópias de referência: -1 em bloco vazio, `0xAAAAAAAA` em bloco com dados) |

O tamanho de `staidxN.mul` é `blocos * 12` (ex.: facet 0, 5505024 bytes). Entrada válida: `lookup >= 0` e `length > 0`; qualquer outra combinação é um bloco vazio.

`staticsN.mul` contém, sem cabeçalho, os registros de 7 bytes de cada bloco, um bloco após o outro:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u16 | `item_id` | id do item (arte = `0x4000 + item_id`) |
| 2 | u8 | `x` | coluna dentro do bloco (0 a 7) |
| 3 | u8 | `y` | linha dentro do bloco (0 a 7) |
| 4 | i8 | `z` | altura |
| 5 | u16 | `hue` | matiz; 0 = sem matiz (ver [tables.md](tables.md)) |

Posição absoluta do estático: `X = bx * 8 + x`, `Y = by * 8 + y`. Número de registros do bloco: `length / 7`. Um bloco pode ter vários registros na mesma célula. Na facet 0 de referência há 2848162 registros, todos com `x` e `y` entre 0 e 7.

Leitor no Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::static_block`)

## Arquivos de diferença

Os arquivos de diferença trazem blocos substitutos. Quando um bloco está listado, o bloco do arquivo base é descartado e o do diff é usado inteiro. Existem para as facets 0, 1 e 2 na cópia de referência.

### mapdifN.mul e mapdiflN.mul (terreno)

- `mapdifN.mul`: sequência de blocos de 196 bytes no mesmo formato de `mapN.mul` (4 bytes de cabeçalho sem significado conhecido, seguidos de 64 células).
- `mapdiflN.mul`: lista de `u32`, um por bloco de `mapdifN.mul`. O valor na posição `i` é o número do bloco (fórmula acima) que o bloco `i` de `mapdifN.mul` substitui.

O número de `u32` em `mapdiflN.mul` é igual a `tamanho(mapdifN.mul) / 196` (facet 0: 1405; facet 1: 8034; facet 2: 268). Os números dos blocos são únicos e menores que o total de blocos da facet. Em `mapdif0.mul` a lista não está em ordem crescente.

### stadifN.mul, stadifiN.mul e stadiflN.mul (estáticos)

- `stadifN.mul`: registros de estáticos de 7 bytes, no formato de `staticsN.mul`.
- `stadifiN.mul`: índice com entradas de 12 bytes (`lookup`, `length`, `extra`), uma por bloco substituto, apontando para `stadifN.mul`.
- `stadiflN.mul`: lista de `u32` (número do bloco substituído); o valor na posição `i` corresponde à entrada `i` de `stadifiN.mul`.

`stadifiN.mul` e `stadiflN.mul` têm o mesmo número de entradas (facet 0: 1807; facet 1: 10931; facet 2: 268). Uma entrada com `lookup = -1` ou `length = 0` significa que o bloco passa a não ter nenhum estático.

Para ler um bloco com os diffs ativos: procurar o número do bloco na lista (`mapdifl`/`stadifl`); se existir na posição `i`, usar o bloco `i` do arquivo de diff; senão usar o arquivo base. O leitor permite ligar e desligar o uso dos diffs.

Campos não interpretados: o `header` de 4 bytes de cada bloco de `mapdifN.mul` e o campo `extra` de `stadifiN.mul`.

Leitor no Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::load_diff`, `land_block`, `static_block`, `diff_counts`)

## Variantes "x" (map0x.mul, map1x.mul, staidx0x.mul, staidx1x.mul, statics0x.mul, statics1x.mul)

Existem para as facets 0 e 1 e têm exatamente a mesma estrutura dos arquivos sem o sufixo (`map0x.mul` 89915392 bytes, `staidx0x.mul` 5505024 bytes). Diferem do conjunto base no conteúdo: `map0x.mul` tem 5245 blocos diferentes de `map0.mul` e `map1x.mul` tem 156 de `map1.mul`; `statics0x.mul` tem 2889031 registros (contra 2848162 em `statics0.mul`).

Campos não interpretados: finalidade das variantes. Não há `mapdif`/`stadif` correspondentes.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## map0.txt

Arquivo de texto com fim de linha CRLF (algumas linhas terminam só em LF). Cada linha tem a forma `x,y,z` (inteiros decimais), por exemplo `0,0,-5`. Contém 482505 linhas, todas com coordenadas distintas da facet 0 (`x` de 0 a 119, `y` de 0 a 4095). O valor `z` de cada linha coincide com o `z` da célula de terreno correspondente em `map0.mul` (482505 de 482505 linhas) e cerca de 99,8% das células têm terreno de id 168 a 171.

Campos não interpretados: finalidade do arquivo (a seleção das células listadas não foi determinada).

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)
