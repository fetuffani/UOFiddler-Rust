# Pares .idx/.mul do cliente

Vários recursos do cliente (arte, gumps, sons, multis, skills, texturas, luzes e animações) são guardados como um arquivo de dados `.mul` acompanhado de um índice `.idx`. O índice é uma tabela de entradas de tamanho fixo; o arquivo de dados contém os blocos referenciados por ela. Este documento descreve o formato genérico do par e o conteúdo de cada recurso. Animações (`anim*.idx`/`anim*.mul`) usam o mesmo formato de índice e estão em [animations.md](animations.md).

Little-endian em todos os campos. Cores de 16 bits seguem o formato descrito em [tables.md](tables.md) (seção "Formato de cor").

## Formato genérico do índice

Propósito: localizar, por número de entrada, um bloco de bytes dentro do arquivo de dados.

Arquivos (índice / dados):

| Índice | Dados | Recurso |
|---|---|---|
| `artidx.mul` | `art.mul` | arte de terreno e de itens |
| `gumpidx.mul` | `gumpart.mul` | gumps (interface e paperdoll) |
| `soundidx.mul` | `sound.mul` | efeitos sonoros |
| `multi.idx` | `multi.mul` | multis (casas, barcos) |
| `Skills.idx` | `skills.mul` | nomes de skills |
| `texidx.mul` | `texmaps.mul` | texturas de terreno |
| `lightidx.mul` | `light.mul` | máscaras de luz |
| `anim.idx`, `anim2.idx` a `anim5.idx` | `anim.mul`, `anim2.mul` a `anim5.mul` | animações |

Cada entrada do índice ocupa 12 bytes. O número de entradas é `tamanho_do_idx / 12`; a entrada `n` começa em `n * 12`.

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | i32 | `lookup` | posição, em bytes, do bloco no arquivo de dados |
| 4 | i32 | `length` | tamanho do bloco em bytes |
| 8 | i32 | `extra` | valor auxiliar, dependente do recurso (ver cada seção) |

Para ler a entrada `n`: ler `length` bytes a partir de `lookup` no arquivo de dados.

Entrada válida: `lookup >= 0` e `length > 0`. Entrada inválida (ausente):

- `lookup = -1` (0xFFFFFFFF), com `length` e `extra` também -1 ou com valores quaisquer (`artidx.mul` tem `length` positivo em entradas com `lookup = -1`);
- ou `length = 0`, com `lookup` qualquer (comum em `gumpidx.mul`, `texidx.mul` e nos índices de animação).

Notas:

- O número da entrada é o identificador do recurso (id da arte, id do gump, id do som etc.).
- Os blocos são contíguos no arquivo de dados nas cópias de referência (o maior `lookup + length` é igual ao tamanho do `.mul`), mas o leitor não depende disso.
- O Fiddler lê no máximo um número fixo de entradas por recurso: arte `0x14000`, gumps `0xFFFF`, multis `0x4000`, texturas `0x4000`, luzes 100, sons `0xFFF`, skills 100.

Leitor no Fiddler: `crates/uo_core/src/file_index.rs` (`FileIndex::from_mul_idx`, `IndexEntry::is_valid`, `FileIndex::get_data`)

## Arte (artidx.mul / art.mul)

Propósito: bitmaps de terreno (tiles de chão) e de itens (estáticos).

Faixas de ids no índice:

| Faixa de índice | Conteúdo |
|---|---|
| `0x0000` a `0x3FFF` | terreno (land), losango 44x44 |
| `0x4000` em diante | itens (estáticos); `índice = 0x4000 + id do item` |

Na cópia de referência `artidx.mul` tem 81884 entradas (0x13FDC); 2855 ids de terreno e 31682 de itens são válidos; o maior índice válido é 55596 (item 39212). `extra` é sempre 0. O arquivo `radarcol.mul` tem uma cor por índice de arte (ver [tables.md](tables.md)).

### Terreno (índice < 0x4000)

Entrada de 2048 bytes (1024 palavras u16), sem cabeçalho. Os 1012 primeiros pixels formam o losango; as 12 palavras finais são zero. Cada pixel é uma cor de 16 bits.

Ordem dos pixels, linha a linha, da esquerda para a direita, numa imagem de 44x44:

| Linhas | Colunas preenchidas na linha `y` | Pixels na linha |
|---|---|---|
| 0 a 21 | `21 - y` até `22 + y` | `2 + 2y` (2, 4, ..., 44) |
| 22 a 43 | `y - 22` até `65 - y` | `44 - 2(y - 22)` (44, 42, ..., 2) |

As posições fora do losango são transparentes.

### Item (índice >= 0x4000)

Imagem com largura e altura variáveis, codificada em corridas por linha.

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 | `data_size` | tamanho em bytes da área de corridas (tudo após a tabela de linhas); igual a `length - 8 - 2*height` |
| 4 | u16 | `width` | largura em pixels (1 a 1024; máximo observado 180) |
| 6 | u16 | `height` | altura em pixels (1 a 1024; máximo observado 591) |
| 8 | u16[height] | `row_lookup` | início de cada linha, em unidades de u16, contado a partir do fim desta tabela |
| 8 + 2*height | corridas | dados | corridas de todas as linhas |

Corridas de uma linha `y` começam na palavra `4 + height + row_lookup[y]` (contada a partir do início da entrada, em u16):

| Tipo | Campo | Significado |
|---|---|---|
| u16 | `x_offset` | pixels transparentes a saltar antes da corrida |
| u16 | `x_run` | quantidade de pixels da corrida |
| u16[x_run] | `pixels` | cores de 16 bits |

A linha termina na corrida com `x_offset = 0` e `x_run = 0`. A posição horizontal acumula: `x += x_offset`, depois os `x_run` pixels, depois `x += x_run`. Pixels não cobertos são transparentes.

Notas:

- Os pixels guardados têm o bit 15 em 0 e nunca são 0; o leitor define o bit 15 ao converter (ver [tables.md](tables.md)).
- A cor é aplicada com matiz (hue) quando o item o tem; ver "Aplicação de matiz" em [tables.md](tables.md).
- Todas as entradas de item da cópia de referência decodificam sem ultrapassar `width`.

Leitor no Fiddler: `crates/uo_core/src/art.rs` (`decode_land`, `decode_static`)

### art.def e Arttable.cfg

- `art.def`: texto, redirecionamento `<id> {<id_novo>} <matiz>`; descrito em [def-files.md](def-files.md).
- `Arttable.cfg`: texto, linhas `<id> { <id>[, <id>...] } <valor>` com `valor` igual a 0 (109 linhas) ou -1 (1 linha, `3 { 3 } -1`). Exemplos: `15821 { 2140 } 0` e `16927 { 16927, 24968, 24969 } 0`.

Campos não interpretados: semântica de `Arttable.cfg` e do campo `valor`.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos) para `art.def` e `Arttable.cfg`

## Gumps (gumpidx.mul / gumpart.mul)

Propósito: imagens de interface e de paperdoll (equipamentos de 50000 em diante, ver `Equipconv.def` e `gump.def` em [def-files.md](def-files.md)).

Campo `extra` do índice: `largura << 16 | altura`, isto é, largura nos 16 bits altos e altura nos 16 baixos. Exemplo: gump 0 tem `extra = 0x028001E0` (640 x 480).

Na cópia de referência há 61244 entradas, 3164 válidas (maior id válido 61243); as inválidas têm `length = 0`.

Entrada (`altura` linhas):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32[height] | `row_lookup` | início de cada linha, em unidades de 4 bytes, contado a partir do início da entrada |
| `row_lookup[y] * 4` | corridas | dados da linha | sequência de pares até somar `width` pixels |

Cada corrida ocupa 4 bytes:

| Tipo | Campo | Significado |
|---|---|---|
| u16 | `color` | cor de 16 bits; `0` = corrida transparente |
| u16 | `run` | número de pixels repetidos com essa cor |

A linha termina quando a soma dos `run` alcança `width`; não há terminador. A primeira linha começa em `row_lookup[0] = height` (logo após a tabela). Em todas as 3164 entradas de referência a soma dos `run` de cada linha é exatamente `width`.

Notas:

- Largura e altura vêm só do `extra`; a entrada não tem cabeçalho próprio.
- Limite do leitor: largura e altura entre 1 e 4096.

Leitor no Fiddler: `crates/uo_core/src/gumps.rs` (`decode_gump`)

## Sons (soundidx.mul / sound.mul)

Propósito: efeitos sonoros em PCM.

Índice: 4096 entradas; o número da entrada é o id do som. Para as entradas válidas `extra = id + 1`. Na cópia de referência há 1633 sons válidos (maior id 1641). Entradas inválidas têm `lookup = -1` e `length = -1`, mas `extra` preenchido (`id + 1`).

Entrada de `sound.mul`:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | char[40] | `name` | nome do arquivo de origem (ex.: `d_frst01.wav`), terminado em zero; os bytes após o zero são resíduo e devem ser ignorados |
| 40 | i16[] | `pcm` | amostras PCM de 16 bits com sinal, mono, até o fim da entrada (`length - 40` bytes, sempre par) |

O Fiddler empacota o PCM num WAV de 44 bytes de cabeçalho com 22050 Hz, mono, 16 bits.

`Sound.def` redireciona ids de som (`<id> {<novo>} <matiz>`); ver [def-files.md](def-files.md).

Leitor no Fiddler: `crates/uo_core/src/sounds.rs` (`SoundEntry::parse`, `to_wav`)

## Multis (multi.idx / multi.mul)

Propósito: estruturas compostas por vários itens (casas, barcos). O número da entrada é o id do multi; o item exibe-se com arte `0x4000 + item_id`.

Na cópia de referência `multi.idx` tem 8182 entradas e 324 válidas (maior id 7667); `extra` é 0.

Uma entrada é uma lista de componentes sem cabeçalho nem contador. O número de componentes é `length / tamanho_do_registro`.

Há dois tamanhos de registro:

| Versão | Tamanho | Campos |
|---|---|---|
| clássica | 12 bytes | `item_id`, `x`, `y`, `z`, `flags` |
| High Seas (cliente 7.0.9 em diante) | 16 bytes | os mesmos mais `unk1` |

Registro (offsets iguais nas duas versões até o byte 11):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u16 | `item_id` | id do item (arte = `0x4000 + item_id`) |
| 2 | i16 | `x` | deslocamento X em relação ao ponto de origem do multi |
| 4 | i16 | `y` | deslocamento Y |
| 6 | i16 | `z` | deslocamento Z |
| 8 | u32 | `flags` | valores observados: 0 e 1 |
| 12 | u32 | `unk1` | só no registro de 16 bytes; valores observados: 0 e 1 |

Como decidir o tamanho do registro: o tamanho vale para o arquivo inteiro, não por entrada. Percorrer todas as entradas válidas:

- se algum `length` não é múltiplo de 12, o arquivo usa 16 bytes;
- se algum `length` não é múltiplo de 16, o arquivo usa 12 bytes;
- se todos forem múltiplos de ambos (ou a situação for contraditória), usar 16.

Decidir por uma única entrada é ambíguo para tamanhos múltiplos de 48 (caber em 12 e em 16). Na cópia de referência todas as 324 entradas são múltiplos de 16 e 209 não são múltiplos de 12, portanto o registro é de 16 bytes.

Campos não interpretados: significado dos bits de `flags` e de `unk1`.

Leitor no Fiddler: `crates/uo_core/src/multis.rs` (`detect_multi_record_size`, `MultiStructure::parse_sized`)

## Skills (Skills.idx / skills.mul)

Propósito: lista de nomes das skills. O número da entrada é o id da skill.

Índice: 256 entradas; 58 válidas (ids 0 a 57). `extra` assume valores como 13, 14 e 15 e não é interpretado.

Entrada:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u8 | `has_action` | 1 se a skill pode ser usada ativamente (botão de uso), 0 caso contrário |
| 1 | char[] | `name` | nome em ASCII, terminado em zero; ocupa o resto da entrada |

Exemplo: id 0 = `00 "Alchemy" 00`; id 1 = `01 "Anatomy" 00`.

Campos não interpretados: `extra` do índice.

Leitor no Fiddler: `crates/uo_gui/src/main.rs` (`render_skills_tab`, usando `FileIndex`)

## Texturas de terreno (texidx.mul / texmaps.mul)

Propósito: texturas aplicadas ao terreno (3D). O id da textura é o campo `texture_id` do registro de terreno em `tiledata.mul` (ver [tables.md](tables.md)).

Índice: 16384 entradas (0x4000); 2647 válidas (maior id 16379). O id 0 nunca é válido.

`extra` e tamanho:

| `extra` | `length` | Dimensão |
|---|---|---|
| 0 | 8192 | 64 x 64 (2330 entradas) |
| 1 | 32768 | 128 x 128 (317 entradas) |

Entrada: `dimensão * dimensão` palavras u16 (cores de 16 bits, bit 15 em 0), linha a linha, sem cabeçalho. O leitor escolhe 128 quando `extra != 0` ou `length >= 32768`.

Leitor no Fiddler: `crates/uo_core/src/textures.rs` (`decode_texture`)

## Luzes (lightidx.mul / light.mul)

Propósito: máscaras de intensidade de luz (fontes de luz, círculos).

Índice: 100 entradas; 55 válidas. `extra = altura << 16 | largura` (largura nos 16 bits baixos, altura nos altos). Exemplo: id 0 tem `extra = 0x006E006E` (110 x 110) e `length = 12100`.

Entrada: `largura * altura` bytes, um por pixel, linha a linha. Os valores são inteiros de 8 bits com sinal; na cópia de referência variam de -30 a 31. Em 54 das 55 entradas `length = largura * altura`; a entrada 50 declara 280 x 280 em `extra`, mas tem `length = 90000` (300 x 300).

Notas:

- O leitor do Fiddler trata cada byte como inteiro sem sinal e converte para cinza multiplicando por 8 (saturando em 255); valores negativos aparecem saturados.

Campos não interpretados: significado físico dos valores negativos.

Leitor no Fiddler: `crates/uo_core/src/light.rs` (`decode_light`)
