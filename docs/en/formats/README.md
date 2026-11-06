# Documentation of the Ultima Online client file formats

Reference for the files in the client directory and for the network protocol.

## Document index

### Indexes and data files
- [index-mul.md](index-mul.md): index/data pairs (`*idx.mul` + `*.mul`) and the common index entry format.

### Maps
- [maps.md](maps.md): `map*.mul`, `statics*.mul`, `staidx*.mul` and difference files (`mapdif*`, `stadif*`).

### Tables
- [tables.md](tables.md): `tiledata.mul`, `hues.mul`, `radarcol.mul`, `palette.mul`, `animdata.mul`, `animinfo.mul`, `chardata.mul`, `fonts.mul`, `unifont*.mul`.

### Animations
- [animations.md](animations.md): `anim*.mul`/`anim*.idx`, `animdata.mul`, `animinfo.mul`.

### UOP
- [uop-container.md](uop-container.md): structure of the `.uop` container.
- [uop-files.md](uop-files.md): the client's `.uop` files and the contents of each one.

### Overview maps
- [overview-maps.md](overview-maps.md): `Multimap.rle` and `facet*.mul`.

### Definitions
- [def-files.md](def-files.md): `.def` files (`Body.def`, `Anim*.def`, `art.def`, `gump.def` and others).

### Text tables
- [text-tables.md](text-tables.md): text `.txt`, `.cfg`, `.lst` and `.dat` files (`Tile*.txt`, `Macros.txt`, `Keynames.txt`, `doors.txt`, `Credits.dat` and others).

### Localization
- [localization.md](localization.md): `Cliloc.*`, `Intloc*.*`, `Gt_*.*`, `Tooltips.*`, `Tilehelp.*`, `Langcode.iff`, `speech.mul` and other per-language files.

### Binaries
- [misc-binaries.md](misc-binaries.md): `Anim256.bin`, `Anim1024.bin`, `Antx*.bin`, `Blueprints.tbp`, `desktop.*`, `skillgrp.mul`, `sjis2uni.mul` and log/installation files.

### Executables
- [executables.md](executables.md): clients, launcher, patcher, libraries and auxiliary files.

### Protocol
- [../protocol/packets.md](../protocol/packets.md): server-to-client and client-to-server packet tables (client 7.0.x).
- [../protocol/gump-layout.md](../protocol/gump-layout.md): gump layout commands.

## File map

| Pattern | Document |
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
