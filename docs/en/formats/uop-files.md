# Client UOP Files

All files below use the container described in `uop-container.md`. The offsets on this page refer to the entry content already decompressed. Little-endian.

## AnimationFrame1.uop to AnimationFrame4.uop

### Purpose

Animation frames for creatures and equipment (palettized format, equivalent to the classic `anim*.mul`). Each entry contains all frames of all directions of one action of one body.

### Entries

Logical name: `build/animationlegacyframe/{body:06}/{action:02}.bin`.

| File | Entries | Distinct bodies | Body range |
|---|---:|---:|---|
| AnimationFrame1.uop | 287 | 12 | 130 to 1026 |
| AnimationFrame2.uop | 3911 | 87 | 323 to 1247 |
| AnimationFrame3.uop | 2791 | 72 | 655 to 1068 |
| AnimationFrame4.uop | 318 | 17 | 735 to 832 |

A (body, action) pair appears in only one of the four files (7307 pairs in total). Actions range from 0 to 78. All entries are zlib.

### Entry header

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | char[4] | magic | `"AMOU"` |
| 4 | u32 | version | 1 |
| 8 | u32 | total_size | size of the decompressed entry |
| 12 | u32 | body | body id (same as `{body}` in the name) |
| 16 | i16[4] | - | not interpreted |
| 24 | u32 | extra_count | number of u32 values in the extra block (256 in most entries; smaller values such as 1 also appear) |
| 28 | u32 | header_size | 40 |
| 32 | u32 | frame_count | number of frame headers |
| 36 | u32 | data_start | `40 + 4 * extra_count` |
| 40 | u32[extra_count] | extra | extra block; not interpreted |
| data_start | frame_header[frame_count] | frames | frame headers, 16 bytes each |

### Frame header (16 bytes)

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u16 | action | action id (same as `{action}` in the name) |
| 2 | u16 | frame_id | frame number, starting at 1 |
| 4 | i16[4] | - | not interpreted |
| 12 | u32 | pixel_offset | distance, from the start of this 16-byte header, to the frame data |

Position of the frame data: `data_start + 16 * i + pixel_offset`, for the i-th header. The last frame ends exactly at the end of the entry.

### Frame data

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u16[256] | palette | 256-color palette |
| 512 | i16 | center_x | horizontal anchor |
| 514 | i16 | center_y | vertical anchor |
| 516 | u16 | width | width in pixels |
| 518 | u16 | height | height in pixels |
| 520 | run[] | runs | sequence of pixel runs |

When `width == 0` or `height == 0` the frame has only 520 bytes (palette and dimensions), with no runs and no terminator.

Each run (`run`):

| Type | Field | Meaning |
|---|---|---|
| u32 | header | raw value; `0x7FFF7FFF` ends the frame |
| u8[n] | indices | `n` palette indices |

For a header other than `0x7FFF7FFF`: `h = header ^ 0x80200000`, then

- `row_x = (h >> 22) & 0x3FF`
- `row_y = (h >> 12) & 0x3FF`
- `n = h & 0xFFF`

Position of the first pixel of the run, in coordinates of the `width x height` image:

- `x = center_x - 0x200 + row_x`
- `y = center_y + height - 0x200 + row_y`

The following `n` pixels occupy `x .. x+n-1` on row `y`.

Color: `palette[idx]`. The value 0 is transparent; any other value is A1R5G5B5 with bit 15 forced to 1 (R in bits 10-14, G in bits 5-9, B in bits 0-4).

### Directions and frame order

- Each entry stores 5 directions (0 to 4) in sequence. The direction of a frame is `(frame_id - 1) / frames_per_direction`.
- `frames_per_direction = round(N / 5)`, where N is the number of frames, counting missing ids between two present ids as empty.
- Directions 5, 6 and 7 reuse directions 3, 2 and 1 mirrored horizontally: pixel `x` becomes `width - 1 - x` and the anchor becomes `width - center_x`.

### Uninterpreted fields

- The four i16 values at offset 16 of the header and at offset 4 of each frame header.
- The `extra` block (`extra_count` u32 values).

Reader in Fiddler: `uo_core::animations` (`UopAnimationIndex`, `decode_animation_uop`).

## AnimationSequence.uop

### Purpose

Table of animation groups per body: indicates which actions exist and which are replaced by another action.

### Entries

Logical name: `build/animationsequence/{id:08}.bin`. 306 entries, ids from 0 to 1247 (sparse). `{id}` is the body. All in zlib.

### Header

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u32 | body | body id (same as the name in 305 of the 306 entries) |
| 4 | u8[48] | - | not interpreted (zeros in most entries) |
| 52 | i32 | group_count | number of group records (29 in 283 entries; also 31, 32, 48, 68) |
| 56 | group[group_count] | groups | group records |

### Group record (variable length)

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | i32 | group | action number; usually sequential, 0, 1, 2, ... |
| 4 | i32 | frame_count | frames in the group; 0 when the action is replaced |
| 8 | i32 | replacement | action that replaces this one; -1 when there is none |
| 12 | f32 | - | 9.0 in most; also 12.0, 6.0 and 0.0 |
| 16 | u8[48] | - | not interpreted |
| 64 | i32 | count_a | number of records in the first sub-block |
| 68 | u8[32 * count_a] | a | 32-byte records; not interpreted |
| 68 + 32 * count_a | i32 | count_b | number of records in the second sub-block |
| +4 | u32[count_b] | b | 4-byte values; not interpreted |

`count_a` is 0 in almost all groups (2 or 4 in 16 groups); `count_b` is 0 or 1.

Replacement rule: `frame_count == 0` occurs exactly when `replacement != -1`. To get the frame file for a body and an action, replace `action` with `replacement` and use the name `build/animationlegacyframe/{body:06}/{replacement:02}.bin`. Without a replacement record the action stays the same.

After the last group there are additional bytes (from 245 to 1353 in the examples); not interpreted.

### Uninterpreted fields

Header bytes 4 to 51, the f32 and the 48 bytes that follow it in each group, sub-blocks `a` and `b`, and the trailing bytes.

Reader in Fiddler: `uo_core::animations` (`UopAnimationIndex::load_sequence_uop`, `get_resolved_action`).

## MainMisc.uop

### Purpose

Miscellaneous client tables. Contains 4 entries, with no established naming pattern, identified by `name_hash`.

| # | name_hash | Decompressed size | Content |
|---:|---|---:|---|
| 0 | `0x6B1786D68753F504` | 5013 | resource map (XML) |
| 1 | `0xC0165B63D153B1DD` | 273936 | table of present resources (8-byte records) |
| 2 | `0x0891F809004D8081` | 193668 | UTF-16 text table (speech keywords) |
| 3 | `0x021C35A18C8BC953` | 8875 | equipment draw order (XML) |

### Entry 0: resource map

XML text (UTF-8, CRLF line terminator) with root `<assetmap>`. Describes, for each resource type (`resourcetype`), where the resource is in loose format (`fileFormat="raw"`, directory `Data/...`) and in packed format (`fileFormat="packed"`, directory `Build/...`).

Elements:

| Element | Attributes | Meaning |
|---|---|---|
| `patternmap` | `resourcetype`, `fileFormat`, `padding`, `directory`, `suffix`, `patternMatch` | name = `directory` + number with `padding` digits + `suffix` |
| `bitmaskmap` | `resourcetype`, `fileFormat`, `directory`, `suffix`, with `section` children | the resource number is split into parts by bit mask; each `section` (`mask`, `padding`, `prefix`, `isDirectory`) produces a piece of the path |
| `simplemap` | `resourcetype`, `fileFormat`, `resourcenumber`, `filename` | fixed path of a single resource |

Resource types and packed folder:

| resourcetype | Packed folder | Suffix |
|---:|---|---|
| 1 | `Build/WorldArt/` | `.dds` |
| 3 | `Build/TerrainDefinition/` | `.bin` |
| 6 | `Build/TileArt/` | `.bin` |
| 8 | `Build/TerrainTexture/` | `.dds` |
| 9 | `Build/Sectors/` (masks `0x3f` and `0xffffffc0`) | `.bin` |
| 11 | `Build/AnimationDefinition/` | `.bin` |
| 12 | `Build/AnimationSequence/` | `.bin` |
| 13 | `Build/EffectDefinitionCollection/` | `.bin` |
| 16 | `Build/AnimationFrame/` (masks `0xffff00` and `0xff`) | `.bin` |
| 25 | `Build/AnimationLegacyFrame/` (masks `0xffff00` and `0xff`) | `.bin` |
| 18 | `Build/Paperdoll/` (masks `0xffffc0` and `0x3f`) | `.bin` |
| 19 | `Build/MultiCollection/` | `.bin` |
| 20 | `Build/MultiCollection/housing.bin` | (fixed file) |
| 21 | `Build/Sectors/waypoint.bin` | (fixed file) |
| 22 | `Build/TileArtLegacy/` | `.dds` |
| 23 | `Build/FacetDefinition/` | `.bin` |
| 26 | `Build/TileArtEnhanced/` | `.dds` |

In `AnimationLegacyFrame`, the resource number is `(body << 8) | action`: the mask `0xffff00` gives the directory (`{body:06}`) and `0xff` gives the file (`{action:02}`), which results in `build/animationlegacyframe/{body:06}/{action:02}.bin`.

### Entry 1: table of present resources

Sequence of 8-byte records, with no header (34242 records).

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u64 | resource | `(resourcetype << 56) | number` |

The high byte is the `resourcetype` from the resource map; the low 56 bits are the resource number. Count per type:

| resourcetype | Records | Correspondence |
|---:|---:|---|
| 3 | 197 | terrain definitions |
| 6 | 32648 | numbers identical to the entries of `tileart.uop` |
| 11 | 767 | animation definitions |
| 12 | 306 | numbers identical to the entries of `AnimationSequence.uop` |
| 19 | 324 | multi collections |

The records are sorted by number within each type, except for type 3.

### Entry 2: text table

UTF-16LE text with BOM (`FF FE`), lines terminated by CRLF (6125 lines). Each line:

```
{id decimal}<TAB>*{text}*
```

The same `id` may repeat across several lines (synonyms). The texts are speech keywords in several languages, for example `0 *提領*`, `4 *加入*`, `168 *wo ist nujel*`.

### Entry 3: equipment draw order

XML text (UTF-8) with root `<wearables>`.

| Element | Attributes | Meaning |
|---|---|---|
| `<draworder>` | one numeric priority per layer (`riding`, `face`, `finger1`, `ears`, `feet`, `legs`, `torso`, `talisman`, `neck`, `chest`, `abovechest`, `arms`, `hands`, `lwrist`, `skirt`, `dress`, `waist`, `feetlegs`, `cape`, `hair`, `facialhair`, `head`, `backpack`, `righthand`, `lefthand`, `drag`); optional `direction` | numeric draw priority of each layer. The second block (`direction="paperdoll"`) applies to the paperdoll |
| `<wearable>` | one layer with a comma-separated list of item ids, or `any` | selects the item that receives rules |
| `<override>` | `draworderpriority`, optional `direction`, and layers with a list of ids or `any` | when the listed layers are equipped (and the direction matches), the item of the parent `<wearable>` starts using `draworderpriority`; the value 0 means do not draw |

The directions used in `direction` are `south`, `southeast`, `east`, `northeast`, `north`, `northwest`, `west`, `southwest` and `paperdoll`.

Reader in Fiddler: `uo_core::paperdoll` (`<draworder>` blocks); the other entries: not interpreted (hex only in the Files tab).

## string_dictionary.uop

### Purpose

Indexed list of strings used by the client (shader names and texture paths).

### Entries

One entry: `build/stringdictionary/string_dictionary.bin` (2989218 bytes decompressed, zlib).

### Layout

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u64 | timestamp | Windows FILETIME (100 ns intervals since 1601-01-01); 2012-01-20 04:30:10 in the example |
| 8 | u32 | count | 93404 |
| 12 | u32 | max_length | 72, equal to the size of the longest string |
| 16 | string[] | strings | sequence up to the end of the entry |

Each string: `u16 length` followed by `length` ASCII bytes, with no terminator. The example contains 93403 strings, none empty; the first is `UOSpriteShader`, followed by paths such as `Data\WorldArt\00008600_NO_DRAW.tga` and `Data\TileArtLegacy\0.tga`.

### Uninterpreted fields

The one-unit difference between `count` (93404) and the number of strings present (93403).

Reader in Fiddler: not interpreted (hex only in the Files tab).

## tileart.uop

### Purpose

Binary descriptors indexed by id, in two ranges (0 to 39212 and 10000000 to 10001247).

### Entries

32648 entries, all zlib, logical name `build/tileart/{id:08}.bin`:

| Id range | Entries |
|---|---:|
| 0 to 39212 (sparse) | 31706 |
| 10000000 to 10001247 (sparse) | 941 |
| 99999999 | 1 |

The ids of these entries match the type 6 records of the resource table in `MainMisc.uop`. The decompressed size ranges from 241 to 1069 bytes. The entries contain no image.

### Entry header

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u16 | version | 3 in all entries |
| 2 | u32 | - | not interpreted (varies per entry) |
| 6 | u32 | id | same as `{id}` in the name for 32644 of the 32647 numbered entries (in the 10000000+ range it is the id itself, without subtraction) |
| 10 | - | - | rest of the structure, with variable size; contains floats (1.0 and 1.5 appear right after the header) |

### Uninterpreted fields

Everything after offset 10; the field at offset 2; the 3 entries whose field at offset 6 differs from the id in the name.

Reader in Fiddler: not interpreted (hex only in the Files tab).

## Legacy content UOPs

These files pack the content of the classic `.mul`/`.idx` files. They do not exist in the sample directory; the layout below is what Fiddler's readers and converter implement.

### artLegacyMUL.uop

- Name: `build/artlegacymul/{i:08}.tga`, `i` from 0 to `0x13FFF`.
- Payload: exactly the bytes that `art.mul` stores for index `i` (terrain from 0 to `0x3FFF`, statics starting at `0x4000`).
- Mapping to MUL/IDX: `artidx.mul` + `art.mul`. Record of `artidx.mul` (12 bytes): `i32 offset`, `i32 length`, `i32 extra`. Missing entry = `-1, -1, -1`. The `extra` field is written as 0.

### gumpartLegacyMUL.uop

- Name: `build/gumpartlegacymul/{i:08}.tga`, `i` from 0 to `0xFFFF`.
- Payload: `u32 width`, `u32 height`, followed by the bytes that `gumpart.mul` stores for index `i`.
- Mapping to MUL/IDX: `gumpidx.mul` + `gumpart.mul`. In `gumpidx.mul` the `extra` field is `(width << 16) | height`; the two values come from the start of the UOP payload.

### soundLegacyMUL.uop

- Name: `build/soundlegacymul/{i:08}.dat`.
- Maps `soundidx.mul` + `sound.mul`. The converter does not handle this file, because the UOP entries do not carry the `extra` field of the classic index.

### map0LegacyMUL.uop to map5LegacyMUL.uop

- Name: `build/map{N}legacymul/{i:08}.dat`.
- Payload: consecutive chunks of `map{N}.mul` of `0xC4000` bytes (802816 = 4096 blocks of 196 bytes); the last chunk may be smaller.
- Mapping: concatenating the chunks in order of `i` reproduces `map{N}.mul`. There is no `.idx`.

Size of the sample `map*.mul` files, in chunks of `0xC4000`:

| File | Bytes | Chunks | Dimensions |
|---|---:|---:|---|
| map0.mul | 89915392 | 112 | 7168 x 4096 |
| map1.mul | 89915392 | 112 | 7168 x 4096 |
| map2.mul | 11289600 | 14.06 | 2304 x 1600 |
| map3.mul | 16056320 | 20 | 2560 x 2048 |
| map4.mul | 6421156 | 7.998 | 1448 x 1448 |
| map5.mul | 16056320 | 20 | 1280 x 4096 |

### MultiCollection.uop

- Name: `build/multicollection/{id:06}.bin` and `build/multicollection/housing.bin`.
- Payload of each multi:

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u32 | multi_id | multi id |
| 4 | u32 | count | number of components |
| 8 | component[count] | components | components, 14 bytes plus extra data |

Component:

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u16 | item_id | item id |
| 2 | i16 | x | X offset |
| 4 | i16 | y | Y offset |
| 6 | i16 | z | Z offset |
| 8 | u16 | flags | component flags |
| 10 | u32 | extra | number of u32 values that follow the component |
| 14 | u32[extra] | - | extra data; not interpreted |

The converter does not handle this file.

### GameData.uop

CSV tables (text) of the 3D client. Entries known by `name_hash`:

| name_hash | Table | Columns |
|---|---|---|
| `0xE840D2286751791C` | legacycontainers.csv | gumpId, left, top, right, bottom, name |
| `0x78AFB72A49CD9BE9` | legacy_to_new_items.csv | legacyId, newId, newSub |
| `0xBF2AE37FAFC672E7` | body_to_mesh.csv | BodyType, file, description |
| `0xF45CF6B325F9D01A` | recipes.csv | not interpreted |

Blank lines and lines starting with `#` are comments; the first line is the header.

Reader in Fiddler: `uo_core::gamedata` (GameData); `uo_core::multis` (MultiCollection); `uo_core::file_index` (`from_uop`, for art/gump/map by default `build/{name}/{i:08}{ext}`); the others: `uo_core::client_files`.

## uop_convert tool

Converts between the legacy UOPs and the classic MUL/IDX files.

```
uop_convert to-mul <source-folder> <destination-folder>
uop_convert to-uop <source-folder> <destination-folder>
```

- `to-mul`: reads `artLegacyMUL.uop`, `gumpartLegacyMUL.uop` and `map0..5LegacyMUL.uop` from the source folder and writes `art.mul` + `artidx.mul`, `gumpart.mul` + `gumpidx.mul` and `map{N}.mul`.
- `to-uop`: does the reverse. The generated UOPs have `version = 5`, `block_capacity = 100`, uncompressed entries (`compression = 0`), `header_length = 0` and `data_hash = 0`.
- Supported types: art, gump and map. Sound and multi are not converted.
- File names are compared case-insensitively. Each type found in the folder is converted; at the end the tool prints the count of converted and failed items (exit code 1 if there is an error).

Conversion rules:

- Art: payload copied unchanged; the index records `offset`, `length` and `extra = 0`.
- Gump: the index `extra` is split into `width = extra >> 16` and `height = extra & 0xFFFF` and written as the two leading u32 values of the UOP payload (and the reverse in `to-mul`).
- Map: `to-uop` splits `map{N}.mul` into chunks of `0xC4000`; `to-mul` concatenates the chunks starting at `i = 0` and stops at the first missing entry.
- Index entries with `offset < 0` or `length <= 0` do not generate an entry in the UOP.

Reader in Fiddler: `uo_core::uop_convert` (`uop_to_mul`, `mul_to_uop`); binary `uo_core/src/bin/uop_convert.rs`.
