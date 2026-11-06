# Client fixed tables

Fixed-size and fixed-structure table files: tile data, hues, radar colors, tile animation, fonts and palette. Little-endian in all fields. The generic index files (art, gumps, skills etc.) are described in [index-mul.md](index-mul.md).

## Color format (16 bits) and hue application

Purpose: color format used by art, gumps, textures, fonts, hues and animation palettes.

16-bit value, A1R5G5B5:

| Bits | Field |
|---|---|
| 15 | alpha (1 = opaque) |
| 14 to 10 | red (0 to 31) |
| 9 to 5 | green (0 to 31) |
| 4 to 0 | blue (0 to 31) |

Converting a 5-bit channel to 8 bits: `v8 = floor(v5 * 255 / 31)`. Converting 8 bits to 5 bits: `v5 = floor(v8 * 31 / 255)`.

Notes:

- In data files bit 15 is always 0 and image pixels are never 0. The reader sets bit 15 when converting (`color | 0x8000`, or `color ^ 0x8000` for item art, gumps and fonts, which gives the same result when the bit is 0).
- A value of 0 after conversion means a transparent pixel. When converting to RGBA, `0` becomes `(0, 0, 0, 0)`; any other value becomes an opaque RGB.
- When converting from RGBA to 16 bits, alpha below 128 becomes 0; if the three 5-bit channels are all 0 the color becomes `0x8001` (so it does not collide with transparent); otherwise bit 15 is set.

Applying a hue to an image:

1. The hue id `n` (1 to 3000) uses record `n - 1` of `hues.mul`; hue 0 means "no hue".
2. For each pixel that is not transparent and not black (RGB different from 0, 0, 0): `index = floor(R8 * 31 / 255)` (the red channel converted to 5 bits); the new RGB is color `index` of the hue's 32-color palette.
3. With the "gray only" mode (items with the `PartialHue` flag in `tiledata.mul`), step 2 applies only to pixels where `R = G = B`.

Reader in Fiddler: `crates/uo_core/src/color.rs` (`color16_to_rgba`, `rgba_to_color16`, `apply_hue`)

## tiledata.mul

Purpose: flags, name and attributes of each terrain tile and each item.

Structure: two sections in sequence, with no file header.

1. Terrain: 512 groups of 32 records (16384 tiles, ids 0 to 0x3FFF).
2. Items: groups of 32 records until the end of the file (item ids `0` to `0xFFFF`; the art is `0x4000 + id`).

Each group starts with a `u32` header (value 0 in the reference copies, ignored) followed by the 32 records.

### Versions and how to detect them

| Version | Flags | Terrain record | Item record | Terrain group | Item group | Terrain section |
|---|---|---|---|---|---|---|
| classic | u32 | 26 bytes | 37 bytes | 836 bytes | 1188 bytes | 428032 bytes |
| High Seas (client 7.0.9 onward) | u64 | 30 bytes | 41 bytes | 964 bytes | 1316 bytes | 493568 bytes |

File sizes: classic with 0x10000 items = 2861056 bytes; High Seas with 0x10000 items = 3188736 bytes (493568 of terrain + 2048 groups of 1316 bytes).

Detection used by the reader: High Seas when the file size is greater than or equal to 3188736 bytes; otherwise classic. The number of items is `(size - terrain_section) / group_size * 32`. The reference file (3188736 bytes) is High Seas with 65536 items.

### Terrain record

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 (classic) or u64 (High Seas) | `flags` | bits from the table below |
| 4 or 8 | u16 | `texture_id` | texture id in `texidx.mul`/`texmaps.mul`; 0 = no texture |
| 6 or 10 | char[20] | `name` | ASCII name, zero-terminated |

### Item record

`F` is the size of the `flags` field: 4 bytes (classic) or 8 bytes (High Seas).

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 or u64 | `flags` | bits from the table below |
| F + 0 | u8 | `weight` | weight; 255 on many fixed scenery items (e.g. `hedge`) |
| F + 1 | u8 | `quality` | on items with `Wearable`, the equipment layer (1 to 23 in the reference copies; e.g. `chainmail tunic` = 13, `backpack` = 21) |
| F + 2 | u16 | `misc_data` | auxiliary data (not interpreted) |
| F + 4 | u8 | `unk2` | not interpreted |
| F + 5 | u8 | `quantity` | not interpreted |
| F + 6 | u16 | `anim_id` | equipment animation id (body of the `EQUIPMENT` type in `mobtypes.txt`; e.g. `chainmail tunic` = 538); 0 if none |
| F + 8 | u8 | `unk3` | not interpreted |
| F + 9 | u8 | `hue` | not interpreted |
| F + 10 | u8 | `stacking_offset` | not interpreted |
| F + 11 | u8 | `value` | not interpreted |
| F + 12 | u8 | `height` | item height in units of `z` |
| F + 13 | char[20] | `name` | ASCII name, zero-terminated |

Record size: `F + 33` = 37 (classic) or 41 (High Seas).

Names may contain client formatting markers (`%s`, e.g. `board%s`).

Notes:

- In the reference file, terrain record 0 has its name shifted by 4 bytes (the text `UNUSED` appears at offset 10 of the file); read with the pattern above, terrain 0 comes out with meaningless flags and `texture_id` and the name `ED`. The other records follow the pattern.
- Examples (High Seas): item `0x0EED` = `gold coin`, flags `0x4000801`; item `0x0E75` = `backpack`, flags `0x604002`.

### Flag bits

Names used by Fiddler. Bits 32 to 37 exist only in the High Seas version.

| Bit | Mask | Name |
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

Fields not interpreted: `Unknown1`, `Unknown2`, `Unknown3`, `misc_data`, `unk2`, `quantity`, `unk3`, `hue`, `stacking_offset`, `value`.

Reader in Fiddler: `crates/uo_core/src/tiledata.rs` (`TileDataManager::from_file`); bit names in `crates/uo_gui/src/main.rs` (`TILEDATA_FLAGS`)

## hues.mul

Purpose: 32-color palettes used to tint art, animations and text.

Structure: 375 groups, 8 hues per group, 3000 hues in total (265500 bytes = 375 * 708).

Group (708 bytes):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 | `header` | 0 in the reference copies; ignored |
| 4 | record[8] | `hues` | 8 records of 88 bytes |

Hue record (88 bytes):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u16[32] | `colors` | 32 colors of 16 bits (see "Color format"), from dark to light |
| 64 | u16 | `table_start` | equal to `colors[0]` in all 3000 records |
| 66 | u16 | `table_end` | is not the last color; in the 1000 named records `Hue (a->b)`, `a` and `b` are `table_start` and `table_end` |
| 68 | char[20] | `name` | ASCII name, zero-terminated; empty in 963 records |

Notes:

- The hue id used in the game is 1-based: hue `n` is the record at index `n - 1` in the file; 0 means "no hue". Record 2999 (hue 3000) and 962 other records have an empty name.

Fields not interpreted: how the client uses `table_start` and `table_end`.

Reader in Fiddler: `crates/uo_core/src/hues.rs` (`HuesManager::from_file`, `Hue::get_rgba_palette`)

## radarcol.mul

Purpose: color of each tile on the minimap (radar).

Structure: sequence of `u16` values with no header, one 16-bit color per art index (see [index-mul.md](index-mul.md)): indices `0` to `0x3FFF` for terrain and `0x4000 + id` for items. The reference copy has 81884 values (163768 bytes), the same number of entries as `artidx.mul`. The reader pads with zeros up to 0x14000 values.

Terrain color `t`: `colors[t & 0x3FFF]`. Item color `i`: `colors[0x4000 + i]`.

Reader in Fiddler: `crates/uo_core/src/radar_col.rs` (`RadarColManager`)

## animdata.mul

Purpose: animation sequence for animated items (campfires, torches, water). Associated with the `Animated` flag of `tiledata.mul`.

Structure: groups of 548 bytes, each with 8 records; the item tile of a record is `group * 8 + position` (item id, without the `0x4000`). The reference file has 8187 complete groups (4486476 bytes) plus 272 trailing zeroed bytes, which the reader ignores.

Group:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 | `header` | 0 in the reference copies |
| 4 | record[8] | `entries` | 8 records of 68 bytes |

Record (68 bytes):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | i8[64] | `frames` | id offsets: frame `k` uses the art of item `id + frames[k]` |
| 64 | u8 | `unk` | not interpreted (0 in 65277 of 65496 records; the other values are small) |
| 65 | u8 | `frame_count` | number of frames used in `frames`; 0 = item without animation |
| 66 | u8 | `frame_interval` | interval between frames (unit not established) |
| 67 | u8 | `frame_start` | delay before starting (unit not established) |

Example: item 109 has `frames = -3, -2, -1, 0, 1, 2` and `frame_count = 6`, cycling through items 106 to 111. The reader keeps only the records with `frame_count > 0` (766 in the reference file).

Fields not interpreted: `unk`, the unit of `frame_interval` and `frame_start`.

Reader in Fiddler: `crates/uo_core/src/animdata.rs` (`AnimDataManager::from_file`)

## animinfo.mul

4000 bytes: 2000 `u16` values, all equal to `0x0204` in the reference file.

Fields not interpreted: the whole file.

Reader in Fiddler: not interpreted (hex only, in the Files tab)

## chardata.mul

20 bytes: the ASCII text ` ver 1` followed by LF (7 bytes) and 13 bytes of data: `b2 02 b2 02 b2 02 b2 02 b2 02 34 00 25` (five `u16` values equal to `0x02B2`, one `u16` `0x0034` and one byte `0x25`).

Fields not interpreted: the 13 data bytes.

Reader in Fiddler: not interpreted (hex only, in the Files tab)

## palette.mul

768 bytes: 256 entries of 3 bytes (values 0 to 255). The order of the color channels has not been established.

Reader in Fiddler: not interpreted (hex only, in the Files tab)

## fonts.mul (ASCII fonts)

Purpose: 10 bitmap fonts of 224 characters (codes `0x20` to `0xFF`), used in client text.

Structure: 10 fonts in sequence, with no file header, in 884766 bytes (the reference file has 143 extra bytes after font 9, not interpreted).

Font:

| Type | Field | Meaning |
|---|---|---|
| u8 | `header` | 1 in all reference fonts |
| glyph[224] | `glyphs` | glyphs for codes `0x20 + i` |

Glyph:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u8 | `width` | width in pixels |
| 1 | u8 | `height` | height in pixels |
| 2 | u8 | `unk` | 3, 4 or 5 (not interpreted) |
| 3 | u16[width*height] | `pixels` | 16-bit colors, row by row; 0 = transparent |

Glyphs with `width = 0` or `height = 0` have no pixels (and `pixels` takes no bytes). In the reference copy, all 224 glyphs of all fonts have non-zero dimensions. Maximum heights of glyphs `0x20` to `0x7F`: 21 (fonts 0, 2, 3, 4, 6, 7, 8), 15 (fonts 1 and 9) and 25 (font 5).

Fields not interpreted: `unk` of each glyph and the trailing bytes of the file.

Reader in Fiddler: `crates/uo_core/src/fonts.rs` (`FontsManager::from_file`, `AsciiFont`)

## unifont*.mul (Unicode fonts)

Purpose: 13 Unicode bitmap fonts with 1 bit per pixel.

Files: `unifont.mul` (font 0) and `unifont1.mul` to `unifont12.mul`.

Structure:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32[0x10000] | `offsets` | position in the file of the glyph for each Unicode code point (BMP); 0 = glyph absent |
| 0x40000 | glyphs | data | glyphs in sequence |

Glyph at position `offsets[c]`:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | i8 | `x_offset` | horizontal offset of the bitmap relative to the cursor position |
| 1 | i8 | `y_offset` | vertical offset of the bitmap, measured from the top of the text line |
| 2 | u8 | `width` | bitmap width |
| 3 | u8 | `height` | bitmap height |
| 4 | u8[height * ceil(width / 8)] | `bits` | `height` rows of `ceil(width / 8)` bytes; 1 bit per pixel, most significant bit first; 1 = filled pixel |

Notes:

- The baseline of Latin letters is around `y = 14` (the bottom of `I` is at `y_offset + height = 14`; descenders such as `g` reach 16).
- Absent glyphs (`offsets[c] = 0`) are not drawn. The space (`0x20`) is absent in the reference fonts; the reader uses a fixed advance of 8 pixels for the space.
- The file does not store the horizontal advance. Fiddler uses `max(x_offset, 0) + width + 1` pixels as the advance of each glyph, and the line height is the baseline of the set `0x21` to `0x7E` plus 2.
- The files `unifont.mul` to `unifont2.mul` have 36347 to 36349 glyphs; `unifont3.mul` to `unifont12.mul` have 65472. No offset equals `0xFFFFFFFF`. The reader discards glyphs whose bit area extends past the end of the file.

Reader in Fiddler: `crates/uo_core/src/fonts.rs` (`UnicodeFont`)
