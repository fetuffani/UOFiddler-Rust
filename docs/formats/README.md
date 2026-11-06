# Documentação dos formatos de arquivo do cliente Ultima Online

Referência dos arquivos do diretório do cliente e do protocolo de rede.

## Índice de documentos

### Índices e arquivos de dados
- [index-mul.md](index-mul.md): pares índice/dados (`*idx.mul` + `*.mul`) e o formato comum de entrada de índice.

### Mapas
- [maps.md](maps.md): `map*.mul`, `statics*.mul`, `staidx*.mul` e arquivos de diferença (`mapdif*`, `stadif*`).

### Tabelas
- [tables.md](tables.md): `tiledata.mul`, `hues.mul`, `radarcol.mul`, `palette.mul`, `animdata.mul`, `animinfo.mul`, `chardata.mul`, `fonts.mul`, `unifont*.mul`.

### Animações
- [animations.md](animations.md): `anim*.mul`/`anim*.idx`, `animdata.mul`, `animinfo.mul`.

### UOP
- [uop-container.md](uop-container.md): estrutura do contêiner `.uop`.
- [uop-files.md](uop-files.md): arquivos `.uop` do cliente e o conteúdo de cada um.

### Mapas de visão geral
- [overview-maps.md](overview-maps.md): `Multimap.rle` e `facet*.mul`.

### Definições
- [def-files.md](def-files.md): arquivos `.def` (`Body.def`, `Anim*.def`, `art.def`, `gump.def` e outros).

### Tabelas de texto
- [text-tables.md](text-tables.md): arquivos `.txt`, `.cfg`, `.lst` e `.dat` de texto (`Tile*.txt`, `Macros.txt`, `Keynames.txt`, `doors.txt`, `Credits.dat` e outros).

### Localização
- [localization.md](localization.md): `Cliloc.*`, `Intloc*.*`, `Gt_*.*`, `Tooltips.*`, `Tilehelp.*`, `Langcode.iff`, `speech.mul` e demais arquivos por idioma.

### Binários
- [misc-binaries.md](misc-binaries.md): `Anim256.bin`, `Anim1024.bin`, `Antx*.bin`, `Blueprints.tbp`, `desktop.*`, `skillgrp.mul`, `sjis2uni.mul` e arquivos de log/instalação.

### Executáveis
- [executables.md](executables.md): clientes, launcher, patcher, bibliotecas e arquivos auxiliares.

### Protocolo
- [../protocol/packets.md](../protocol/packets.md): tabelas de pacotes servidor-cliente e cliente-servidor (client 7.0.x).
- [../protocol/gump-layout.md](../protocol/gump-layout.md): comandos de layout de gump.

## Mapa de arquivos

| Padrão | Documento |
|---|---|
| `artidx.mul`, `art.mul` | [index-mul.md](index-mul.md) |
| `gumpidx.mul`, `gumpart.mul` | [index-mul.md](index-mul.md) |
| `soundidx.mul`, `sound.mul` | [index-mul.md](index-mul.md) |
| `texidx.mul`, `texmaps.mul` | [index-mul.md](index-mul.md) |
| `lightidx.mul`, `light.mul` | [index-mul.md](index-mul.md) |
| `multi.idx`, `multi.mul` | [index-mul.md](index-mul.md) |
| `Skills.idx`, `skills.mul` | [index-mul.md](index-mul.md) |
| `map*.mul`, `map*x.mul`, `mapdif*.mul`, `mapdifl*.mul` | [maps.md](maps.md) |
| `statics*.mul`, `staidx*.mul`, `stadif*.mul`, `stadifi*.mul`, `stadifl*.mul` | [maps.md](maps.md) |
| `anim.mul`, `anim.idx`, `anim2..5.mul`, `anim2..5.idx` | [animations.md](animations.md) |
| `animdata.mul`, `animinfo.mul` | [tables.md](tables.md), [animations.md](animations.md) |
| `AnimationFrame*.uop`, `AnimationSequence.uop` | [uop-files.md](uop-files.md), [uop-container.md](uop-container.md) |
| `MainMisc.uop`, `string_dictionary.uop`, `tileart.uop` | [uop-files.md](uop-files.md), [uop-container.md](uop-container.md) |
| `Anim256.bin`, `Anim1024.bin`, `Antx256.bin`, `Antx1024.bin` | [misc-binaries.md](misc-binaries.md) |
| `facet*.mul`, `Multimap.rle` | [overview-maps.md](overview-maps.md) |
| `tiledata.mul`, `hues.mul`, `radarcol.mul`, `palette.mul` | [tables.md](tables.md) |
| `fonts.mul`, `unifont*.mul`, `chardata.mul` | [tables.md](tables.md) |
| `speech.mul`, `Langcode.iff` | [localization.md](localization.md) |
| `skillgrp.mul`, `sjis2uni.mul` | [misc-binaries.md](misc-binaries.md) |
| `*.def` (`Body`, `Bodyconv`, `Anim1..4`, `art`, `gump`, `Corpse`, `Equipconv`, `Intrface`, `Music`, `Sound`, `TexTerr`, `stitchin`, `entitled`) | [def-files.md](def-files.md) |
| `*.txt` (`Tile256`, `Tile1024`, `Keynames`, `Macros`, `Umacros`, `macros*d`, `Prof`, `doors`, `floors`, `walls`, `roof`, `stairs`, `teleprts`, `mobtypes`, `misc`, `metrics`, `gamegen`, `suppinfo`, `map0`) | [text-tables.md](text-tables.md) |
| `Cliloc.*`, `Intloc*.*`, `Gt_*.*`, `Tooltips.*`, `Tilehelp.*`, `Gesture.*`, `Intro.*`, `Chat.*`, `Professn.*`, `options.*`, `OPTNUOTD.*`, `optuotd.*`, `Skilname.*`, `SKILNAME.*`, `SKILL16.*`, `SKILL30.*` | [localization.md](localization.md) |
| `Blueprints.tbp` | [misc-binaries.md](misc-binaries.md) |
| `desktop.uo1`, `desktop.nwb`, `Desktop/**` | [misc-binaries.md](misc-binaries.md) |
| `Credits.dat`, `Helpindx.dat`, `Version.dat`, `Version.txt`, `Verinfo`, `Obscene.lst`, `uobscene.lst`, `ignore.lst`, `conf_ignore.lst`, `Default.mac` | [text-tables.md](text-tables.md) |
| `Login.cfg`, `uo.cfg`, `Uoreg.cfg`, `Uomax.cfg`, `Uoupg.cfg`, `Arttable.cfg`, `client.exe.txt` | [text-tables.md](text-tables.md) |
| `vercfg.cfg`, `telltext.cfg`, `regfile.reg`, `patchlog`, `Uoreport0.log` | [misc-binaries.md](misc-binaries.md) |
| `*.exe`, `*.dll*`, `*.chm`, `*.manifest`, `client.zip`, `patch/`, `GDF/`, `Logs/`, `music/` | [executables.md](executables.md) |
