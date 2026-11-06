# Arquivos .def do cliente

Tabelas de texto que redirecionam identificadores (corpo de animação, gump, som, arte) ou descrevem recursos de interface. Todas ficam no diretório do cliente. Exceto onde indicado, são texto ASCII com fim de linha CRLF e sem BOM.

## Sintaxe comum

Formato de linha de redirecionamento (Body.def, Corpse.def, gump.def, Sound.def, Music.def, Anim1.def a Anim4.def, art.def, TexTerr.def):

```
<ID_ORIGINAL> {<ID_NOVO>[, <ID_NOVO2>...]} <MATIZ>
```

| Campo | Descrição |
|---|---|
| `ID_ORIGINAL` | inteiro decimal; chave da linha |
| `{...}` | grupo entre chaves, com um ou mais inteiros separados por vírgula (e espaço opcional). O primeiro valor é o destino principal; os demais são alternativas |
| `MATIZ` | inteiro decimal (hue) aplicado ao resultado; `0` = sem matiz |

Regras de leitura:

- Separadores entre campos são espaços e/ou tabs, em quantidade variável (`2500  {3} 1645`, `2500<TAB>{3}<TAB>1645`).
- `#` inicia comentário até o fim da linha; vale também no meio da linha (`401 866 867 60867 0 # texto`). Linhas começando por `#` são comentários inteiros. Linhas comentadas podem conter entradas desativadas (`#679 {528} 1249`).
- Linhas de comentário longas aparecem entre aspas (`"# texto..."`); devem ser ignoradas como as demais.
- Linhas em branco são ignoradas; pode haver espaços e tabs sobrando no fim da linha.
- Em chave duplicada, Body.def e gump.def usam a primeira ocorrência; Equipconv.def usa a última para a mesma dupla (`bodyType`, `equipmentID`).
- O último registro pode não ter fim de linha.

Exemplo de grupo com vários valores (Body.def):

```
46 {12, 59} 1106
114 {200, 226, 228, 204} 1109
```

## Body.def

Redireciona o corpo de animação original para outro corpo e matiz antes de a animação ser buscada. Cabeçalho do arquivo: `# Format is: <ORIG BODY> {<NEW BODY>} <NEW HUE>`. Contém 237 entradas (IDs de 11 a 1762), das quais 227 com um valor no grupo, 5 com dois, 2 com três e 3 com quatro.

```
11 {28} 1401
38 {9} 0
55 {53, 54} 1154
```

Aplicação: o corpo pedido é substituído pelo primeiro valor do grupo; o matiz da linha (se diferente de 0) passa a ser o matiz do corpo. Depois disso aplica-se o Bodyconv.def. Blocos de comentário delimitam épocas (`# Begin AoS Stuff` / `# End AoS Stuff`).

Leitor no Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::parse`, `resolve`)

## Bodyconv.def

Indica em qual arquivo de animação (`anim2.mul` a `anim5.mul`) o corpo está armazenado e com qual número de corpo dentro dele. Corpos sem linha ficam em `anim.mul`.

Formato (tabs, 5 colunas numéricas):

```
<CORPO> <anim2> <anim3> <anim4> <anim5>
```

| Coluna | Significado |
|---|---|
| 1 | corpo original (após Body.def) |
| 2 | corpo dentro de `anim2.mul` (versão LBR) |
| 3 | corpo dentro de `anim3.mul` (versão AoS) |
| 4 | corpo dentro de `anim4.mul` (versão AoW) |
| 5 | corpo dentro de `anim5.mul` (versão Mondain) |

`-1` significa "não está neste arquivo". Em todas as 547 linhas ativas com destino, exatamente uma coluna é >= 0 (95 na coluna 2, 34 na 3, 131 na 4, 287 na 5); 4 linhas têm as quatro colunas em `-1`. Se mais de uma coluna for >= 0, vale a primeira, na ordem anim2, anim3, anim4, anim5. Comentários finais após a quinta coluna são permitidos (`877 -1 -1 661 -1 # Book 01`). O cabeçalho do arquivo é um bloco de comentários (alguns entre aspas) que descreve as faixas: 0-199 monstros, 200-399 animais, 400+ humanos/elfos/equipamentos; índice máximo 2048.

Exemplos:

```
157	1	-1	-1	-1
942	-1	-1	-1	492
483	-1	-1	-1	345	#	Horns_Gargoyle_Male_Legacy
```

Posição do corpo dentro de cada `animN.idx` (animação 0, direção 0), usada depois da conversão:

| Arquivo | Corpo | Índice base |
|---|---|---|
| anim.mul, anim4.mul | < 200 | `corpo * 110` |
| | 200..399 | `22000 + (corpo - 200) * 65` |
| | >= 400 | `35000 + (corpo - 400) * 175` |
| anim2.mul | < 200 | `corpo * 110` |
| | >= 200 | `22000 + (corpo - 200) * 65` |
| anim3.mul | < 300 | `corpo * 65` |
| | 300..399 | `33000 + (corpo - 300) * 110` |
| | >= 400 | `35000 + (corpo - 400) * 175` |
| anim5.mul | < 200 e diferente de 34 | `corpo * 110` |
| | 200..399 (e 34) | `22000 + (corpo - 200) * 65` |
| | >= 400 | `35000 + (corpo - 400) * 175` |

Índice final = base + `ação * 5` + direção (direções 0 a 4; direções 5 a 7 espelham 3 a 1).

Leitor no Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver`)

## Equipconv.def

Converte o ID de equipamento (animação) para outro conforme o tipo de corpo do portador, podendo trocar o gump do paperdoll e o matiz.

Formato (tabs; comentário `#` ao final da linha):

```
<bodyType> <equipmentID> <convertToID> <GumpIDToUse> <hue>
```

| Campo | Descrição |
|---|---|
| `bodyType` | tipo de corpo do portador. Valores presentes: 401 (humano feminino), 605 (elfo masculino), 606 (elfo feminino), 666 (gárgula masculino), 667 (gárgula feminino), 745 |
| `equipmentID` | ID de equipamento/animação original |
| `convertToID` | ID de equipamento para o qual é convertido |
| `GumpIDToUse` | gump do paperdoll (regra abaixo) |
| `hue` | matiz do equipamento (todas as linhas ativas presentes usam 0) |

Regra de `GumpIDToUse`:

| Valor | Gump resultante |
|---|---|
| `0` | `equipmentID + 50000` |
| `-1` | `convertToID + 50000` |
| outro | o próprio valor é o ID do gump |

Quantidade nas linhas ativas: 48 com `0`, 23 com `-1`, 15 com valor literal.

Regra de gump sem entrada: se não existe linha para (`bodyType`, `equipmentID`), o gump masculino é `equipmentID + 50000`.

Regra feminina (+10000): quando o portador é feminino e o gump resolvido está em 50000..59999, usa-se `gump + 10000` (faixa 60000) desde que esse gump exista no `gumpart`; caso contrário permanece o gump masculino. Um valor literal já na faixa 60000 (ex.: `60867`) não recebe nova soma.

Depois de resolvido, o gump passa pelo gump.def (abaixo): se houver redirecionamento, o gump e (quando diferente de 0) o matiz são substituídos. O matiz final vem do `hue` do Equipconv ou, se o gump.def definir matiz diferente de 0, deste último.

Consulta inversa (gump para animação): primeiro os alvos explícitos do Equipconv; senão 50000..59999 equivale a animação `gump - 50000` masculina e 60000..69999 a `gump - 60000` feminina.

Exemplos:

```
401	866	867	60867	0	#	Helm of swiftness M to F
401	538	986	0	0			# female chain substitution
606	968	977	-1	0			#hide chest
666	865	-1	50885	0			#Male Garge
```

Notas: o cabeçalho é um bloco de comentários (inclui uma linha entre aspas `"#   -GumpID (0 = ...`); linhas iniciadas em `#` seguidas de números são entradas desativadas; blocos são separados por comentários de seção (`#female human`, `# female elf`, `#Gargoyle Talon Swaps`).

Leitor no Fiddler: `crates/uo_core/src/equip_gump.rs` (`EquipGumpMap::parse`, `gump_for_anim`, `anim_for_gump`)

## gump.def

Redireciona um gump (usado apenas no paperdoll) para outro gump e matiz. Mesmo formato dos redirecionamentos (`<GUMP> {<NOVO>} <MATIZ>`), com IDs na faixa 50000 (masculino) e 60000 (feminino). Para equipamentos é preciso declarar as duas faixas, senão o paperdoll feminino usa a arte masculina. Contém 32 entradas.

```
50759	{50468}	0
60759	{60468}	0
60482 {60477} 35
```

Aplicação: após resolver o gump (Equipconv ou `ID + 50000`), se o gump consta no gump.def ele é trocado pelo novo e, se o matiz da linha for diferente de 0, substitui o matiz.

Leitor no Fiddler: `crates/uo_core/src/equip_gump.rs`

## Corpse.def

Mesmo formato e cabeçalho do Body.def (`<ORIG BODY> {<NEW BODY>} <NEW HUE>`), aplicado ao corpo usado ao desenhar cadáveres. Contém 240 chaves distintas (IDs de 11 a 1762); 230 delas também existem no Body.def (220 com destino e matiz idênticos), e há linhas próprias (por exemplo `13 {51} 16385`). Comentários de seção: `#Dragon Armor`, `#Begin AoS Stuff`, `# Male Vampire`.

```
13 {51} 16385
46 {12, 59} 1106
```

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Anim1.def, Anim2.def, Anim3.def, Anim4.def

Tabelas de redirecionamento de corpo no formato `<ORIG> {<NOVO>} <MATIZ>`. Não possuem comentários. O terceiro campo é sempre 0.

| Arquivo | Entradas | IDs | Destino |
|---|---|---|---|
| Anim1.def | 9 | 13 a 21 | sempre >= 0 (ex.: `13 {5} 0`) |
| Anim2.def | 237 | 35 a 271 | 182 com `-1` (ex.: `35 {-1} 0`) e 55 com corpo (ex.: `43 {23} 0`) |
| Anim3.def | 0 | arquivo vazio (0 bytes) | |
| Anim4.def | 0 | arquivo vazio (0 bytes) | |

`-1` indica que não há corpo de destino.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Sound.def

Redireciona um ID de som para outro. Formato `<SOM> {<NOVO>} <MATIZ>`; o terceiro campo é sempre 0. Contém 786 entradas (IDs de 654 a 1440): 435 com destino >= 0 (ex.: `654 {487} 0`) e 351 com `-1` (sem som de destino, ex.: `778 {-1} 0`). Uma linha está sem a chave de fechamento (`805 {331 0`); um leitor deve ignorar ou tolerar linhas malformadas.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Music.def

Mesmo formato dos redirecionamentos, com uma única entrada neutra e sem fim de linha: `0 {0} 0`.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## art.def

Redireciona um índice de arte para outro, com matiz. Formato `<ID> {<NOVO>} <MATIZ>`; separadores tab ou espaço, ambos presentes no mesmo arquivo. Contém 620 entradas (IDs de 2500 a 26319), uma chave duplicada. Comentários de seção: `#Begin AoS Stuff`, `# Samurai stuff`, `# Ninja stuff`, `# BOD system`, `# Mahjong Game`.

```
2500	{3}	1645
25073	{24863}	1161
25349	{21824} 0
```

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## TexTerr.def

Mesmo formato do art.def (`<ID> {<NOVO>} <MATIZ>`, separadores espaço ou tab). Contém 285 entradas (IDs de 2500 a 26288), 284 delas com ID também presente em art.def; não tem comentários.

```
2500  {3} 1645
25940 {20320} 1645
```

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## entitled.def

Arquivo binário de 4 bytes: `01 00 00 00` (inteiro de 32 bits little-endian com valor 1). Não é texto.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## Intrface.def

Script de layout da interface (gumps de interface do cliente). Texto UTF-16 little-endian com BOM `FF FE`, fim de linha CRLF, 7940 linhas. O cabeçalho (`// NOTICE...`) pede para não editar o arquivo.

Sintaxe observada:

- `//` inicia comentário de linha.
- `# nome` abre um bloco nomeado e `#enddef` (ou `# enddef`) o fecha. Os blocos são referenciados por nome em outros pontos (`include MessageBoxBackground`, e como último argumento de `interface`, `layoutbutton`, etc.).
- Um comando por linha, argumentos separados por espaço: `rect x y w h`, `background gump x y ...`, `button id x y gumpNormal gumpPressed ...`, `textsimple`, `text`, `textarea`, `toggle`, `container`, `decoration`, `statusbar`, `vlist`, `colorpicker`, entre outros. Os mais frequentes são `sizablebackground` (375 linhas), `text` (288), `rect` (254), `textarea` (229), `layoutbutton` (221), `background` (212).
- Linhas iniciadas por tab são propriedades do elemento imediatamente anterior (`fontnumber 2`, `textoutline 25297`, `emboss`, `rotation 3`, `settooltip 1062709`, `setbuttonholdable`).
- Coordenadas podem ser expressões: `R-170` (relativo à borda direita), `B-(238+0*gw11321)` (relativo à borda inferior), `gw11321`/`gh11264` (largura/altura do gump indicado).
- `settooltip` e `tooltip` recebem números de cliloc (ex.: `settooltip 1062709`).

Exemplo:

```
# mahjongboard
rect 100 100 gw11264 gh11264
fadeininterface 500 100
background 11264 0 0 0 0
textsimple 1606501 258 499 200 20 0
	fontnumber 2
	textoutline 25297
```

Campos não interpretados: significado individual de cada argumento posicional e de cada comando.

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## stitchin.def

Regras de composição de aparência (cabelo, barba, roupas) para o personagem 3D. Texto ASCII, CRLF, 9202 linhas. Primeira linha `ver 1`. Comentários com `//`.

Cada definição começa em `# <ID>` (675 blocos) e termina em `# enddef`/`#enddef`. Diretivas dentro do bloco:

| Diretiva | Argumentos |
|---|---|
| `covers` | partes do corpo cobertas |
| `coveredBy` | partes que, se cobertas, afetam este item |
| `remove` | lista de IDs removidos |
| `replace A with B` | substitui o ID A por B |
| `whenhuedreplacewith` | ID usado quando há matiz |

Partes do corpo: `HEAD FACE EARS NECK TORSO UPPER_ARMS_TOP UPPER_ARMS_BOTTOM LOWER_ARMS_TOP LOWER_ARMS_BOTTOM HANDS PELVIS UPPER_LEGS_TOP UPPER_LEGS_BOTTOM LOWER_LEGS_TOP LOWER_LEGS_BOTTOM FEET`.

Exemplo:

```
# 990
covers HEAD FACE EARS NECK TORSO ... FEET
 remove 703 903 701 710 902 702 901 700 900
# enddef
```

Campos não interpretados: significado dos IDs referenciados (modelos 3D).

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)
