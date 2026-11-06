# Arquivos UOP do cliente

Todos os arquivos abaixo usam o contêiner descrito em `uop-container.md`. Os offsets desta página se referem ao conteúdo da entrada já descomprimido. Little-endian.

## AnimationFrame1.uop a AnimationFrame4.uop

### Finalidade

Quadros de animação de criaturas e equipamentos (formato paletizado, equivalente aos `anim*.mul` clássicos). Cada entrada contém todos os quadros de todas as direções de uma ação de um corpo.

### Entradas

Nome lógico: `build/animationlegacyframe/{body:06}/{action:02}.bin`.

| Arquivo | Entradas | Corpos distintos | Faixa de corpo |
|---|---:|---:|---|
| AnimationFrame1.uop | 287 | 12 | 130 a 1026 |
| AnimationFrame2.uop | 3911 | 87 | 323 a 1247 |
| AnimationFrame3.uop | 2791 | 72 | 655 a 1068 |
| AnimationFrame4.uop | 318 | 17 | 735 a 832 |

Um par (body, action) aparece em apenas um dos quatro arquivos (7307 pares no total). Ações de 0 a 78. Todas as entradas são zlib.

### Cabeçalho da entrada

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | char[4] | magic | `"AMOU"` |
| 4 | u32 | version | 1 |
| 8 | u32 | total_size | tamanho da entrada descomprimida |
| 12 | u32 | body | id do corpo (igual ao `{body}` do nome) |
| 16 | i16[4] | - | não interpretado |
| 24 | u32 | extra_count | número de u32 do bloco extra (256 na maioria das entradas; também aparecem valores menores, como 1) |
| 28 | u32 | header_size | 40 |
| 32 | u32 | frame_count | número de cabeçalhos de quadro |
| 36 | u32 | data_start | `40 + 4 * extra_count` |
| 40 | u32[extra_count] | extra | bloco extra; não interpretado |
| data_start | frame_header[frame_count] | frames | cabeçalhos de quadro, 16 bytes cada |

### Cabeçalho de quadro (16 bytes)

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u16 | action | id da ação (igual ao `{action}` do nome) |
| 2 | u16 | frame_id | número do quadro, começando em 1 |
| 4 | i16[4] | - | não interpretado |
| 12 | u32 | pixel_offset | distância, a partir do início deste cabeçalho de 16 bytes, até os dados do quadro |

Posição dos dados do quadro: `data_start + 16 * i + pixel_offset`, para o i-ésimo cabeçalho. O último quadro termina exatamente no fim da entrada.

### Dados do quadro

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u16[256] | palette | paleta de 256 cores |
| 512 | i16 | center_x | âncora horizontal |
| 514 | i16 | center_y | âncora vertical |
| 516 | u16 | width | largura em pixels |
| 518 | u16 | height | altura em pixels |
| 520 | run[] | runs | sequência de trechos de pixels |

Quando `width == 0` ou `height == 0` o quadro tem apenas 520 bytes (paleta e dimensões), sem trechos e sem terminador.

Cada trecho (`run`):

| Tipo | Campo | Significado |
|---|---|---|
| u32 | header | valor bruto; `0x7FFF7FFF` encerra o quadro |
| u8[n] | indices | `n` índices de paleta |

Para o header diferente de `0x7FFF7FFF`: `h = header ^ 0x80200000`, depois

- `row_x = (h >> 22) & 0x3FF`
- `row_y = (h >> 12) & 0x3FF`
- `n = h & 0xFFF`

Posição do primeiro pixel do trecho, em coordenadas da imagem `width x height`:

- `x = center_x - 0x200 + row_x`
- `y = center_y + height - 0x200 + row_y`

Os `n` pixels seguintes ocupam `x .. x+n-1` na linha `y`.

Cor: `palette[idx]`. O valor 0 é transparente; qualquer outro valor é A1R5G5B5 com o bit 15 forçado para 1 (R nos bits 10-14, G nos bits 5-9, B nos bits 0-4).

### Direções e ordem dos quadros

- Cada entrada armazena 5 direções (0 a 4) em sequência. A direção de um quadro é `(frame_id - 1) / frames_por_direcao`.
- `frames_por_direcao = round(N / 5)`, onde N é o número de quadros contando como vazios os ids ausentes entre dois ids presentes.
- As direções 5, 6 e 7 reutilizam as direções 3, 2 e 1 espelhadas horizontalmente: o pixel `x` passa a `width - 1 - x` e a âncora passa a `width - center_x`.

### Campos não interpretados

- Os quatro i16 no offset 16 do cabeçalho e no offset 4 de cada cabeçalho de quadro.
- O bloco `extra` (`extra_count` u32).

Leitor no Fiddler: `uo_core::animations` (`UopAnimationIndex`, `decode_animation_uop`).

## AnimationSequence.uop

### Finalidade

Tabela de grupos de animação por corpo: indica quais ações existem e quais são substituídas por outra ação.

### Entradas

Nome lógico: `build/animationsequence/{id:08}.bin`. 306 entradas, ids de 0 a 1247 (esparso). O `{id}` é o corpo. Todas em zlib.

### Cabeçalho

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u32 | body | id do corpo (igual ao do nome em 305 das 306 entradas) |
| 4 | u8[48] | - | não interpretado (zeros na maioria das entradas) |
| 52 | i32 | group_count | número de registros de grupo (29 em 283 entradas; também 31, 32, 48, 68) |
| 56 | group[group_count] | groups | registros de grupo |

### Registro de grupo (comprimento variável)

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | i32 | group | número da ação; em geral sequencial, 0, 1, 2, ... |
| 4 | i32 | frame_count | quadros do grupo; 0 quando a ação é substituída |
| 8 | i32 | replacement | ação que substitui esta; -1 quando não há |
| 12 | f32 | - | 9.0 na maioria; também 12.0, 6.0 e 0.0 |
| 16 | u8[48] | - | não interpretado |
| 64 | i32 | count_a | número de registros do primeiro subbloco |
| 68 | u8[32 * count_a] | a | registros de 32 bytes; não interpretados |
| 68 + 32 * count_a | i32 | count_b | número de registros do segundo subbloco |
| +4 | u32[count_b] | b | valores de 4 bytes; não interpretados |

`count_a` vale 0 em quase todos os grupos (2 ou 4 em 16 grupos); `count_b` vale 0 ou 1.

Regra de substituição: `frame_count == 0` ocorre exatamente quando `replacement != -1`. Para obter o arquivo de quadros de um corpo e uma ação, trocar `action` por `replacement` e usar o nome `build/animationlegacyframe/{body:06}/{replacement:02}.bin`. Sem registro de substituição a ação permanece a mesma.

Após o último grupo há bytes adicionais (de 245 a 1353 nos exemplos); não interpretados.

### Campos não interpretados

Bytes 4 a 51 do cabeçalho, o f32 e os 48 bytes seguintes de cada grupo, os subblocos `a` e `b` e os bytes finais.

Leitor no Fiddler: `uo_core::animations` (`UopAnimationIndex::load_sequence_uop`, `get_resolved_action`).

## MainMisc.uop

### Finalidade

Tabelas diversas do cliente. Contém 4 entradas, sem padrão de nome estabelecido, identificadas pelo `name_hash`.

| # | name_hash | Tamanho descomprimido | Conteúdo |
|---:|---|---:|---|
| 0 | `0x6B1786D68753F504` | 5013 | mapa de recursos (XML) |
| 1 | `0xC0165B63D153B1DD` | 273936 | tabela de recursos presentes (registros de 8 bytes) |
| 2 | `0x0891F809004D8081` | 193668 | tabela de texto UTF-16 (palavras-chave de fala) |
| 3 | `0x021C35A18C8BC953` | 8875 | ordem de desenho dos equipamentos (XML) |

### Entrada 0: mapa de recursos

Texto XML (UTF-8, terminador de linha CRLF) com raiz `<assetmap>`. Descreve, para cada tipo de recurso (`resourcetype`), onde o recurso fica no formato solto (`fileFormat="raw"`, diretório `Data/...`) e no formato empacotado (`fileFormat="packed"`, diretório `Build/...`).

Elementos:

| Elemento | Atributos | Significado |
|---|---|---|
| `patternmap` | `resourcetype`, `fileFormat`, `padding`, `directory`, `suffix`, `patternMatch` | nome = `directory` + número com `padding` dígitos + `suffix` |
| `bitmaskmap` | `resourcetype`, `fileFormat`, `directory`, `suffix`, com filhos `section` | o número do recurso é dividido em partes por máscara de bits; cada `section` (`mask`, `padding`, `prefix`, `isDirectory`) gera um trecho do caminho |
| `simplemap` | `resourcetype`, `fileFormat`, `resourcenumber`, `filename` | caminho fixo de um recurso único |

Tipos de recurso e pasta empacotada:

| resourcetype | Pasta empacotada | Sufixo |
|---:|---|---|
| 1 | `Build/WorldArt/` | `.dds` |
| 3 | `Build/TerrainDefinition/` | `.bin` |
| 6 | `Build/TileArt/` | `.bin` |
| 8 | `Build/TerrainTexture/` | `.dds` |
| 9 | `Build/Sectors/` (máscaras `0x3f` e `0xffffffc0`) | `.bin` |
| 11 | `Build/AnimationDefinition/` | `.bin` |
| 12 | `Build/AnimationSequence/` | `.bin` |
| 13 | `Build/EffectDefinitionCollection/` | `.bin` |
| 16 | `Build/AnimationFrame/` (máscaras `0xffff00` e `0xff`) | `.bin` |
| 25 | `Build/AnimationLegacyFrame/` (máscaras `0xffff00` e `0xff`) | `.bin` |
| 18 | `Build/Paperdoll/` (máscaras `0xffffc0` e `0x3f`) | `.bin` |
| 19 | `Build/MultiCollection/` | `.bin` |
| 20 | `Build/MultiCollection/housing.bin` | (arquivo fixo) |
| 21 | `Build/Sectors/waypoint.bin` | (arquivo fixo) |
| 22 | `Build/TileArtLegacy/` | `.dds` |
| 23 | `Build/FacetDefinition/` | `.bin` |
| 26 | `Build/TileArtEnhanced/` | `.dds` |

Em `AnimationLegacyFrame`, o número do recurso é `(body << 8) | action`: a máscara `0xffff00` dá o diretório (`{body:06}`) e `0xff` dá o arquivo (`{action:02}`), o que resulta em `build/animationlegacyframe/{body:06}/{action:02}.bin`.

### Entrada 1: tabela de recursos presentes

Sequência de registros de 8 bytes, sem cabeçalho (34242 registros).

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u64 | resource | `(resourcetype << 56) | numero` |

O byte alto é o `resourcetype` do mapa de recursos; os 56 bits baixos são o número do recurso. Contagem por tipo:

| resourcetype | Registros | Correspondência |
|---:|---:|---|
| 3 | 197 | definições de terreno |
| 6 | 32648 | números idênticos aos das entradas de `tileart.uop` |
| 11 | 767 | definições de animação |
| 12 | 306 | números idênticos aos das entradas de `AnimationSequence.uop` |
| 19 | 324 | coleções de multi |

Os registros estão ordenados por número dentro de cada tipo, com exceção do tipo 3.

### Entrada 2: tabela de texto

Texto UTF-16LE com BOM (`FF FE`), linhas terminadas em CRLF (6125 linhas). Cada linha:

```
{id decimal}<TAB>*{texto}*
```

O mesmo `id` pode se repetir em várias linhas (sinônimos). Os textos são palavras-chave de fala em vários idiomas, por exemplo `0 *提領*`, `4 *加入*`, `168 *wo ist nujel*`.

### Entrada 3: ordem de desenho dos equipamentos

Texto XML (UTF-8) com raiz `<wearables>`.

| Elemento | Atributos | Significado |
|---|---|---|
| `<draworder>` | uma prioridade numérica por camada (`riding`, `face`, `finger1`, `ears`, `feet`, `legs`, `torso`, `talisman`, `neck`, `chest`, `abovechest`, `arms`, `hands`, `lwrist`, `skirt`, `dress`, `waist`, `feetlegs`, `cape`, `hair`, `facialhair`, `head`, `backpack`, `righthand`, `lefthand`, `drag`); `direction` opcional | prioridade numérica de desenho de cada camada. O segundo bloco (`direction="paperdoll"`) vale para o paperdoll |
| `<wearable>` | uma camada com lista de ids de item separados por vírgula, ou `any` | seleciona o item que recebe regras |
| `<override>` | `draworderpriority`, `direction` opcional, e camadas com lista de ids ou `any` | quando as camadas listadas estão equipadas (e a direção coincide), o item do `<wearable>` pai passa a usar `draworderpriority`; o valor 0 significa não desenhar |

As direções usadas em `direction` são `south`, `southeast`, `east`, `northeast`, `north`, `northwest`, `west`, `southwest` e `paperdoll`.

Leitor no Fiddler: `uo_core::paperdoll` (blocos `<draworder>`); as demais entradas: não interpretado (apenas hex na aba Arquivos).

## string_dictionary.uop

### Finalidade

Lista indexada de strings usadas pelo cliente (nomes de shaders e caminhos de texturas).

### Entradas

Uma entrada: `build/stringdictionary/string_dictionary.bin` (2989218 bytes descomprimidos, zlib).

### Layout

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u64 | timestamp | FILETIME do Windows (intervalos de 100 ns desde 1601-01-01); 2012-01-20 04:30:10 no exemplo |
| 8 | u32 | count | 93404 |
| 12 | u32 | max_length | 72, igual ao tamanho da maior string |
| 16 | string[] | strings | sequência até o fim da entrada |

Cada string: `u16 length` seguido de `length` bytes ASCII, sem terminador. O exemplo contém 93403 strings, nenhuma vazia; a primeira é `UOSpriteShader`, seguida de caminhos como `Data\WorldArt\00008600_NO_DRAW.tga` e `Data\TileArtLegacy\0.tga`.

### Campos não interpretados

A diferença de uma unidade entre `count` (93404) e o número de strings presentes (93403).

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos).

## tileart.uop

### Finalidade

Descritores binários indexados por id, em duas faixas (0 a 39212 e 10000000 a 10001247).

### Entradas

32648 entradas, todas zlib, nome lógico `build/tileart/{id:08}.bin`:

| Faixa de id | Entradas |
|---|---:|
| 0 a 39212 (esparso) | 31706 |
| 10000000 a 10001247 (esparso) | 941 |
| 99999999 | 1 |

Os ids dessas entradas coincidem com os registros de tipo 6 da tabela de recursos de `MainMisc.uop`. O tamanho descomprimido varia de 241 a 1069 bytes. As entradas não contêm imagem.

### Cabeçalho da entrada

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u16 | version | 3 em todas as entradas |
| 2 | u32 | - | não interpretado (varia por entrada) |
| 6 | u32 | id | igual ao `{id}` do nome em 32644 das 32647 entradas numeradas (nas da faixa 10000000+ é o próprio id, sem subtração) |
| 10 | - | - | restante da estrutura, com tamanho variável; contém floats (1.0 e 1.5 aparecem logo após o cabeçalho) |

### Campos não interpretados

Tudo após o offset 10; o campo no offset 2; as 3 entradas cujo campo no offset 6 difere do id do nome.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos).

## UOPs de conteúdo legado

Estes arquivos empacotam o conteúdo dos `.mul`/`.idx` clássicos. Não existem no diretório de exemplo; o layout abaixo é o que os leitores e o conversor do Fiddler implementam.

### artLegacyMUL.uop

- Nome: `build/artlegacymul/{i:08}.tga`, `i` de 0 a `0x13FFF`.
- Payload: exatamente os bytes que `art.mul` guarda para o índice `i` (terrenos de 0 a `0x3FFF`, estáticos a partir de `0x4000`).
- Mapeamento para MUL/IDX: `artidx.mul` + `art.mul`. Registro do `artidx.mul` (12 bytes): `i32 offset`, `i32 length`, `i32 extra`. Entrada ausente = `-1, -1, -1`. O campo `extra` é gravado como 0.

### gumpartLegacyMUL.uop

- Nome: `build/gumpartlegacymul/{i:08}.tga`, `i` de 0 a `0xFFFF`.
- Payload: `u32 width`, `u32 height`, seguidos dos bytes que `gumpart.mul` guarda para o índice `i`.
- Mapeamento para MUL/IDX: `gumpidx.mul` + `gumpart.mul`. No `gumpidx.mul` o campo `extra` é `(width << 16) | height`; os dois valores saem do início do payload do UOP.

### soundLegacyMUL.uop

- Nome: `build/soundlegacymul/{i:08}.dat`.
- Mapeia `soundidx.mul` + `sound.mul`. O conversor não trata este arquivo, porque as entradas do UOP não trazem o campo `extra` do índice clássico.

### map0LegacyMUL.uop a map5LegacyMUL.uop

- Nome: `build/map{N}legacymul/{i:08}.dat`.
- Payload: pedaços consecutivos de `map{N}.mul` com `0xC4000` bytes (802816 = 4096 blocos de 196 bytes); o último pedaço pode ser menor.
- Mapeamento: concatenar os pedaços em ordem de `i` reproduz `map{N}.mul`. Não há `.idx`.

Tamanho dos arquivos `map*.mul` de exemplo, em pedaços de `0xC4000`:

| Arquivo | Bytes | Pedaços | Dimensões |
|---|---:|---:|---|
| map0.mul | 89915392 | 112 | 7168 x 4096 |
| map1.mul | 89915392 | 112 | 7168 x 4096 |
| map2.mul | 11289600 | 14,06 | 2304 x 1600 |
| map3.mul | 16056320 | 20 | 2560 x 2048 |
| map4.mul | 6421156 | 7,998 | 1448 x 1448 |
| map5.mul | 16056320 | 20 | 1280 x 4096 |

### MultiCollection.uop

- Nome: `build/multicollection/{id:06}.bin` e `build/multicollection/housing.bin`.
- Payload de cada multi:

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u32 | multi_id | id do multi |
| 4 | u32 | count | número de componentes |
| 8 | componente[count] | components | componentes, 14 bytes mais dados extras |

Componente:

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u16 | item_id | id do item |
| 2 | i16 | x | deslocamento X |
| 4 | i16 | y | deslocamento Y |
| 6 | i16 | z | deslocamento Z |
| 8 | u16 | flags | flags do componente |
| 10 | u32 | extra | número de u32 que seguem o componente |
| 14 | u32[extra] | - | dados extras; não interpretados |

O conversor não trata este arquivo.

### GameData.uop

Tabelas CSV (texto) do cliente 3D. Entradas conhecidas pelo `name_hash`:

| name_hash | Tabela | Colunas |
|---|---|---|
| `0xE840D2286751791C` | legacycontainers.csv | gumpId, left, top, right, bottom, name |
| `0x78AFB72A49CD9BE9` | legacy_to_new_items.csv | legacyId, newId, newSub |
| `0xBF2AE37FAFC672E7` | body_to_mesh.csv | BodyType, arquivo, descrição |
| `0xF45CF6B325F9D01A` | recipes.csv | não interpretado |

Linhas em branco e iniciadas por `#` são comentários; a primeira linha é o cabeçalho.

Leitor no Fiddler: `uo_core::gamedata` (GameData); `uo_core::multis` (MultiCollection); `uo_core::file_index` (`from_uop`, para art/gump/map por padrão `build/{nome}/{i:08}{ext}`); os demais: `uo_core::client_files`.

## Ferramenta uop_convert

Converte entre os UOPs legados e os arquivos MUL/IDX clássicos.

```
uop_convert to-mul <pasta-origem> <pasta-destino>
uop_convert to-uop <pasta-origem> <pasta-destino>
```

- `to-mul`: lê `artLegacyMUL.uop`, `gumpartLegacyMUL.uop` e `map0..5LegacyMUL.uop` da pasta de origem e grava `art.mul` + `artidx.mul`, `gumpart.mul` + `gumpidx.mul` e `map{N}.mul`.
- `to-uop`: faz o caminho inverso. Os UOPs gerados têm `version = 5`, `block_capacity = 100`, entradas sem compressão (`compression = 0`), `header_length = 0` e `data_hash = 0`.
- Tipos suportados: art, gump e map. Sound e multi não são convertidos.
- Nomes de arquivo são comparados sem distinção de maiúsculas. Cada tipo encontrado na pasta é convertido; ao final a ferramenta imprime a contagem de convertidos e de erros (código de saída 1 se houver erro).

Regras de conversão:

- Art: payload copiado sem alteração; o índice grava `offset`, `length` e `extra = 0`.
- Gump: o `extra` do índice é separado em `width = extra >> 16` e `height = extra & 0xFFFF` e gravado como os dois u32 iniciais do payload do UOP (e o inverso em `to-mul`).
- Map: `to-uop` divide `map{N}.mul` em pedaços de `0xC4000`; `to-mul` concatena os pedaços a partir de `i = 0` e para na primeira entrada ausente.
- Entradas do índice com `offset < 0` ou `length <= 0` não geram entrada no UOP.

Leitor no Fiddler: `uo_core::uop_convert` (`uop_to_mul`, `mul_to_uop`); binário `uo_core/src/bin/uop_convert.rs`.
