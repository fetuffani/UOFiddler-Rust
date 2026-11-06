# Miscellaneous binary files

Client data files that do not belong to the `.mul`/`.idx` indexes, the maps, the tables, or the UOP containers. 

Conventions: integers are little-endian unless stated otherwise.

## Anim256.bin / Anim1024.bin

Animation frame atlas (client with texture-based rendering). Both files have the same size (4,082,540 bytes) and the same number of records (204,127). Each file works together with the `Antx*.bin` of the same suffix (`Anim256.bin` with `Antx256.bin`, `Anim1024.bin` with `Antx1024.bin`).

No header. Records of 20 bytes (5 x u32):

| Offset | Type | Field | Note |
|---|---|---|---|
| 0x00 | u32 | `group` | Minimum value 110 (0x6E), maximum 139124. Repeats in runs of 1 to 20 consecutive records. Identical in both files. |
| 0x04 | u32 | `frame` | Strictly increasing throughout the file (0 to 204559), with gaps. Identical in both files. |
| 0x08 | u32 | `sheet` | Record index in `Antx*.bin` (always valid). Non-decreasing throughout the file. |
| 0x0C | u32 | `x` | Horizontal position; within runs with the same `sheet`, it increases by the width of the `Antx` record. Full interpretation not established. |
| 0x10 | u32 | `y` | Vertical position; full interpretation not established. |

First records of `Anim1024.bin` (`group`, `frame`, `sheet`, `x`, `y`): `(110,0,0,0,0)`, `(110,1,0,134,0)`, `(110,2,0,268,0)`.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## Antx256.bin / Antx1024.bin

Sheet table referenced by `Anim*.bin`. `Antx256.bin` has 18,856 records (377,120 bytes); `Antx1024.bin` has 1,433 records (28,660 bytes).

No header. Records of 20 bytes (5 x u32):

| Offset | Type | Field | Note |
|---|---|---|---|
| 0x00 | u32 | `index` | Equal to the record's position (0, 1, 2, ...). |
| 0x04 | u32 | `width` | Width in pixels (5 to 508). |
| 0x08 | u32 | `height` | Height in pixels (4 to 376). |
| 0x0C | u32 | `atlasWidth` | Texture dimension (up to 512 in `Antx256`, up to 1024 in `Antx1024`). |
| 0x10 | u32 | `atlasHeight` | Same for the height. |

The last two records of each file repeat the final `index`; the last one has `atlasWidth` and `atlasHeight` equal to 0.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## Blueprints.tbp

Binary blueprint data (94,993 bytes). Internal structure not interpreted.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## desktop.uo1, desktop.nwb and Desktop/*/desktop.uos

Saved client UI state (window position and open state). Sizes: `desktop.uo1` 72 bytes, `desktop.nwb` 164 bytes, `Desktop/common/desktop.uos` 163 bytes. Internal structure not interpreted.

Related files in `Desktop/<profile>/`:

| File | Content |
|---|---|
| `Multicache.dat` | 8-byte text: `ver 5` + CRLF. |
| `chardata.mul` | 35 bytes; begins with the text `ver 1` + LF, followed by uninterpreted binary values. |
| `macros2d.txt`, `skillgrp.mul` | Profile macros and skill groups. |

Reader in Fiddler: not interpreted (hex only in the Files tab)

## sjis2uni.mul

Shift-JIS to Unicode conversion table: 65,536 u16 entries (131,072 bytes), indexed by the 16-bit Shift-JIS code; the value is the Unicode code point (0 when there is no mapping). Example: entry `0x8140` = `0x3000`, entry `0x8143` = `0xFF0C`.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## skillgrp.mul

Grouping of skills by category (338 bytes), little-endian:

| Field | Type | Description |
|---|---|---|
| `count` | i32 | Number of groups (7) |
| `names` | `char[17]` x (`count` - 1) | Names of groups 1 to `count`-1, padded with NUL (`Combat`, `Trade Skills`, `Magic`, `Wilderness`, `Thieving`, `Bard`); group 0 has no stored name |
| `skillGroup` | i32 x 58 | Group index of each skill, in skill id order (values 0 to 6) |

Reader in Fiddler: not interpreted (hex only in the Files tab)

## Multimap.rle

Overview map. Described in [overview-maps.md](overview-maps.md).

## Files described in other documents

| File | Document |
|---|---|
| `Skills.idx` | [index-mul.md](index-mul.md) |
| `Langcode.iff` | [localization.md](localization.md) |
| `Multimap.rle` | [overview-maps.md](overview-maps.md) |
| `Credits.dat`, `Helpindx.dat`, `Version.dat`, `Version.txt`, `Verinfo`, `Obscene.lst`, `uobscene.lst`, `ignore.lst`, `conf_ignore.lst`, `Default.mac`, `Arttable.cfg`, `Login.cfg`, `uo.cfg`, `Uoreg.cfg`, `Uomax.cfg`, `Uoupg.cfg`, `chardata.mul` | [text-tables.md](text-tables.md) |

## Log and installation files

| File | Format | Function |
|---|---|---|
| `vercfg.cfg` | text, one value per line | Patcher parameters (patch folder, server, window, URL) |
| `telltext.cfg` | text | Empty |
| `regfile.reg` | UTF-16 text (Registry Editor 5.00) | Installation registry keys |
| `patchlog` | text | Log of patch download sessions |
| `Uoreport0.log` | text | Error log (empty) |
