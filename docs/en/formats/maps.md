# Maps (facets)

The world is divided into numbered facets (`mapN`). Each facet is a grid of blocks of 8 x 8 cells. Three files describe a facet: the terrain (`mapN.mul`), the static index (`staidxN.mul`) and the statics (`staticsN.mul`). Diff files (`mapdif`, `stadif`) replace whole blocks. Little-endian in all fields.

## Facets and dimensions

| N | Name | Width (cells) | Height (cells) | Blocks (W x H) | Total blocks | Size of `mapN.mul` |
|---|---|---|---|---|---|---|
| 0 | Felucca | 7168 | 4096 | 896 x 512 | 458752 | 89915392 |
| 1 | Trammel | 7168 | 4096 | 896 x 512 | 458752 | 89915392 |
| 2 | Ilshenar | 2304 | 1600 | 288 x 200 | 57600 | 11289600 |
| 3 | Malas | 2560 | 2048 | 320 x 256 | 81920 | 16056320 |
| 4 | Tokuno | 1448 | 1448 | 181 x 181 | 32761 | 6421156 |
| 5 | Ter Mur | 1280 | 4096 | 160 x 512 | 81920 | 16056320 |

The size of `mapN.mul` is `blocks * 196`. The height is fixed per facet; the width in blocks is `file_size / 196 / (height / 8)`. The reader uses the height from the table and computes the width from the file size.

Coordinates: the cell `(x, y)` belongs to the block `(bx, by) = (x >> 3, y >> 3)`, and is the cell with index `(y & 7) * 8 + (x & 7)` within it (row by row).

Block number (used in `mapN.mul`, `staidxN.mul` and in the list files of the diffs):

```
number = bx * (height / 8) + by
```

That is, the blocks are ordered column by column (all the `by` values of the same `bx` in sequence).

## mapN.mul (terrain)

Purpose: terrain tile and height of each cell.

File: `map0.mul` to `map5.mul`. A sequence of 196-byte blocks, with no file header; block `number` is at `number * 196`.

Block (196 bytes):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u32 | `header` | in the reference copies it is equal to the block number; the reader ignores it |
| 4 | cell[64] | `cells` | 64 cells of 3 bytes, in the order described above |

Cell (3 bytes):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u16 | `tile_id` | terrain id (0 to 0x3FFF; art and flags in `art.mul`/`tiledata.mul`) |
| 2 | i8 | `z` | height (-128 to 127) |

Reader in Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::load_map`, `land_block`)

## staidxN.mul and staticsN.mul (statics)

Purpose: fixed world items (trees, walls, buildings) per block.

`staidxN.mul` has one 12-byte entry per block, in block-number order (`number * 12`). It is the generic index described in [index-mul.md](index-mul.md):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | i32 | `lookup` | position of the block's records in `staticsN.mul`; `-1` = block without statics |
| 4 | i32 | `length` | bytes used (multiple of 7); `-1` = no statics |
| 8 | i32 | `extra` | not used (in the reference copies: -1 in an empty block, `0xAAAAAAAA` in a block with data) |

The size of `staidxN.mul` is `blocks * 12` (e.g. facet 0, 5505024 bytes). A valid entry is `lookup >= 0` and `length > 0`; any other combination is an empty block.

`staticsN.mul` contains, with no header, the 7-byte records of each block, one block after another:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u16 | `item_id` | item id (art = `0x4000 + item_id`) |
| 2 | u8 | `x` | column within the block (0 to 7) |
| 3 | u8 | `y` | row within the block (0 to 7) |
| 4 | i8 | `z` | height |
| 5 | u16 | `hue` | hue; 0 = no hue (see [tables.md](tables.md)) |

Absolute position of the static: `X = bx * 8 + x`, `Y = by * 8 + y`. Number of records in the block: `length / 7`. A block may have several records in the same cell. In the reference facet 0 there are 2848162 records, all with `x` and `y` between 0 and 7.

Reader in Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::static_block`)

## Diff files

Diff files carry replacement blocks. When a block is listed, the block from the base file is discarded and the one from the diff is used in full. They exist for facets 0, 1 and 2 in the reference copy.

### mapdifN.mul and mapdiflN.mul (terrain)

- `mapdifN.mul`: a sequence of 196-byte blocks in the same format as `mapN.mul` (4 bytes of header with no known meaning, followed by 64 cells).
- `mapdiflN.mul`: a list of `u32`, one per block of `mapdifN.mul`. The value at position `i` is the block number (formula above) that block `i` of `mapdifN.mul` replaces.

The number of `u32` values in `mapdiflN.mul` equals `size(mapdifN.mul) / 196` (facet 0: 1405; facet 1: 8034; facet 2: 268). The block numbers are unique and smaller than the total number of blocks of the facet. In `mapdif0.mul` the list is not in ascending order.

### stadifN.mul, stadifiN.mul and stadiflN.mul (statics)

- `stadifN.mul`: 7-byte static records, in the format of `staticsN.mul`.
- `stadifiN.mul`: index with 12-byte entries (`lookup`, `length`, `extra`), one per replacement block, pointing into `stadifN.mul`.
- `stadiflN.mul`: a list of `u32` (number of the replaced block); the value at position `i` corresponds to entry `i` of `stadifiN.mul`.

`stadifiN.mul` and `stadiflN.mul` have the same number of entries (facet 0: 1807; facet 1: 10931; facet 2: 268). An entry with `lookup = -1` or `length = 0` means the block ends up with no statics.

To read a block with the diffs active: search for the block number in the list (`mapdifl`/`stadifl`); if it exists at position `i`, use block `i` of the diff file; otherwise use the base file. The reader allows enabling and disabling the use of the diffs.

Fields not interpreted: the 4-byte `header` of each block of `mapdifN.mul` and the `extra` field of `stadifiN.mul`.

Reader in Fiddler: `crates/uo_core/src/maps.rs` (`MapManager::load_diff`, `land_block`, `static_block`, `diff_counts`)

## "x" variants (map0x.mul, map1x.mul, staidx0x.mul, staidx1x.mul, statics0x.mul, statics1x.mul)

They exist for facets 0 and 1 and have exactly the same structure as the files without the suffix (`map0x.mul` 89915392 bytes, `staidx0x.mul` 5505024 bytes). They differ from the base set in content: `map0x.mul` has 5245 blocks that differ from `map0.mul`, and `map1x.mul` has 156 that differ from `map1.mul`; `statics0x.mul` has 2889031 records (versus 2848162 in `statics0.mul`).

Fields not interpreted: purpose of the variants. There are no corresponding `mapdif`/`stadif` files.

Reader in Fiddler: not interpreted (hex only, in the Files tab)

## map0.txt

Text file with CRLF line endings (some lines end only in LF). Each line has the form `x,y,z` (decimal integers), for example `0,0,-5`. It contains 482505 lines, all with distinct coordinates of facet 0 (`x` from 0 to 119, `y` from 0 to 4095). The `z` value of each line matches the `z` of the corresponding terrain cell in `map0.mul` (482505 of 482505 lines), and about 99.8% of the cells have terrain id 168 to 171.

Fields not interpreted: purpose of the file (the selection of the listed cells has not been determined).

Reader in Fiddler: not interpreted (hex only, in the Files tab)
