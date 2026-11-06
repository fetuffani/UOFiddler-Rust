# Client .idx/.mul pairs

Several client resources (art, gumps, sounds, multis, skills, textures, lights and animations) are stored as a `.mul` data file accompanied by an `.idx` index. The index is a table of fixed-size entries; the data file contains the blocks it references. This document describes the generic format of the pair and the contents of each resource. Animations (`anim*.idx`/`anim*.mul`) use the same index format and are covered in [animations.md](animations.md).

Little-endian in all fields. 16-bit colors follow the format described in [tables.md](tables.md) (section "Color format").

## Generic index format

Purpose: locate, by entry number, a block of bytes inside the data file.

Files (index / data):

| Index | Data | Resource |
|---|---|---|
| `artidx.mul` | `art.mul` | terrain and item art |
| `gumpidx.mul` | `gumpart.mul` | gumps (interface and paperdoll) |
| `soundidx.mul` | `sound.mul` | sound effects |
| `multi.idx` | `multi.mul` | multis (houses, boats) |
| `Skills.idx` | `skills.mul` | skill names |
| `texidx.mul` | `texmaps.mul` | terrain textures |
| `lightidx.mul` | `light.mul` | light masks |
| `anim.idx`, `anim2.idx` to `anim5.idx` | `anim.mul`, `anim2.mul` to `anim5.mul` | animations |

Each index entry takes 12 bytes. The number of entries is `idx_size / 12`; entry `n` starts at `n * 12`.

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | i32 | `lookup` | position, in bytes, of the block in the data file |
| 4 | i32 | `length` | size of the block in bytes |
| 8 | i32 | `extra` | auxiliary value, resource-dependent (see each section) |

To read entry `n`: read `length` bytes starting at `lookup` in the data file.

Valid entry: `lookup >= 0` and `length > 0`. Invalid (missing) entry:

- `lookup = -1` (0xFFFFFFFF), with `length` and `extra` also -1 or with arbitrary values (`artidx.mul` has positive `length` in entries with `lookup = -1`);
- or `length = 0`, with any `lookup` (common in `gumpidx.mul`, `texidx.mul` and in the animation indexes).

Notes:

- The entry number is the resource identifier (art id, gump id, sound id, etc.).
- The blocks are contiguous in the data file in the reference copies (the largest `lookup + length` equals the size of the `.mul`), but the reader does not depend on this.
- Fiddler reads at most a fixed number of entries per resource: art `0x14000`, gumps `0xFFFF`, multis `0x4000`, textures `0x4000`, lights 100, sounds `0xFFF`, skills 100.

Reader in Fiddler: `crates/uo_core/src/file_index.rs` (`FileIndex::from_mul_idx`, `IndexEntry::is_valid`, `FileIndex::get_data`)

## Art (artidx.mul / art.mul)

Purpose: bitmaps of terrain (ground tiles) and items (statics).

Id ranges in the index:

| Index range | Contents |
|---|---|
| `0x0000` to `0x3FFF` | terrain (land), 44x44 diamond |
| `0x4000` onward | items (statics); `index = 0x4000 + item id` |

In the reference copy `artidx.mul` has 81884 entries (0x13FDC); 2855 terrain ids and 31682 item ids are valid; the largest valid index is 55596 (item 39212). `extra` is always 0. The file `radarcol.mul` has one color per art index (see [tables.md](tables.md)).

### Terrain (index < 0x4000)

2048-byte entry (1024 u16 words), no header. The first 1012 pixels form the diamond; the last 12 words are zero. Each pixel is a 16-bit color.

Pixel order, row by row, left to right, in a 44x44 image:

| Rows | Columns filled in row `y` | Pixels in the row |
|---|---|---|
| 0 to 21 | `21 - y` to `22 + y` | `2 + 2y` (2, 4, ..., 44) |
| 22 to 43 | `y - 22` to `65 - y` | `44 - 2(y - 22)` (44, 42, ..., 2) |

Positions outside the diamond are transparent.

### Item (index >= 0x4000)

Image with variable width and height, encoded in per-row runs.

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 | `data_size` | size in bytes of the run area (everything after the row table); equal to `length - 8 - 2*height` |
| 4 | u16 | `width` | width in pixels (1 to 1024; maximum observed 180) |
| 6 | u16 | `height` | height in pixels (1 to 1024; maximum observed 591) |
| 8 | u16[height] | `row_lookup` | start of each row, in u16 units, counted from the end of this table |
| 8 + 2*height | runs | data | runs of all rows |

Runs of a row `y` start at word `4 + height + row_lookup[y]` (counted from the start of the entry, in u16):

| Type | Field | Meaning |
|---|---|---|
| u16 | `x_offset` | transparent pixels to skip before the run |
| u16 | `x_run` | number of pixels in the run |
| u16[x_run] | `pixels` | 16-bit colors |

The row ends at the run with `x_offset = 0` and `x_run = 0`. The horizontal position accumulates: `x += x_offset`, then the `x_run` pixels, then `x += x_run`. Pixels not covered are transparent.

Notes:

- The stored pixels have bit 15 at 0 and are never 0; the reader sets bit 15 when converting (see [tables.md](tables.md)).
- The color is applied with a hue when the item has one; see "Hue application" in [tables.md](tables.md).
- All item entries of the reference copy decode without exceeding `width`.

Reader in Fiddler: `crates/uo_core/src/art.rs` (`decode_land`, `decode_static`)

### art.def and Arttable.cfg

- `art.def`: text, redirect `<id> {<new_id>} <hue>`; described in [def-files.md](def-files.md).
- `Arttable.cfg`: text, lines `<id> { <id>[, <id>...] } <value>` with `value` equal to 0 (109 lines) or -1 (1 line, `3 { 3 } -1`). Examples: `15821 { 2140 } 0` and `16927 { 16927, 24968, 24969 } 0`.

Uninterpreted fields: semantics of `Arttable.cfg` and of the `value` field.

Reader in Fiddler: not interpreted (hex only in the Files tab) for `art.def` and `Arttable.cfg`

## Gumps (gumpidx.mul / gumpart.mul)

Purpose: interface and paperdoll images (equipment from 50000 onward, see `Equipconv.def` and `gump.def` in [def-files.md](def-files.md)).

Index `extra` field: `width << 16 | height`, that is, width in the high 16 bits and height in the low 16. Example: gump 0 has `extra = 0x028001E0` (640 x 480).

In the reference copy there are 61244 entries, 3164 valid (largest valid id 61243); the invalid ones have `length = 0`.

Entry (`height` rows):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32[height] | `row_lookup` | start of each row, in 4-byte units, counted from the start of the entry |
| `row_lookup[y] * 4` | runs | row data | sequence of pairs until they add up to `width` pixels |

Each run takes 4 bytes:

| Type | Field | Meaning |
|---|---|---|
| u16 | `color` | 16-bit color; `0` = transparent run |
| u16 | `run` | number of pixels repeated with this color |

The row ends when the sum of the `run` values reaches `width`; there is no terminator. The first row starts at `row_lookup[0] = height` (right after the table). In all 3164 reference entries the sum of the `run` values of each row is exactly `width`.

Notes:

- Width and height come only from `extra`; the entry has no header of its own.
- Reader limit: width and height between 1 and 4096.

Reader in Fiddler: `crates/uo_core/src/gumps.rs` (`decode_gump`)

## Sounds (soundidx.mul / sound.mul)

Purpose: PCM sound effects.

Index: 4096 entries; the entry number is the sound id. For valid entries `extra = id + 1`. In the reference copy there are 1633 valid sounds (largest id 1641). Invalid entries have `lookup = -1` and `length = -1`, but `extra` filled in (`id + 1`).

`sound.mul` entry:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | char[40] | `name` | name of the source file (e.g. `d_frst01.wav`), zero-terminated; the bytes after the zero are residue and must be ignored |
| 40 | i16[] | `pcm` | signed 16-bit PCM samples, mono, up to the end of the entry (`length - 40` bytes, always even) |

Fiddler packs the PCM into a WAV with a 44-byte header at 22050 Hz, mono, 16 bits.

`Sound.def` redirects sound ids (`<id> {<new>} <hue>`); see [def-files.md](def-files.md).

Reader in Fiddler: `crates/uo_core/src/sounds.rs` (`SoundEntry::parse`, `to_wav`)

## Multis (multi.idx / multi.mul)

Purpose: structures composed of several items (houses, boats). The entry number is the multi id; the item is displayed with art `0x4000 + item_id`.

In the reference copy `multi.idx` has 8182 entries and 324 valid (largest id 7667); `extra` is 0.

An entry is a list of components with no header or counter. The number of components is `length / record_size`.

There are two record sizes:

| Version | Size | Fields |
|---|---|---|
| classic | 12 bytes | `item_id`, `x`, `y`, `z`, `flags` |
| High Seas (client 7.0.9 onward) | 16 bytes | the same plus `unk1` |

Record (offsets are the same in both versions up to byte 11):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u16 | `item_id` | item id (art = `0x4000 + item_id`) |
| 2 | i16 | `x` | X offset relative to the multi's origin point |
| 4 | i16 | `y` | Y offset |
| 6 | i16 | `z` | Z offset |
| 8 | u32 | `flags` | observed values: 0 and 1 |
| 12 | u32 | `unk1` | only in the 16-byte record; observed values: 0 and 1 |

How to decide the record size: the size applies to the whole file, not per entry. Go through all valid entries:

- if any `length` is not a multiple of 12, the file uses 16 bytes;
- if any `length` is not a multiple of 16, the file uses 12 bytes;
- if all are multiples of both (or the situation is contradictory), use 16.

Deciding from a single entry is ambiguous for sizes that are multiples of 48 (fitting both 12 and 16). In the reference copy all 324 entries are multiples of 16 and 209 are not multiples of 12, so the record is 16 bytes.

Uninterpreted fields: meaning of the bits of `flags` and of `unk1`.

Reader in Fiddler: `crates/uo_core/src/multis.rs` (`detect_multi_record_size`, `MultiStructure::parse_sized`)

## Skills (Skills.idx / skills.mul)

Purpose: list of skill names. The entry number is the skill id.

Index: 256 entries; 58 valid (ids 0 to 57). `extra` takes values such as 13, 14 and 15 and is not interpreted.

Entry:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u8 | `has_action` | 1 if the skill can be used actively (use button), 0 otherwise |
| 1 | char[] | `name` | ASCII name, zero-terminated; takes up the rest of the entry |

Example: id 0 = `00 "Alchemy" 00`; id 1 = `01 "Anatomy" 00`.

Uninterpreted fields: index `extra`.

Reader in Fiddler: `crates/uo_gui/src/main.rs` (`render_skills_tab`, using `FileIndex`)

## Terrain textures (texidx.mul / texmaps.mul)

Purpose: textures applied to the terrain (3D). The texture id is the `texture_id` field of the terrain record in `tiledata.mul` (see [tables.md](tables.md)).

Index: 16384 entries (0x4000); 2647 valid (largest id 16379). Id 0 is never valid.

`extra` and size:

| `extra` | `length` | Dimension |
|---|---|---|
| 0 | 8192 | 64 x 64 (2330 entries) |
| 1 | 32768 | 128 x 128 (317 entries) |

Entry: `dimension * dimension` u16 words (16-bit colors, bit 15 at 0), row by row, no header. The reader chooses 128 when `extra != 0` or `length >= 32768`.

Reader in Fiddler: `crates/uo_core/src/textures.rs` (`decode_texture`)

## Lights (lightidx.mul / light.mul)

Purpose: light intensity masks (light sources, circles).

Index: 100 entries; 55 valid. `extra = height << 16 | width` (width in the low 16 bits, height in the high). Example: id 0 has `extra = 0x006E006E` (110 x 110) and `length = 12100`.

Entry: `width * height` bytes, one per pixel, row by row. The values are signed 8-bit integers; in the reference copy they range from -30 to 31. In 54 of the 55 entries `length = width * height`; entry 50 declares 280 x 280 in `extra`, but has `length = 90000` (300 x 300).

Notes:

- The Fiddler reader treats each byte as an unsigned integer and converts to gray by multiplying by 8 (saturating at 255); negative values appear saturated.

Uninterpreted fields: physical meaning of the negative values.

Reader in Fiddler: `crates/uo_core/src/light.rs` (`decode_light`)
