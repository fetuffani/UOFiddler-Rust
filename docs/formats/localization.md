# Localização do cliente

## 1. Contêiner IFF de strings localizadas

### Finalidade e arquivos

Contêiner no estilo IFF (chunks com tamanho big-endian) que guarda uma lista ordenada de strings de interface em um idioma. O nome do arquivo é `<recurso>.<idioma>`; maiúsculas e minúsculas variam (`Tooltips.enu`, `INTLOC02.ENU`, `Intloc02.cht`).

Idioma, sufixo do arquivo e código gravado no chunk `INFO`:

| Sufixo | Idioma | Código no `INFO` |
|---|---|---|
| `enu` | inglês (EUA) | `enu` |
| `deu` | alemão | `deu` |
| `esp` | espanhol | `esp` |
| `fra` | francês | `fra` |
| `jpn` | japonês | `jpn` (em `Intro.jpn`: `Jpn`) |
| `kor` | coreano | `kor` |
| `cht` | chinês tradicional | `cht` (em `Gesture.cht`: `bg5`) |
| `chs` | chinês simplificado | usado apenas por `Cliloc.chs` (que não é IFF) |

O código do `INFO` não deve ser usado para escolher a codificação; a largura de caractere está no próprio `INFO` (abaixo).

Famílias de recursos presentes (nome do recurso no `INFO`, quantidade de strings na versão `enu`, idiomas presentes):

| Arquivos | Recurso | Strings | Conteúdo |
|---|---|---|---|
| `Tooltips.*` | `tooltips` | 61 | dicas dos botões da tela inicial (`Visit www.uo.com`, `View online help`, `Previous screen`) |
| `Gesture.*` | `gesture` | 272 | nomes de gestos/emotes; o índice da string é o número do gesto (32 = `bow`, 33 = `salute`); a maioria vazia |
| `Chat.*` | `chat` | 105 | textos da janela de chat (`Create a New Conference`, `Ignore a Player`) |
| `Professn.*` | `professn` | 57 | nomes e descrições das profissões (`ADVANCED`, `ADVENTURER`, `MERCHANT`...) |
| `Skilname.*`, `SKILNAME.*` | `skilname` | 52 em `enu` (49 nos demais) | nomes das skills, na ordem dos IDs de skill (índice 0 = `Alchemy`); o último item é vazio |
| `SKILL16.*` | `skill16` | 9 | texto de ajuda de uma skill, com chunks `POSN` e `COLR` |
| `SKILL30.*` | `skill30` | 4 | idem, com `POSN` e `COLR` |
| `Tilehelp.*` | `tilehelp` | 95 (90 em `jpn`) | textos de ajuda por grupo de tiles, com chunk `TKEY` |
| `Intloc00.*`, `intloc00.enu` | `intloc00` | 620 | mensagens de login e conexão (`Quit Ultima Online?`, `Connecting...`) |
| `Intloc01.*` | `intloc01` | 100 | rótulos do chat (`Participants`, `Whisper`, `WhoIs`) |
| `Intloc02.*`, `INTLOC02.*` | `intloc02` | 150 | `You see:`, criação e exclusão de personagem |
| `INTLOC06.*` | `intloc06` | 200 | botões de treino de skill (`Train Alchemy`...) |
| `INTLOC10.*` | `intloc10` | 210 | interface do Codex of Wisdom (`Context Help`, `Index`, `History`) |
| `intloc11.enu` | `intloc11` | 33 | palavras de emote (`yawn`, `faint`, `bow`, `salute`, `applaud`) |
| `Gt_0000`, `Gt_1010`, `Gt_2000`, `Gt_2310`, `Gt_2400`, `Gt_4000`, `Gt_5000`, `Gt_5400` (`.enu` etc.) | `gt_NNNN` | 55, 303, 233, 15, 130, 208, 208, 21 | textos de gumps do cliente; na versão `enu` quase todas vazias (somente `Gt_4000.enu` tem 32 strings, ex.: `Read Mail`, `Address List`) |
| `Intro.*` | `intro` | 28 (27 em `deu`) | texto da introdução, com chunks `TIME` e `COLR` |
| `OPTNUOTD.*`, `optuotd.JPN` | `optnuotd` | 208 (212 `enu`, 194 `jpn`, 196 `kor`) | janela de opções e dica do dia; `optuotd.JPN` é idêntico a `OPTNUOTD.JPN` |
| `options.*` | `options` | 220 a 222 | janela de opções (em `options.deu` o nome do recurso é `Optionen`) |

Idiomas por família: `Tooltips`, `Chat`, `Gesture`, `Professn`, `Tilehelp`, `Intro`, `Gt_*`, `OPTNUOTD`, `options`: todos os sete (`cht deu enu esp fra jpn kor`); `Intloc00` e `Intloc01`: sete (o inglês de `Intloc00` é `intloc00.enu`); `Intloc02`: `cht deu esp fra` mais `INTLOC02.ENU/JPN/KOR`; `Skilname`: `cht deu esp fra` mais `SKILNAME.ENU/JPN/KOR`; `SKILL16`, `SKILL30`, `INTLOC06`, `INTLOC10`: `CHT ENU JPN KOR`; `intloc11`: apenas `enu`.

### Layout binário

Todos os inteiros de tamanho de chunk são big-endian; os inteiros dentro do `INFO` e dos chunks auxiliares são little-endian. Cada chunk tem payload com preenchimento de 1 byte NUL quando o tamanho é ímpar (o byte de preenchimento não entra no tamanho declarado).

| Offset | Tamanho | Campo |
|---|---|---|
| 0 | 4 | `FORM` |
| 4 | 4 | tamanho (BE) = tamanho do arquivo - 8 |
| 8 | 4 | tipo de forma `DATA` |
| 12 | 4 | `FORM` |
| 16 | 4 | tamanho (BE) = tamanho do arquivo - 20 |
| 20 | 4 | tipo de forma `LANG` |
| 24 | 4 | id de chunk `INFO` (os 8 bytes de 20 a 27 formam o texto `LANGINFO`) |
| 28 | 4 | tamanho do `INFO` (BE) |
| 32 | N | payload do `INFO` (abaixo), mais 1 byte de preenchimento se N for ímpar |
| depois | 4 + 4 + M | chunk `TEXT`: id, tamanho M (BE), strings, preenchimento se M for ímpar |
| depois | | chunks opcionais (`POSN`, `COLR`, `TIME`, `TKEY`) |

Payload do `INFO` (tamanho = 21 + comprimento do nome do recurso):

| Campo | Tipo | Descrição |
|---|---|---|
| código do idioma | ASCII + NUL | `enu`, `deu`... |
| largura | `u32` | bytes por caractere das strings: `1` (8 bits) ou `2` (UTF-16LE) |
| flag | `u32` | `0`; `1` somente nos `Tilehelp.*` |
| nome do recurso | ASCII + NUL | `tooltips`, `intloc00`... |
| constante | `u32` | `1` em todos os arquivos |
| contagem | `u32` | quantidade de strings do `TEXT` |

Payload do `TEXT`: as strings em ordem, cada uma terminada por NUL do tamanho da largura (1 byte ou 2 bytes `00 00`); strings vazias são terminadores consecutivos; a última string também é terminada. A contagem do `INFO` é igual ao número de terminadores. A string é referenciada pelo seu índice (base 0).

Codificação conforme a largura:

| Largura | Idiomas observados | Codificação |
|---|---|---|
| 1 | `enu`, `deu`, `esp`, `fra` e `Gt_4000.jpn` | ASCII / Windows-1252 (Latin-1 para os acentos) |
| 2 | `jpn`, `kor`, `cht` | UTF-16 little-endian, sem BOM |

Todos os arquivos `jpn`, `kor` e `cht` têm largura 2, com a única exceção de `Gt_4000.jpn` (largura 1, texto em inglês); todos os arquivos `enu`, `deu`, `esp` e `fra` têm largura 1. Os sete `Tilehelp.*` têm flag 1.

### Chunks opcionais

Quantidades por string da lista (`n` = contagem do `INFO`):

| Chunk | Arquivos | Tamanho | Conteúdo |
|---|---|---|---|
| `POSN` | `SKILL16.*` (72 bytes, n=9), `SKILL30.*` (32 bytes, n=4) | `n * 8` | pares `u32` (x, y) de posição de cada string |
| `COLR` | `SKILL16.*` (108), `SKILL30.*` (48), `Intro.jpn` (336) | `n * 12` | três `u32` por string, valores 0 a 255 (componentes de cor) |
| `TIME` | todos os `Intro.*` (224 bytes; 216 em `Intro.deu`, que tem 27 strings) | `n * 8` | dois `u32` por string (início e fim), crescentes; ex. `Intro.enu`: 680 750, 760 840, 850 945... |
| `TKEY` | `Tilehelp.*` | variável | registros consecutivos: `u32` índice da string (0, 1, 2...), `u32` quantidade K, K x `u32` IDs de item estático associados à string. `Tilehelp.enu` tem 95 registros em 5500 bytes |

Unidade de `TIME` e significado das cores não interpretados.

### Marcações dentro das strings de opções

`OPTNUOTD.*` e `options.*` misturam texto e diretivas na lista: `##`, `#TAB0`, `#CHECKBOX%Sound`, `#SLIDER[0-255]%SoundVol`; a string anterior à diretiva é o rótulo do controle.

### Exemplo (`Tooltips.enu`, 1448 bytes)

```
46 4F 52 4D 00 00 05 A0  "FORM" size=0x5A0
44 41 54 41              "DATA"
46 4F 52 4D 00 00 05 94  "FORM" size=0x594
4C 41 4E 47              "LANG"
49 4E 46 4F 00 00 00 1D  "INFO" size=29
65 6E 75 00              "enu\0"
01 00 00 00              largura = 1
00 00 00 00              flag = 0
74 6F 6F 6C 74 69 70 73 00  "tooltips\0"
01 00 00 00              constante = 1
3D 00 00 00              contagem = 61
00                       preenchimento
54 45 58 54 00 00 05 61  "TEXT" size=0x561
"Visit www.uo.com\0Change account and credit card information\0..."
```

Leitor no Fiddler: `crates/uo_core/src/client_files.rs` (`parse_iff_text`; lê o `INFO` e a largura, decodifica Windows-1252 ou UTF-16LE, limita à contagem declarada; os chunks `POSN`, `COLR`, `TIME` e `TKEY` não são interpretados)

## 2. Langcode.iff

Tabela de códigos de idioma do Windows, em IFF. 4612 bytes.

Estrutura:

```
FORM <tam> DATA
  FORM <tam> LANG
    FORM <tam> WIN3
      CODE <tam> <payload>     (135 chunks)
```

Os tamanhos de `FORM` seguem a mesma regra dos demais arquivos (4604, 4592 e 4580 no arquivo presente). Cada chunk `CODE` (id `CODE`, tamanho BE, payload com preenchimento par) contém:

| Campo | Tipo | Descrição |
|---|---|---|
| código | 3 bytes ASCII + NUL | código do idioma/região (`ARA`, `ENU`, `DEU`, `ESP`, `FRA`, `JPN`, `KOR`, `CHT`, `CHS`...) |
| valor | `u32` little-endian | 80 chunks com `63` (`?`); demais com valores como 101 (`ENU`), 102 (`FRA`), 103 (`DEU`), 99 (`CHT`, `CHS`), 106 (`JPN`), 115 (`ESP`) |
| idioma | ASCII + NUL | nome do idioma em inglês (`Arabic`, `English`, `Korean (Extended Wansung)`) |
| região | ASCII + NUL | nome da região (`Saudi Arabia`, `United States`, `Japan`, `PRC`) |

Os 135 registros têm 133 códigos distintos.

Exemplos:

```
ARA  63   Arabic    Saudi Arabia
ENU  101  English   United States
DEU  103  German    Germany
JPN  106  Japanese  Japan
KOR  63   Korean (Extended Wansung)  Korea
CHT  99   Chinese   Taiwan
CHS  99   Chinese   PRC
```

Campos não interpretados: significado do campo `valor`.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## 3. Cliloc.*

### Finalidade e arquivos

Tabela de textos numerados (client localization) usada por tooltips, gumps, ajuda e profissões. Um arquivo por idioma: `Cliloc.enu`, `Cliloc.deu`, `Cliloc.esp`, `Cliloc.fra`, `Cliloc.jpn`, `Cliloc.kor`, `Cliloc.cht`, `Cliloc.chs`. Os sufixos são os da seção 1.

| Arquivo | Entradas | Menor / maior número |
|---|---|---|
| Cliloc.enu | 110292 | 500000 / 3011032 |
| Cliloc.jpn | 47526 | 500000 / 3011032 |
| Cliloc.cht | 42122 | 500000 / 3011032 |
| Cliloc.kor | 33193 | 500000 / 3011032 |
| Cliloc.deu | 18628 | 500001 / 3010161 |
| Cliloc.esp | 18247 | 500000 / 3010161 |
| Cliloc.fra | 18191 | 500000 / 3010161 |
| Cliloc.chs | 453 | 500012 / 3010161 |

Os arquivos de outros idiomas são subconjuntos; um número ausente deve ser buscado em `Cliloc.enu`.

### Layout binário (little-endian)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | `u32` | `2` (cabeçalho) |
| 4 | `u16` | `1` (cabeçalho) |
| 6 | | sequência de entradas até o fim do arquivo |

Entrada:

| Campo | Tipo | Descrição |
|---|---|---|
| número | `u32` | número do cliloc |
| flag | `u8` | `0` em todas as entradas dos arquivos presentes |
| comprimento | `u16` | tamanho do texto em bytes |
| texto | `comprimento` bytes | UTF-8, sem NUL final |

As entradas estão em ordem crescente de número e o arquivo termina exatamente após a última. Textos vazios existem (60864 em `Cliloc.enu`). Os arquivos presentes não são comprimidos; o Fiddler trata como comprimido (compressão Mythic) um arquivo cujos 4 primeiros bytes são todos diferentes de zero.

### Conteúdo do texto

- Marcadores de argumento `~N_nome~`, onde N (a partir de 1) é o índice do argumento substituído pelo chamador: `~1_name~ misses the target altogether.` (500604), `~1_VAL~` (1114778). Há 1570 textos com marcadores em `Cliloc.enu`.
- Alguns textos contêm HTML de gump (`<BASEFONT COLOR=#DFDFC0>`, `<DIV ALIGN=RIGHT>`, `<a href = "?165">`).

### Uso do número de cliloc

| Onde | Como |
|---|---|
| Gumps (`xmfhtmlgump x y w h cliloc background scrollbar`, `AddHtmlLocalized`) | o texto do elemento HTML vem do cliloc em vez de uma string do gump (no editor de gumps do Fiddler, o valor 0 equivale a usar texto literal) |
| Tooltips de itens | o servidor envia o número e os argumentos; o cliente monta o texto com os marcadores `~N_nome~` |
| `Prof.txt` | `NameId` e `DescId` (ex.: 1062948 = `SAMURAI`) |
| `Helpindx.dat` | três números por tópico (ex.: 3010031 = `Paperdoll`) |
| `walls.txt`, `roof.txt`, `misc.txt` | coluna `TID` (ex.: 1060054 = `Dark Wood Walls`) |
| `Intrface.def` | `settooltip`, `tooltip` (ex.: 1062709) |

Exemplo (`Cliloc.enu`):

```
1062948  SAMURAI
3010030  Movement
1060054  Dark Wood Walls
500000   Reputation aversion triggered.
```

Leitor no Fiddler: `crates/uo_core/src/cliloc.rs` (`ClilocManager::from_file`); mythic em `crates/uo_core/src/mythic.rs`; uso em gumps em `crates/uo_gui/src/gumpstudio_tab.rs`

## 4. speech.mul

### Finalidade

Lista de palavras-chave de fala reconhecidas pelo cliente, cada uma associada a um identificador numérico. Arquivo único, 119714 bytes, 6125 entradas, sem cabeçalho.

### Layout (big-endian)

Entradas consecutivas até o fim do arquivo:

| Campo | Tipo | Descrição |
|---|---|---|
| id | `i16` BE | identificador da palavra-chave (0 a 512; 513 valores distintos; vários registros podem ter o mesmo id) |
| comprimento | `i16` BE | tamanho do texto em bytes (máximo 65 no arquivo presente) |
| texto | `comprimento` bytes | UTF-8, sem NUL final |

Observações:

- Os textos aparecem em vários idiomas, com asteriscos delimitando o termo em muitas entradas (`*barrier*`, `*バリア*`; 4988 entradas contêm `*`).
- Existem entradas de comprimento zero.
- Ex.: id 0 repete-se para `*提領*`; id 512 agrupa `*barrier*`, `*バリア*` e `ばりあ*`.

Exemplo (primeiros bytes):

```
00 00 00 08 2A E6 8F 90 E9 A0 98 2A   id=0 len=8 "*提領*"
00 01 00 08 2A E7 B5 90 E5 AD 98 2A   id=1 len=8 "*結存*"
```

Leitor no Fiddler: `crates/uo_core/src/speech.rs` (`SpeechManager::from_file`; interrompe a leitura se um comprimento passar de 256)
