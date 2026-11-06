# Animações (anim.mul e anim2 a anim5)

As animações de criaturas, humanos e equipamentos ficam em cinco pares `.idx`/`.mul`: `anim.mul` e `anim2.mul` a `anim5.mul`. Cada par usa o índice genérico de 12 bytes descrito em [index-mul.md](index-mul.md). O número da entrada do índice é calculado a partir do corpo (body), da ação e da direção. Little-endian em todos os campos.

Os arquivos `AnimationFrame*.uop` e `AnimationSequence.uop` (formato UOP) não são tratados aqui.

## Arquivos e índices

| Arquivo (índice / dados) | Entradas no `.idx` | Entradas válidas | Tamanho do `.mul` |
|---|---|---|---|
| `anim.idx` / `anim.mul` | 148810 | 35703 | 194950053 |
| `anim2.idx` / `anim2.mul` | 22455 | 5575 | 138407878 |
| `anim3.idx` / `anim3.mul` | 129850 | 3455 | 26107085 |
| `anim4.idx` / `anim4.mul` | 84700 | 20036 | 60543047 |
| `anim5.idx` / `anim5.mul` | 79275 | 11812 | 91405236 |

Entrada válida: `lookup >= 0` e `length > 0`; `extra` é sempre 0. Entradas inválidas são `(-1, -1, -1)` ou têm `length = 0`.

Cada animação (corpo, ação) ocupa 5 entradas consecutivas, uma por direção armazenada (0 a 4). O índice da entrada é:

```
indice = base(arquivo, corpo) + acao * 5 + direcao_armazenada
```

### Base por arquivo

O número de entradas reservadas a um corpo depende da faixa do corpo: 110 (22 ações), 65 (13 ações) ou 175 (35 ações).

| Arquivo | Corpo | `base(corpo)` | Entradas por corpo |
|---|---|---|---|
| `anim.mul` e `anim4.mul` | 0 a 199 | `corpo * 110` | 110 |
| | 200 a 399 | `22000 + (corpo - 200) * 65` | 65 |
| | 400 em diante | `35000 + (corpo - 400) * 175` | 175 |
| `anim2.mul` | 0 a 199 | `corpo * 110` | 110 |
| | 200 em diante | `22000 + (corpo - 200) * 65` | 65 |
| `anim3.mul` | 0 a 299 | `corpo * 65` | 65 |
| | 300 a 399 | `33000 + (corpo - 300) * 110` | 110 |
| | 400 em diante | `35000 + (corpo - 400) * 175` | 175 |
| `anim5.mul` | 0 a 199, exceto 34 | `corpo * 110` | 110 |
| | 200 a 399 e o corpo 34 | `22000 + (corpo - 200) * 65` | 65 |
| | 400 em diante | `35000 + (corpo - 400) * 175` | 175 |

Notas:

- No `anim5.mul` o corpo 34 segue a fórmula da faixa 200 a 399 (`22000 + (34 - 200) * 65`, índices 11210 a 11274, contíguos e válidos no arquivo de referência).
- No `anim3.mul` as faixas 300 a 399 e 400 em diante se sobrepõem a partir do índice 35000; a faixa é escolhida apenas pelo número do corpo, na ordem da tabela.
- Em `anim.mul` os corpos 0 a 199 têm 22 ações no layout, mas cada corpo usa só algumas (por exemplo, o corpo 12 tem ações 0 a 6, 10 a 12 e 15 a 21 preenchidas).

Leitor no Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::base_index`, `index`)

## Entrada de animação (frames de uma direção)

O bloco apontado por uma entrada do índice contém todos os frames de uma (corpo, ação, direção armazenada).

Cabeçalho:

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | u16[256] | `palette` | paleta de 256 cores de 16 bits (bit 15 em 0; o leitor define o bit 15) |
| 512 | u32 | `frame_count` | número de frames (o leitor aceita de 1 a 100; valores observados até 15) |
| 516 | u32[frame_count] | `frame_offset` | posição de cada frame, contada a partir do offset 512 do bloco (`posicao = 512 + frame_offset`) |

Um `frame_offset` igual a 0 marca um frame ausente. Entradas com menos de 516 bytes ou com `frame_count` fora de 1 a 100 devem ser descartadas.

Frame (na posição `512 + frame_offset`):

| Offset | Tipo | Campo | Significado |
|---|---|---|---|
| 0 | i16 | `center_x` | coluna da imagem onde fica o ponto de ancoragem horizontal |
| 2 | i16 | `center_y` | deslocamento vertical do ponto de ancoragem (normalmente negativo) |
| 4 | u16 | `width` | largura (1 a 1024) |
| 6 | u16 | `height` | altura (1 a 1024) |
| 8 | u32[] | `runs` | corridas até o terminador |

Cada corrida é um `u32` de cabeçalho seguido de `run` bytes, cada um um índice na paleta (índice no `palette` do bloco):

- O frame termina quando o `u32` lido vale `0x7FFF7FFF` (comparado antes do XOR abaixo).
- Para qualquer outro valor: `h = valor XOR 0x80200000`. A constante é `(0x200 << 22) | (0x200 << 12)`.
- Campos de `h`: `x = (h >> 22) & 0x3FF`, `y = (h >> 12) & 0x3FF`, `run = h & 0xFFF`.
- Posição do primeiro pixel da corrida: coluna `(center_x - 0x200) + x`, linha `(center_y + height - 0x200) + y`. Os `run` pixels seguintes ocupam colunas consecutivas na mesma linha. O `XOR` transforma o deslocamento com sinal de 10 bits em um valor com bias `0x200`; o deslocamento efetivo é `dx = x - 0x200` e `dy = y - 0x200`.
- Pixels fora da imagem são descartados. Pixels não cobertos por corrida são transparentes.

Ponto de ancoragem: o pixel de deslocamento (0, 0) cai na coluna `center_x` e na linha `height + center_y` da imagem. Para desenhar, o canto superior esquerdo da imagem vai em `ancora - (center_x, height + center_y)`.

Exemplo (`anim.mul`, corpo 400, ação 0, direção 0): `frame_count = 10`, primeiro frame com `center_x = 12`, `center_y = -11`, `width = 24`, `height = 64`.

Nas amostras de cada um dos 5 arquivos de referência todos os frames decodificam com pixels dentro da imagem e terminam no marcador `0x7FFF7FFF`.

Leitor no Fiddler: `crates/uo_core/src/animations.rs` (`decode_animation_mul`)

## Direções

Cada animação armazena 5 direções (0 a 4). As direções 5 a 7 são o espelhamento horizontal das armazenadas:

| Direção | Rótulo no Fiddler | Entrada lida | Espelhada |
|---|---|---|---|
| 0 | South | 0 | não |
| 1 | South-West | 1 | não |
| 2 | West | 2 | não |
| 3 | North-West | 3 | não |
| 4 | North | 4 | não |
| 5 | North-East | 3 | sim |
| 6 | East | 2 | sim |
| 7 | South-East | 1 | sim |

Regra: para `direcao > 4`, a entrada lida é `8 - direcao` e o frame é espelhado horizontalmente (coluna `x` vira `width - 1 - x`); o `center_x` do frame espelhado é `width - center_x`.

## Ações por tipo de animação

O número da ação é o segundo fator do índice (`acao * 5`). A quantidade de ações reservada a um corpo depende da faixa do corpo (seção "Base por arquivo"), não do tipo. O tipo de animação (monstro, marítimo, animal, humano) define o nome de cada número no Fiddler. O tipo vem de `mobtypes.txt` (ou de `Animationlist.xml`), e os identificadores numéricos usados pelo Fiddler são 0 = monstro, 1 = marítimo, 2 = animal, 3 = humano/equipamento.

### Monstro (tipo 0)

| Ação | Nome no Fiddler |
|---|---|
| 0 | Walk |
| 1 | Stand / Idle |
| 2 | Die 1 |
| 3 | Die 2 |
| 4 | Attack 1 |
| 5 | Attack 2 |
| 6 | Attack 3 |
| 7 | Cast / Special 1 |
| 8 | Cast / Special 2 |
| 9 | Get Hit |
| 10 | Block Right |
| 11 | Block Left |
| 12 | Idle |
| 13 | Fidget |
| 14 | Fly |
| 15 | Take Off |
| 16 | Get Hit In Air |

O layout de monstro em `anim.mul` reserva 22 ações; os números 17 a 21 existem no índice, mas o Fiddler não os nomeia. No corpo 12 (dragão) as ações 0 e 1 têm 10 e 1 frames.

### Animal (tipo 2)

Layout com 13 ações (65 entradas).

| Ação | Nome no Fiddler | Frames (corpo 200, dir. 0) |
|---|---|---|
| 0 | Walk | 5 |
| 1 | Run | 5 |
| 2 | Idle 1 | 1 |
| 3 | Eat | 5 |
| 4 | Alert | 3 |
| 5 | Attack 1 | 5 |
| 6 | Attack 2 | 5 |
| 7 | Get Hit | 5 |
| 8 | Die 1 | 6 |
| 9 | Idle 2 | 5 |
| 10 | Fidget | 3 |
| 11 | Lie Down | 6 |
| 12 | Die 2 | 6 |

### Marítimo (tipo 1)

| Ação | Nome no Fiddler |
|---|---|
| 0 | Walk |
| 1 | Run |
| 2 | Idle 1 |
| 3 | Idle 2 |
| 4 | Fidget |
| 5 | Attack 1 |
| 6 | Attack 2 |
| 7 | Get Hit |
| 8 | Die 1 |

### Humano e equipamento (tipo 3)

Layout com 35 ações (175 entradas), ações 0 a 34, usado pelos corpos 400 em diante em `anim.mul`, `anim3.mul`, `anim4.mul` e `anim5.mul`. Os corpos 400 e 401 do arquivo de referência têm as 35 ações com 5 direções.

| Ação | Rótulo no Fiddler | Frames (corpo 400, dir. 0) |
|---|---|---|
| 0 | Walk_01 | 10 |
| 1 | WalkStaff_01 | 10 |
| 2 | Run_01 | 10 |
| 3 | RunStaff_01 | 10 |
| 4 | Idle_01 | 1 |
| 5 | Idle_02 | 5 |
| 6 | Fidget_Yawn_Stretch_01 | 5 |
| 7 | CombatIdle1H_01 | 1 |
| 8 | CombatIdle2H_01 | 1 |
| 9 | AttackSlash1H_01 | 7 |
| 10 | AttackPierce1H_01 | 7 |
| 11 | AttackBash1H_01 | 7 |
| 12 | AttackBash2H_01 | 7 |
| 13 | AttackSlash2H_01 | 7 |
| 14 | AttackPierce2H_01 | 7 |
| 15 | CombatAdvance_1H_01 | 10 |
| 16 | Spell1 | 7 |
| 17 | Spell2 | 7 |
| 18 | AttackBow_01 | 7 |
| 19 | AttackCrossbow_01 | 7 |
| 20 | GetHit_Fr_Hi_01 | 5 |
| 21 | GetHit_Fr_Med_01 | 6 |
| 22 | GetHit_Fr_Low_01 | 6 |
| 23 | Block_Shield_01 | 5 |
| 24 | Punch_01 | 5 |
| 25 | Bow_01 | 1 |
| 26 | Salute_01 | 5 |
| 27 | Ingest_Eat_01 | 5 |
| 28 | Horse_Walk_01 | 7 |
| 29 | Horse_Run_01 | 5 |
| 30 | Horse_Idle_01 | 5 |
| 31 | Horse_Attack1H_01 | 7 |
| 32 | Horse_AttackBow_01 | 5 |
| 33 | Horse_AttackCrossbow_01 | 5 |
| 34 | Horse_Spell2_01 | 5 |

A tabela do Fiddler lista ainda as ações 35 (`Die_Hard_Fwd_01`) e 36 (`Die_Hard_Back_01`), que ficam fora das 35 ações do layout de 175 entradas.

Campos não interpretados: os rótulos das ações humanas são os nomes mostrados pelo Fiddler; no arquivo só existem o número da ação e a contagem de frames. As contagens de frames da coluna à direita servem para conferir a numeração e não vêm do Fiddler.

Leitor no Fiddler: `crates/uo_gui/src/main.rs` (`ACTIONS_MONSTER`, `ACTIONS_ANIMAL`, `ACTIONS_SEA`, `ACTIONS_HUMAN`, `DIRECTIONS`)

## Resolução de corpo para arquivo

Dado um corpo pedido, o cliente (e o Fiddler) decide em qual arquivo e com qual número de corpo a animação está, usando `Body.def` e `Bodyconv.def`. Ambos são texto; a sintaxe geral está em [def-files.md](def-files.md).

### Body.def

Linhas `<CORPO_ORIGINAL> {<CORPO_NOVO>[, ...]} <MATIZ>`. O corpo é substituído pelo primeiro valor entre chaves e o matiz da linha acompanha o resultado. Em chave duplicada vale a primeira ocorrência.

```
11 {28} 1401
46 {12, 59} 1106
```

### Bodyconv.def

Linhas com cinco colunas numéricas separadas por tab ou espaço: `<CORPO> <anim2> <anim3> <anim4> <anim5>`. Cada coluna é o número do corpo dentro do arquivo `animN.mul` correspondente, ou `-1` se o corpo não está nesse arquivo. Vale a primeira coluna (na ordem anim2, anim3, anim4, anim5) com valor não negativo. Sem linha, ou com todas as colunas `-1`, o corpo fica em `anim.mul`.

```
157	1	-1	-1	-1
```

### Algoritmo (ordem do Fiddler)

1. Aplicar `Body.def`: `(corpo, matiz) = body_def[corpo]`, se existir.
2. Aplicar `Bodyconv.def`: se existir linha, `(arquivo, corpo) = (anim2..anim5, valor da coluna)`; senão `arquivo = anim.mul`.
3. Calcular o índice com a fórmula da seção "Base por arquivo" para `(arquivo, corpo, acao, direcao)` e ler o bloco.
4. Se o bloco não existir ou não decodificar, tentar a animação em UOP.
5. Se ainda assim não existir, tentar o corpo original (sem conversão) em cada um de `anim.mul` a `anim5.mul`, na ordem.

Leitor no Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::parse`, `resolve`, `index`, `load_animation_frames`)

### Corpse.def

Mesmo formato do `Body.def`. Aplica-se ao corpo usado para desenhar cadáveres.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

### Anim1.def a Anim4.def

Tabelas de texto no formato `<ORIG> {<NOVO>} <MATIZ>` (ver [def-files.md](def-files.md)). `Anim1.def` tem 9 linhas (13 a 21); `Anim2.def` tem corpos 35 a 271 com destinos como `{23}` ou `{-1}`; `Anim3.def` e `Anim4.def` estão vazios.

Campos não interpretados: significado dos números (corpo ou ação) nestes arquivos.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## mobtypes.txt

Propósito: tipo de animação de cada corpo.

Texto, uma linha por corpo; linhas iniciadas por `#` são comentário e linhas sem número no início são ignoradas. Campos separados por tabs/espaços:

| Campo | Significado |
|---|---|
| `ID` | corpo (decimal) |
| `TYPE` | `MONSTER`, `SEA_MONSTER`, `ANIMAL`, `HUMAN` ou `EQUIPMENT` |
| `FLAGS` | hexadecimal; o Fiddler interpreta apenas o bit `0x10000` (corpo animado pelos arquivos UOP) |

Texto após `#` na mesma linha é comentário. O arquivo de referência tem 1210 linhas (com comentários e linhas em branco); por tipo: 297 `MONSTER`, 97 `ANIMAL`, 25 `HUMAN`, 4 `SEA_MONSTER` e 732 `EQUIPMENT`. Algumas linhas têm um campo vazio extra (tab duplo) ou uma coluna numérica a mais antes do comentário.

Conversão de `TYPE` para o tipo de animação do Fiddler: `MONSTER` = 0, `SEA_MONSTER` = 1, `ANIMAL` = 2, `HUMAN` e `EQUIPMENT` = 3 (`EQUIPMENT` também marca o corpo como equipamento).

Campos não interpretados: os demais bits de `FLAGS` (por exemplo `1008`, `2A`, `20000`).

Leitor no Fiddler: `crates/uo_core/src/animations.rs` (`parse_mobtypes_txt`, `load_animation_defs`)
