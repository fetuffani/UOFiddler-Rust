# Arquivos binários diversos

Arquivos de dados do cliente que não pertencem aos índices `.mul`/`.idx`, aos mapas, às tabelas nem aos contêineres UOP. 

Convenções: inteiros em little-endian, salvo indicação contrária.

## Anim256.bin / Anim1024.bin

Atlas de quadros de animação (cliente com renderização por texturas). Os dois arquivos têm o mesmo tamanho (4.082.540 bytes) e o mesmo número de registros (204.127). Cada arquivo trabalha em conjunto com o `Antx*.bin` de mesmo sufixo (`Anim256.bin` com `Antx256.bin`, `Anim1024.bin` com `Antx1024.bin`).

Sem cabeçalho. Registros de 20 bytes (5 x u32):

| Offset | Tipo | Campo | Observação |
|---|---|---|---|
| 0x00 | u32 | `group` | Valor mínimo 110 (0x6E), máximo 139124. Repete em sequências de 1 a 20 registros consecutivos. Idêntico nos dois arquivos. |
| 0x04 | u32 | `frame` | Estritamente crescente ao longo do arquivo (0 a 204559), com lacunas. Idêntico nos dois arquivos. |
| 0x08 | u32 | `sheet` | Índice de registro em `Antx*.bin` (sempre válido). Não decrescente ao longo do arquivo. |
| 0x0C | u32 | `x` | Posição horizontal; em sequências do mesmo `sheet` cresce pela largura do registro de `Antx`. Interpretação completa não estabelecida. |
| 0x10 | u32 | `y` | Posição vertical; interpretação completa não estabelecida. |

Primeiros registros de `Anim1024.bin` (`group`, `frame`, `sheet`, `x`, `y`): `(110,0,0,0,0)`, `(110,1,0,134,0)`, `(110,2,0,268,0)`.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## Antx256.bin / Antx1024.bin

Tabela de folhas (sheets) referenciadas por `Anim*.bin`. `Antx256.bin` tem 18.856 registros (377.120 bytes); `Antx1024.bin` tem 1.433 registros (28.660 bytes).

Sem cabeçalho. Registros de 20 bytes (5 x u32):

| Offset | Tipo | Campo | Observação |
|---|---|---|---|
| 0x00 | u32 | `index` | Igual à posição do registro (0, 1, 2, ...). |
| 0x04 | u32 | `width` | Largura em pixels (5 a 508). |
| 0x08 | u32 | `height` | Altura em pixels (4 a 376). |
| 0x0C | u32 | `atlasWidth` | Dimensão de textura (até 512 em `Antx256`, até 1024 em `Antx1024`). |
| 0x10 | u32 | `atlasHeight` | Idem para a altura. |

Os dois últimos registros de cada arquivo repetem o `index` final; o último tem `atlasWidth` e `atlasHeight` iguais a 0.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## Blueprints.tbp

Dados binários de blueprints (94.993 bytes). Estrutura interna não interpretada.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## desktop.uo1, desktop.nwb e Desktop/*/desktop.uos

Estado salvo da interface (posição e abertura de janelas) do cliente. Tamanhos: `desktop.uo1` 72 bytes, `desktop.nwb` 164 bytes, `Desktop/common/desktop.uos` 163 bytes. Estrutura interna não interpretada.

Arquivos relacionados em `Desktop/<perfil>/`:

| Arquivo | Conteúdo |
|---|---|
| `Multicache.dat` | Texto de 8 bytes: `ver 5` + CRLF. |
| `chardata.mul` | 35 bytes; começa com o texto `ver 1` + LF, seguido de valores binários não interpretados. |
| `macros2d.txt`, `skillgrp.mul` | Macros e grupos de skills do perfil. |

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## sjis2uni.mul

Tabela de conversão Shift-JIS para Unicode: 65.536 entradas u16 (131.072 bytes), indexada pelo código Shift-JIS de 16 bits; o valor é o ponto de código Unicode (0 quando não há mapeamento). Exemplo: entrada `0x8140` = `0x3000`, entrada `0x8143` = `0xFF0C`.

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## skillgrp.mul

Agrupamento das skills por categoria (338 bytes), little-endian:

| Campo | Tipo | Descrição |
|---|---|---|
| `count` | i32 | Número de grupos (7) |
| `names` | `char[17]` x (`count` - 1) | Nomes dos grupos 1 a `count`-1, preenchidos com NUL (`Combat`, `Trade Skills`, `Magic`, `Wilderness`, `Thieving`, `Bard`); o grupo 0 não tem nome armazenado |
| `skillGroup` | i32 x 58 | Índice do grupo de cada skill, na ordem dos ids de skill (valores 0 a 6) |

Leitor no Fiddler: não interpretado (apenas hex na aba Arquivos)

## Multimap.rle

Mapa de visão geral. Descrito em [overview-maps.md](overview-maps.md).

## Arquivos descritos em outros documentos

| Arquivo | Documento |
|---|---|
| `Skills.idx` | [index-mul.md](index-mul.md) |
| `Langcode.iff` | [localization.md](localization.md) |
| `Multimap.rle` | [overview-maps.md](overview-maps.md) |
| `Credits.dat`, `Helpindx.dat`, `Version.dat`, `Version.txt`, `Verinfo`, `Obscene.lst`, `uobscene.lst`, `ignore.lst`, `conf_ignore.lst`, `Default.mac`, `Arttable.cfg`, `Login.cfg`, `uo.cfg`, `Uoreg.cfg`, `Uomax.cfg`, `Uoupg.cfg`, `chardata.mul` | [text-tables.md](text-tables.md) |

## Arquivos de log e instalação

| Arquivo | Formato | Função |
|---|---|---|
| `vercfg.cfg` | texto, um valor por linha | Parâmetros do patcher (pasta de patch, servidor, janela, URL) |
| `telltext.cfg` | texto | Vazio |
| `regfile.reg` | texto UTF-16 (Registry Editor 5.00) | Chaves de registro da instalação |
| `patchlog` | texto | Log de sessões de download de patch |
| `Uoreport0.log` | texto | Log de erros (vazio) |
