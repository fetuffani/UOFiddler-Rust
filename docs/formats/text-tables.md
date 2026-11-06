# Tabelas de texto, configuração e arquivos pequenos do cliente

Convenção de leitura para todos os arquivos de texto: aceitar BOM UTF-8 (`EF BB BF`), UTF-16 com BOM (`FF FE` little-endian ou `FE FF` big-endian) e, sem BOM, ASCII/Latin-1; aceitar CRLF e LF. Salvo indicação, os arquivos abaixo são ASCII com CRLF.

## mobtypes.txt

Tipo e flags de cada corpo de animação. ASCII, CRLF, 1210 linhas.

```
<ID> <TIPO> <FLAGS>
```

| Coluna | Descrição |
|---|---|
| `ID` | corpo, decimal |
| `TIPO` | `MONSTER`, `ANIMAL`, `SEA_MONSTER`, `HUMAN` ou `EQUIPMENT` (maiúsculas) |
| `FLAGS` | inteiro em hexadecimal, sem prefixo (`0`, `2A`, `1008`, `10000`) |

Separadores: tab ou espaços, em quantidade variável. As duas primeiras linhas são comentários (`# Animation types for animation lookups`, `# ID TYPE FLAGS`). Comentários `#` também aparecem no fim de linha e em linhas inteiras (entradas desativadas, ex.: `# 130 MONSTER 8`). Algumas linhas trazem uma quarta coluna `0` antes do comentário (`753 MONSTER 10000 0 # Gargoyle Slave`), e duas linhas (`492`, `427`) têm o comentário no lugar das flags. Há 1099 linhas de dados, com 1048 IDs distintos; os IDs não estão em ordem crescente e alguns se repetem. Contagem por tipo: EQUIPMENT 723, MONSTER 255, ANIMAL 96, HUMAN 21, SEA_MONSTER 4.

Bit em uso: `0x10000` (flag `10000`, 144 linhas) marca animação armazenada nos contêineres UOP.

Exemplo:

```
1	MONSTER	0
5	ANIMAL	2A
666	HUMAN 		10000	0 #	Gargoyle Male
```

Campos não interpretados: demais bits de `FLAGS`.

Leitor no Fiddler: `crates/uo_core/src/animations.rs` (`parse_mobtypes_txt`)

## Tile256.txt e Tile1024.txt

Tabelas de pares `<ID><TAB><VALOR>`, ambos decimais, sem cabeçalho nem comentários, CRLF.

| Arquivo | Linhas | Faixa de ID | Faixa de valor | Valores distintos |
|---|---|---|---|---|
| Tile256.txt | 1604 | 2 a 5000 | 0 a 162 | 163 |
| Tile1024.txt | 1581 | 2 a 2799 | 0 a 10 | 11 |

Os IDs são únicos e quase todos em ordem crescente. Exemplo (Tile256.txt): `16<TAB>1`, `26<TAB>2`, `5000<TAB>157`.

Campos não interpretados: significado do valor.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Tabelas de construção de casas: walls, floors, roof, doors, stairs, teleprts, misc

Catálogos de peças para o editor de casas. ASCII, tab como separador, CRLF (teleprts.txt usa `CR CR LF`).

Estrutura comum:

1. Linha 1: tipo de cada coluna (`int` ... `string`).
2. Linha 2: nome de cada coluna.
3. Linhas de dados: uma categoria/estilo por linha; a última coluna (`Comment`) é o nome em texto livre.

doors.txt tem linhas em branco nas posições 2 e 4; todos os arquivos terminam com CRLF.

| Arquivo | Colunas | Linhas de dados |
|---|---|---|
| walls.txt | `Category Style TID South1 South2 South3 Corner East1 East2 East3 Post WindowS AltWindowS WindowE AltWindowE SecondAltWindowS SecondAltWindowE FeatureMask Comment` | 171 |
| floors.txt | `Category F1 ... F16 FeatureMask Comment` | 55 |
| roof.txt | `Category Style TID North East South West NSCrosspiece EWCrosspiece NDent EDent SDent WDent NTPiece ETPiece STPiece WTPiece XPiece Extra Piece FeatureMask Comment` | 34 |
| doors.txt | `Category Piece1 ... Piece8 FeatureMask Comment` | 36 |
| stairs.txt | `Category Block North East South West Squared1 Squared2 Rounded1 Rounded2 MultiNorth MultiEast MultiSouth MultiWest FeatureMask Comment` | 17 |
| teleprts.txt | `Category F1 ... F16 FeatureMask Comment` | 1 |
| misc.txt | `Category Style TID Piece1 ... Piece8 FeatureMask Comment` | 91 |

Notas:

- Os valores de peça são IDs de arte de item (tiles estáticos); `0` = peça ausente.
- `TID` é um número de cliloc com o nome da categoria; `0` = sem nome próprio (a linha continua o estilo anterior). Ex.: TID 1060054 corresponde ao texto "Dark Wood Walls" em Cliloc.enu.
- `FeatureMask` é uma máscara de bits de recursos exigidos. Valores presentes: 0, 64, 128, 512, 65536, 262144, 524288.
- `walls.txt`, `roof.txt`, `doors.txt`: 18/20/10 colunas numéricas respectivamente mais `Comment`.
- misc.txt tem uma linha com 12 colunas e separador de espaços em vez de tab entre duas peças (`19261   19249`); leitores devem tolerar.

Exemplos:

```
Category	Style	TID	South1	...	FeatureMask	Comment
0	0	1060054	10	7	12	6	13	8	11	9	0	14	15	0	0	0	0	Dark Wood Standard Walls
```

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## suppinfo.txt

Informação de suporte (apoio estrutural) por peça de construção, uma linha por tile. ASCII, CRLF, 1783 linhas.

- Linha 1: tipos (primeira coluna vazia, depois `int` x15 e `string`); linha 2: nomes.
- Colunas: (1) símbolo de forma, texto curto (`\`, `./`, `v`, `.`, `o`, `*`, `` ` ``, `*\`, `*/`, `..`, `\\`, `//`); (2) `tileNumber`; (3..16) `top bottom adj_UN adj_LN adj_UE adj_LE adj_US adj_LS adj_UW adj_LW directSupports cango W cango N cango NWC` (inteiros 0/1); (17) `Comment`, geralmente vazio.
- Algumas linhas usam espaços em vez de tabs em campos 0/1.

```
	tileNumber	top	bottom	adj_UN	adj_LN	adj_UE	adj_LE	adj_US	adj_LS	adj_UW	adj_LW	directSupports	cango W	cango N	cango NWC	Comment
\	10	1	1	0	0	1	1	1	1	1	1	1	0	1	0	Dark Wood Std
```

Campos não interpretados: semântica dos flags de adjacência e do símbolo de forma.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Keynames.txt

Nomes das teclas. ASCII, CRLF, 89 linhas. A linha de índice N (começando em 0) é o nome da tecla de código de varredura N: linha 0 `NULL`, 1 `Esc`, 2 `1`, 14 `Backspace`, 15 `Tab`, 28 `Enter`, 57 `Space`, 59 `F1`, 87 `F11`, 88 `F12`. Códigos sem tecla usam `NULL`. Teclado numérico: `Num 7`, `Num 8`, `Num +`, `Num Del`.

Os nomes deste arquivo são os usados na primeira coluna dos arquivos de macro.

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Macros.txt, Umacros.txt, macros2d.txt, Default.mac

Atalhos de teclado (macros). Cada macro é um bloco terminado por uma linha `########`.

```
<TECLA> <ALT> <CTRL> <SHIFT>
<AÇÃO>
########
```

| Linha | Descrição |
|---|---|
| Cabeçalho | nome da tecla (conforme Keynames.txt; pode ter espaço, ex.: `Num 8`) seguido de três flags 0/1 separadas por espaço: Alt, Ctrl, Shift. O nome é tudo que precede os três últimos tokens |
| Ação | texto da ação (`Open Status`, `War/Peace`, `Bow`, `Say Forward`); `NULL` = nenhuma ação |
| `########` | fim do bloco (oito `#`) |

Arquivos:

| Arquivo | Conteúdo |
|---|---|
| Default.mac | 22 macros padrão; ação em texto simples; 67 linhas, CRLF. Inclui `Say Forward`, `Say Stop`, `Say Turn Left`, `Say Raise Anchor` nas 9 teclas `Num` (flags `0 1 0`) |
| Macros.txt | 22 macros (13 letras com Ctrl, `B` e `S` com Alt, 7 teclas `Num` com `1 1 0`), todas com ação `NULL ` (o texto `NULL` é seguido de espaço) |
| Umacros.txt | arquivo vazio (0 bytes) |
| macros2d.txt | 15 macros; cabeçalho em ASCII e linha de ação formada por `+` (1 byte) seguido do texto em UTF-16 little-endian sem BOM, terminada por CRLF em um byte (`+O\0p\0e\0n\0 \0S\0t\0a\0t\0u\0s\0`) |

Exemplos (Default.mac / Macros.txt):

```
S 0 1 0
Open Status
########
B 1 0 0
Bow
########
Num 8 0 1 0
Say Forward
########
```

Ações presentes em macros2d.txt: `Open Status`, `Open Chat`, `Open MageSpellbook`, `War/Peace`, `Open Paperdoll`, `Open Skills`, `Open Journal`, `Open QuestLog`, `Open SpellWeavingSpellbook`, `Open Backpack`, `Open Overview`, `Open Configuration`, `QuitGame`, `Bow`, `Salute`. As ações aceitam espaço final.

Campos não interpretados: um macro com mais de uma ação (todos os blocos presentes têm uma única ação); o terceiro flag nunca é diferente de 0.

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Prof.txt

Definição das profissões da criação de personagem. ASCII, CRLF, 124 linhas, sem fim de linha após o último `End`. Indentação com tabs, vários tabs entre chave e valor (alinhamento).

```
Begin
	Name			Samurai
	TrueName		"Samurai"
	NameId			1062948
	DescId			1062950
	Desc			6
	TopLevel		true
	Gump			5591
	Type			Profession
	Skill			Bushido			30
	Skill			Swordsmanship		30
	Skill			Focus		30
	Skill			Parrying	30
	Stat			Str			40
	Stat			Dex			30
	Stat			Int			20
End
```

| Chave | Descrição |
|---|---|
| `Name` | nome interno |
| `TrueName` | nome entre aspas |
| `NameId` | número de cliloc do nome (1062948 = "SAMURAI") |
| `DescId` | número de cliloc da descrição |
| `Desc` | índice numérico da descrição (1 a 7, único por profissão) |
| `TopLevel` | `true` |
| `Gump` | ID do gump da profissão |
| `Type` | `Profession` |
| `Skill <nome> <valor>` | uma por linha; o nome pode ter espaços (`Spirit Speak`, `Evaluate Intelligence`); o valor é o último token |
| `Stat <Str|Dex|Int> <valor>` | três linhas por profissão |

Há 7 profissões (Samurai, Ninja, Paladin, Necromancer, Warrior, Mage, Blacksmith), cada uma com 4 skills de valor 30 e stats somando 90.

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## map0.txt

Lista de células do mapa 0 em texto, ASCII, CRLF, 482505 linhas de três inteiros separados por vírgula: `x,y,z`.

- `x` de 0 a 119 e `y` de 0 a 4095; `z` de -16 a 3 (o valor mais comum é -5).
- A ordem segue blocos de 8x8 células: o bloco varia primeiro em Y e depois em X (o arquivo cobre os blocos de coluna 0 a 14); dentro do bloco as 64 células vão por linha (x de 0 a 7 para cada y).
- A última linha está truncada (`112,2969,-5` sem terminador completo).

```
0,0,-5
1,0,-5
...
7,7,-5
0,8,-5
```

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Version.txt, Version.dat, Verinfo, client.exe.txt

| Arquivo | Conteúdo |
|---|---|
| Version.txt | versão do cliente em uma linha: `1.25.35` + CRLF (9 bytes) |
| Version.dat | 4 bytes: `xx` + CRLF |
| Verinfo | `Win32_UOSA 65` + CRLF: plataforma e número de build separados por espaço |
| client.exe.txt | texto de apoio de 8 linhas: um cabeçalho de duas frases e a linha `Format style is: DLL Name::API Name->Call to API`, seguidos de linhas `DLL::símbolo->instrução` (ex.: `UL.dll::?SendPingMessage@@YA_NPADH00H@Z->call dword ptr [b65180]`) |

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## metrics.txt e gamegen.txt

metrics.txt é um relatório de configuração de máquina, ASCII, CRLF, 27 linhas, no formato `Chave: valor`, com linhas em branco entre grupos: `Instance Id`, `OS Version`, `CPU Manufacturer`, `CPU Family`, `CPU Model`, `CPU Clock Speed`, `CPU Quantity`, `Physical Memory`, `Desktop Resolution and Color Depth`, `DirectX Version`, `Video Card Description`, `Video Card Vendor Id`, `Video Card Device Id`, `Video Card Memory`, `Distribution`, `Client Running`, `Clients Installed`, `Partial Install`, `Language Preference`, `DVD-ROM drive`.

```
Instance Id: 34
OS Version: 0.0.0
Physical Memory: 0MB
Partial Install: no
```

gamegen.txt tem 11 bytes e contém apenas o texto `metrics.txt`, sem fim de linha.

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Login.cfg, Uomax.cfg, Uoreg.cfg, Uoupg.cfg, uo.cfg

Configuração do cliente no formato `Chave=Valor`, uma por linha, CRLF, sem seções. Comentário de linha com `;` (uo.cfg). Valores booleanos: `on`/`off`, `yes`/`no`, `true`.

| Arquivo | Conteúdo |
|---|---|
| Login.cfg | `LoginServer=localhost,2593` (host e porta separados por vírgula; 28 bytes) |
| Uomax.cfg | opções de execução |
| Uoreg.cfg | opções de execução (inclui `OrderCache`) |
| Uoupg.cfg | subconjunto com apenas as opções de tradução (9 linhas) |
| uo.cfg | opções de execução e estado da última sessão |

Chaves presentes em Uomax.cfg/Uoreg.cfg: `CdRomDataPath`, `CacheSize`, `HdUsage`, `PageFlip`, `Sound`, `FullScreen`, `Music`, `SavePassword`, `Password`, `SoundVol` (0 a 255), `MusicVol` (0 a 255), `FilterPassword`, `ObscenityFilter`, `OrderCache` (só Uoreg), `UnicodeSpeech`, `TranslationServer`, `TranslationServerIPAddress` (host,porta), `TranslationServerTranslatorName`, `TranslationServerSystranNFWMarker`, `TranslationServerSystranTopicalGlossaries`, `TranslationServerSystranDocumentType`, `TranslationServerAutomaticTranslation`, `TranslationServerClickTranslation`.

Chaves adicionais em uo.cfg: `ClientFirstLaunch`, `NextLoginKey`, `PlayerName`, `ClientLogin`, `LastTip`, `ShowTips`, `CurrentChatChannel`, `LastServerID`, `DefaultChar`, `AcctID`. `Password` e `FilterPassword` são campos de credencial e não devem ser preenchidos em arquivos distribuídos.

Exemplo:

```
LoginServer=localhost,2593
```

```
CacheSize=25
HdUsage=max
SoundVol=255
UnicodeSpeech=on
TranslationServerIPAddress=127.0.0.1,28888
```

Leitor no Fiddler: não interpretado (apenas texto/tabela na aba Arquivos)

## Arttable.cfg

Tabela de redirecionamento de arte, ASCII, CRLF, 110 linhas. Dois formatos, sempre com espaços ao redor das chaves:

```
<ID> { <NOVO> } <FLAG>
<ID> { <A>, <B>, <C> } <FLAG>
```

| Forma | Linhas | Exemplo |
|---|---|---|
| Um destino, `FLAG` 0 | 49 | `15821 { 2140 } 0` |
| Um destino, `FLAG` -1 | 1 | `3 { 3 } -1` (primeira linha) |
| Três destinos, `FLAG` 0 | 60 | `16927 { 16927, 24968, 24969 } 0` |

Nas 60 linhas de três destinos o primeiro valor é sempre igual ao ID da chave.

Campos não interpretados: significado de `FLAG` e dos dois valores adicionais.

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Helpindx.dat

Índice de tópicos de ajuda (Codex de ajuda do cliente). Texto UTF-8 (apenas ASCII presente), CRLF, 351 linhas.

- `# Título` (comentário) precede cada tópico (176 comentários).
- Linha de tópico, separada por tabs: `[@]<id> <cliloc1> <cliloc2> <cliloc3> [<id relacionado> ...]`.

| Campo | Descrição |
|---|---|
| `@` opcional | prefixo presente em 133 dos 174 tópicos; significado não interpretado |
| `id` | número do tópico, de 1 a 176, único |
| `cliloc1` | cliloc do título do tópico (3010031 = "Paperdoll") |
| `cliloc2` | cliloc da seção (3010030 = "Movement"; 3013030 para os tópicos de casas) |
| `cliloc3` | cliloc do texto (3010101 = texto do paperdoll) |
| restantes | IDs de tópicos relacionados (zero a 16) |

Os textos de cliloc podem conter hiperlinks `<a href = "?165">` onde o número é um ID de tópico.

```
# Movement
1	3010030	3010030	3010100
# Paperdoll
2	3010031	3010030	3010101	3	4	5	6	7	8	9	10	11	12
# Help Button
@3	3010032	3010030	3010102
```

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Credits.dat

Texto dos créditos, UTF-8, CRLF, 693 linhas (10732 bytes). Linhas iniciadas por `! ` são diretivas; as demais são texto exibido.

| Linha | Efeito |
|---|---|
| `! song N [song M ...]` | lista de músicas |
| `! defaultfont N` | fonte padrão |
| `! defaultcolor #RRGGBB` | cor padrão |
| `! color #RRGGBB` | cor do trecho seguinte (129 diretivas no total) |
| `---=== título ===---`, `-== seção ==-` | títulos |

```
! song 49 song 50
! defaultfont 0
! defaultcolor #9999FF
! color #FFFF00
---=== Ultima Online: High Seas Booster Pack ===---
```

Leitor no Fiddler: não interpretado (apenas texto na aba Arquivos)

## Obscene.lst, uobscene.lst, ignore.lst, conf_ignore.lst

| Arquivo | Estrutura |
|---|---|
| Obscene.lst | 61 bytes, CRLF, uma palavra por linha, com os caracteres deslocados por uma cifra simples (não são texto legível) |
| uobscene.lst | 1048 bytes; mesmo conteúdo em unidades de 16 bits little-endian, cifradas, com terminador `0x0017` entre palavras |
| ignore.lst | lista de jogadores ignorados; registros de 34 bytes: nome em 30 bytes terminado e preenchido com NUL, seguido de `uint32` little-endian (no arquivo presente, 1 registro: nome `Nao Ha Vagas`, valor 160412) |
| conf_ignore.lst | vazio (0 bytes) |

Campos não interpretados: cifra de Obscene.lst/uobscene.lst; significado do `uint32` de ignore.lst.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## chardata.mul

Arquivo binário de 20 bytes que começa com a linha de texto ` ver 1` seguida de LF (`20 76 65 72 20 31 0A`), seguida de 13 bytes binários: cinco pares `B2 02` (cinco valores 16 bits little-endian, 0x02B2) e `34 00 25`.

Campos não interpretados: significado dos 13 bytes após a linha de versão.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)
