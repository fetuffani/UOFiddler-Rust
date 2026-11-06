# Tabelas fixas do cliente

Arquivos de tabela de tamanho e estrutura fixos: dados de tiles, matizes (hues), cores do radar, animação de tiles, fontes e paleta. Little-endian em todos os campos. Os arquivos de índice genéricos (arte, gumps, skills etc.) estão em [index-mul.md](index-mul.md).

## Formato de cor (16 bits) e aplicação de matiz

Propósito: formato de cor usado por arte, gumps, texturas, fontes, matizes e paletas de animação.

Valor de 16 bits, A1R5G5B5:

| Bits | Campo |
|---|---|
| 15 | alfa (1 = opaco) |
| 14 a 10 | vermelho (0 a 31) |
| 9 a 5 | verde (0 a 31) |
| 4 a 0 | azul (0 a 31) |

Conversão de canal de 5 para 8 bits: `v8 = floor(v5 * 255 / 31)`. Conversão de 8 para 5 bits: `v5 = floor(v8 * 31 / 255)`.

Notas:

- Nos arquivos de dados o bit 15 está sempre em 0 e os pixels de imagem nunca valem 0. O leitor define o bit 15 ao converter (`cor | 0x8000`, ou `cor ^ 0x8000` em arte de item, gumps e fontes, que dá o mesmo resultado quando o bit está em 0).
- Valor 0 após a conversão significa pixel transparente. Ao converter para RGBA, `0` vira `(0, 0, 0, 0)`; qualquer outro valor vira RGB opaco.
- Ao converter de RGBA para 16 bits, alfa menor que 128 vira 0; se os três canais de 5 bits forem 0 a cor passa a `0x8001` (para não colidir com transparente); caso contrário o bit 15 é definido.

Aplicação de matiz (hue) a uma imagem:

1. O id de matiz `n` (1 a 3000) usa o registro `n - 1` de `hues.mul`; matiz 0 significa "sem matiz".
2. Para cada pixel não transparente e não preto (RGB diferente de 0, 0, 0): `indice = floor(R8 * 31 / 255)` (o canal vermelho convertido para 5 bits); o novo RGB é a cor `indice` da paleta de 32 cores do matiz.
3. Com o modo "só cinza" (itens com o flag `PartialHue` em `tiledata.mul`), o passo 2 só vale para pixels com `R = G = B`.

Leitor no Fiddler: `crates/uo_core/src/color.rs` (`color16_to_rgba`, `rgba_to_color16`, `apply_hue`)

## tiledata.mul

Propósito: flags, nome e atributos de cada tile de terreno e de cada item.

Estrutura: duas seções em sequência, sem cabeçalho de arquivo.

1. Terreno: 512 grupos de 32 registros (16384 tiles, ids 0 a 0x3FFF).
2. Itens: grupos de 32 registros até o fim do arquivo (ids de item `0` a `0xFFFF`; a arte é `0x4000 + id`).

Cada grupo começa com um `u32` de cabeçalho (valor 0 nas cópias de referência, ignorado) seguido dos 32 registros.

### Versões e como detectar

| Versão | Flags | Registro de terreno | Registro de item | Grupo de terreno | Grupo de item | Seção de terreno |
|---|---|---|---|---|---|---|
| clássica | u32 | 26 bytes | 37 bytes | 836 bytes | 1188 bytes | 428032 bytes |
| High Seas (cliente 7.0.9 em diante) | u64 | 30 bytes | 41 bytes | 964 bytes | 1316 bytes | 493568 bytes |

Tamanhos de arquivo: clássico com 0x10000 itens = 2861056 bytes; High Seas com 0x10000 itens = 3188736 bytes (493568 de terreno + 2048 grupos de 1316 bytes).

Detecção usada pelo leitor: High Seas quando o tamanho do arquivo é maior ou igual a 3188736 bytes; caso contrário clássico. O número de itens é `(tamanho - seção_de_terreno) / tamanho_do_grupo * 32`. O arquivo de referência (3188736 bytes) é High Seas com 65536 itens.

### Registro de terreno

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 (clássico) ou u64 (High Seas) | `flags` | bits da tabela abaixo |
| 4 ou 8 | u16 | `texture_id` | id da textura em `texidx.mul`/`texmaps.mul`; 0 = sem textura |
| 6 ou 10 | char[20] | `name` | nome em ASCII, terminado em zero |

### Registro de item

`F` é o tamanho do campo `flags`: 4 bytes (clássico) ou 8 bytes (High Seas).

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 ou u64 | `flags` | bits da tabela abaixo |
| F + 0 | u8 | `weight` | peso; 255 em muitos itens fixos do cenário (ex.: `hedge`) |
| F + 1 | u8 | `quality` | em itens com `Wearable`, a camada de equipamento (1 a 23 nas cópias de referência; ex.: `chainmail tunic` = 13, `backpack` = 21) |
| F + 2 | u16 | `misc_data` | dado auxiliar (não interpretado) |
| F + 4 | u8 | `unk2` | não interpretado |
| F + 5 | u8 | `quantity` | não interpretado |
| F + 6 | u16 | `anim_id` | id de animação do equipamento (corpo do tipo `EQUIPMENT` em `mobtypes.txt`; ex.: `chainmail tunic` = 538); 0 se não há |
| F + 8 | u8 | `unk3` | não interpretado |
| F + 9 | u8 | `hue` | não interpretado |
| F + 10 | u8 | `stacking_offset` | não interpretado |
| F + 11 | u8 | `value` | não interpretado |
| F + 12 | u8 | `height` | altura do item em unidades de `z` |
| F + 13 | char[20] | `name` | nome em ASCII, terminado em zero |

Tamanho do registro: `F + 33` = 37 (clássico) ou 41 (High Seas).

Nomes podem conter marcadores de formatação do cliente (`%s`, ex.: `board%s`).

Notas:

- No arquivo de referência o registro 0 de terreno tem o nome deslocado 4 bytes (o texto `UNUSED` aparece no offset 10 do arquivo); lido pelo padrão acima, o terreno 0 resulta em flags e `texture_id` sem sentido e nome `ED`. Os demais registros seguem o padrão.
- Exemplos (High Seas): item `0x0EED` = `gold coin`, flags `0x4000801`; item `0x0E75` = `backpack`, flags `0x604002`.

### Bits de flags

Nomes usados pelo Fiddler. Os bits 32 a 37 só existem na versão High Seas.

| Bit | Máscara | Nome |
|---|---|---|
| 0 | `0x1` | Background |
| 1 | `0x2` | Weapon |
| 2 | `0x4` | Transparent |
| 3 | `0x8` | Translucent |
| 4 | `0x10` | Wall |
| 5 | `0x20` | Damaging |
| 6 | `0x40` | Impassable |
| 7 | `0x80` | Wet |
| 8 | `0x100` | Unknown1 |
| 9 | `0x200` | Surface |
| 10 | `0x400` | Bridge |
| 11 | `0x800` | Generic |
| 12 | `0x1000` | Window |
| 13 | `0x2000` | NoShoot |
| 14 | `0x4000` | PrefixA |
| 15 | `0x8000` | PrefixAn |
| 16 | `0x10000` | Internal |
| 17 | `0x20000` | Foliage |
| 18 | `0x40000` | PartialHue |
| 19 | `0x80000` | Unknown2 |
| 20 | `0x100000` | Map |
| 21 | `0x200000` | Container |
| 22 | `0x400000` | Wearable |
| 23 | `0x800000` | LightSource |
| 24 | `0x1000000` | Animated |
| 25 | `0x2000000` | NoDiagonal |
| 26 | `0x4000000` | Unknown3 |
| 27 | `0x8000000` | Armor |
| 28 | `0x10000000` | Roof |
| 29 | `0x20000000` | Door |
| 30 | `0x40000000` | StairBack |
| 31 | `0x80000000` | StairRight |
| 32 | `0x100000000` | AlphaBlend |
| 33 | `0x200000000` | UseNewArt |
| 34 | `0x400000000` | ArtUsed |
| 35 | `0x800000000` | NoHouse |
| 36 | `0x1000000000` | NoDraw |
| 37 | `0x2000000000` | HoverOver |

Campos não interpretados: `Unknown1`, `Unknown2`, `Unknown3`, `misc_data`, `unk2`, `quantity`, `unk3`, `hue`, `stacking_offset`, `value`.

Leitor no Fiddler: `crates/uo_core/src/tiledata.rs` (`TileDataManager::from_file`); nomes dos bits em `crates/uo_gui/src/main.rs` (`TILEDATA_FLAGS`)

## hues.mul

Propósito: paletas de 32 cores usadas para tingir arte, animações e texto.

Estrutura: 375 grupos, 8 matizes por grupo, 3000 matizes no total (265500 bytes = 375 * 708).

Grupo (708 bytes):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 | `header` | 0 nas cópias de referência; ignorado |
| 4 | registro[8] | `hues` | 8 registros de 88 bytes |

Registro de matiz (88 bytes):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u16[32] | `colors` | 32 cores de 16 bits (ver "Formato de cor"), do escuro ao claro |
| 64 | u16 | `table_start` | igual a `colors[0]` em todos os 3000 registros |
| 66 | u16 | `table_end` | não é a última cor; nos 1000 registros nomeados `Hue (a->b)`, `a` e `b` são `table_start` e `table_end` |
| 68 | char[20] | `name` | nome em ASCII, terminado em zero; vazio em 963 registros |

Notas:

- O id de matiz usado no jogo é 1-based: o matiz `n` é o registro de índice `n - 1` do arquivo; 0 significa "sem matiz". O registro 2999 (matiz 3000) e outros 962 registros têm nome vazio.

Campos não interpretados: uso pelo cliente de `table_start` e `table_end`.

Leitor no Fiddler: `crates/uo_core/src/hues.rs` (`HuesManager::from_file`, `Hue::get_rgba_palette`)

## radarcol.mul

Propósito: cor de cada tile no minimapa (radar).

Estrutura: sequência de `u16` sem cabeçalho, uma cor de 16 bits por índice de arte (ver [index-mul.md](index-mul.md)): índices `0` a `0x3FFF` para terreno e `0x4000 + id` para item. Na cópia de referência há 81884 valores (163768 bytes), o mesmo número de entradas de `artidx.mul`. O leitor completa com zeros até 0x14000 valores.

Cor do terreno `t`: `colors[t & 0x3FFF]`. Cor do item `i`: `colors[0x4000 + i]`.

Leitor no Fiddler: `crates/uo_core/src/radar_col.rs` (`RadarColManager`)

## animdata.mul

Propósito: sequência de animação de itens animados (fogueiras, tochas, água). Associada ao flag `Animated` de `tiledata.mul`.

Estrutura: grupos de 548 bytes, cada um com 8 registros; o tile de item de um registro é `grupo * 8 + posição` (id do item, sem o `0x4000`). O arquivo de referência tem 8187 grupos completos (4486476 bytes) mais 272 bytes finais zerados, que o leitor ignora.

Grupo:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32 | `header` | 0 nas cópias de referência |
| 4 | registro[8] | `entries` | 8 registros de 68 bytes |

Registro (68 bytes):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | i8[64] | `frames` | deslocamentos de id: o frame `k` usa a arte do item `id + frames[k]` |
| 64 | u8 | `unk` | não interpretado (0 em 65277 de 65496 registros; demais valores pequenos) |
| 65 | u8 | `frame_count` | número de frames usados em `frames`; 0 = item sem animação |
| 66 | u8 | `frame_interval` | intervalo entre frames (unidade não estabelecida) |
| 67 | u8 | `frame_start` | atraso antes de iniciar (unidade não estabelecida) |

Exemplo: item 109 tem `frames = -3, -2, -1, 0, 1, 2` e `frame_count = 6`, percorrendo os itens 106 a 111. O leitor guarda apenas os registros com `frame_count > 0` (766 no arquivo de referência).

Campos não interpretados: `unk`, unidade de `frame_interval` e `frame_start`.

Leitor no Fiddler: `crates/uo_core/src/animdata.rs` (`AnimDataManager::from_file`)

## animinfo.mul

4000 bytes: 2000 valores `u16`, todos iguais a `0x0204` no arquivo de referência.

Campos não interpretados: todo o arquivo.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## chardata.mul

20 bytes: o texto ASCII ` ver 1` seguido de LF (7 bytes) e 13 bytes de dados: `b2 02 b2 02 b2 02 b2 02 b2 02 34 00 25` (cinco `u16` iguais a `0x02B2`, um `u16` `0x0034` e um byte `0x25`).

Campos não interpretados: os 13 bytes de dados.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## palette.mul

768 bytes: 256 entradas de 3 bytes (valores 0 a 255). A ordem dos canais de cor não foi estabelecida.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## fonts.mul (fontes ASCII)

Propósito: 10 fontes bitmap de 224 caracteres (códigos `0x20` a `0xFF`), usadas em textos do cliente.

Estrutura: 10 fontes em sequência, sem cabeçalho de arquivo, em 884766 bytes (o arquivo de referência tem 143 bytes adicionais após a fonte 9, não interpretados).

Fonte:

| Tipo | Campo | Significado |
|---|---|---|
| u8 | `header` | 1 em todas as fontes de referência |
| glifo[224] | `glyphs` | glifos para os códigos `0x20 + i` |

Glifo:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u8 | `width` | largura em pixels |
| 1 | u8 | `height` | altura em pixels |
| 2 | u8 | `unk` | 3, 4 ou 5 (não interpretado) |
| 3 | u16[width*height] | `pixels` | cores de 16 bits, linha a linha; 0 = transparente |

Glifos com `width = 0` ou `height = 0` não têm pixels (e `pixels` não ocupa bytes). Na cópia de referência todos os 224 glifos de todas as fontes têm dimensões não nulas. Alturas máximas dos glifos `0x20` a `0x7F`: 21 (fontes 0, 2, 3, 4, 6, 7, 8), 15 (fontes 1 e 9) e 25 (fonte 5).

Campos não interpretados: `unk` de cada glifo e os bytes finais do arquivo.

Leitor no Fiddler: `crates/uo_core/src/fonts.rs` (`FontsManager::from_file`, `AsciiFont`)

## unifont*.mul (fontes Unicode)

Propósito: 13 fontes Unicode em bitmap de 1 bit por pixel.

Arquivos: `unifont.mul` (fonte 0) e `unifont1.mul` a `unifont12.mul`.

Estrutura:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u32[0x10000] | `offsets` | posição do glifo de cada código Unicode (BMP) no arquivo; 0 = glifo ausente |
| 0x40000 | glifos | dados | glifos em sequência |

Glifo na posição `offsets[c]`:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | i8 | `x_offset` | deslocamento horizontal do bitmap em relação à posição do cursor |
| 1 | i8 | `y_offset` | deslocamento vertical do bitmap, medido a partir do topo da linha de texto |
| 2 | u8 | `width` | largura do bitmap |
| 3 | u8 | `height` | altura do bitmap |
| 4 | u8[height * ceil(width / 8)] | `bits` | `height` linhas de `ceil(width / 8)` bytes; 1 bit por pixel, bit mais significativo primeiro; 1 = pixel preenchido |

Notas:

- A linha de base das letras latinas fica em torno de `y = 14` (a parte inferior de `I` está em `y_offset + height = 14`; descendentes como `g` chegam a 16).
- Glifos ausentes (`offsets[c] = 0`) não são desenhados. O espaço (`0x20`) é ausente nas fontes de referência; o leitor usa avanço fixo de 8 pixels para o espaço.
- O arquivo não guarda o avanço horizontal. O Fiddler usa `max(x_offset, 0) + width + 1` pixels como avanço de cada glifo e a altura de linha é a base do conjunto `0x21` a `0x7E` mais 2.
- Os arquivos `unifont.mul` a `unifont2.mul` têm 36347 a 36349 glifos; `unifont3.mul` a `unifont12.mul` têm 65472. Nenhum offset vale `0xFFFFFFFF`. O leitor descarta glifos cuja área de bits ultrapassa o fim do arquivo.

Leitor no Fiddler: `crates/uo_core/src/fonts.rs` (`UnicodeFont`)
