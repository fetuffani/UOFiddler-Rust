# Overview maps (Multimap.rle and facetNN.mul)

Image files of the world map used by the client to display the map at reduced scale. Little-endian.

| File | Content |
|---|---|
| `Multimap.rle` | 1-bit-per-pixel image (land or background), with RLE |
| `facet00.mul` to `facet05.mul` | color image per facet, with RLE per line |

## Multimap.rle

### Layout

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u32 | width | width in pixels |
| 4 | u32 | height | height in pixels |
| 8 | u8[] | runs | runs, up to the end of the file |

Each byte of `runs`:

| Bits | Meaning |
|---|---|
| 7 | 1 = land (pixel on), 0 = background |
| 0 to 6 | run length (1 to 127) |

Pixels are generated in line order, from left to right and from top to bottom, in a single continuous sequence (a run may cross the end of a line). The sum of the lengths is exactly `width * height`.

### Sample file

- Dimensions: 2560 x 2048 (5242880 pixels), 320056 bytes.
- Sum of the run lengths: 5242880.
- No byte has length 0.

### Notes

- Runs longer than 127 pixels are split into several consecutive bytes with the same bit 7.
- Fiddler rejects images with `width` or `height` equal to 0 or greater than 16384.

Reader in Fiddler: `uo_core::multimap` (`decode_multimap` with `header32 = true`).

## facetNN.mul

### Layout

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | i16 | width | width in pixels |
| 2 | i16 | height | height in pixels |
| 4 | line[height] | rows | one line per `y`, from top to bottom |

Each line:

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u32 | byte_length | size in bytes of the line's records (multiple of 3) |
| 4 | record[byte_length / 3] | records | records of the line |

Each record (3 bytes):

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u8 | run | number of consecutive pixels with this color (1 to 255) |
| 1 | u16 | color | 5:5:5 color |

Rules:

- The sum of the `run` values of each line is exactly `width`.
- The file ends right after the last line (no extra bytes).
- `color == 0` indicates an area with no mapping (outside the map).

### Color conversion

`color` is made up of bits `A RRRRR GGGGG BBBBB`:

| Bits | Channel |
|---|---|
| 10 to 14 | R (0 to 31) |
| 5 to 9 | G (0 to 31) |
| 0 to 4 | B (0 to 31) |
| 15 | see below |

Expansion from 5 to 8 bits: `v8 = (v5 << 3) | (v5 >> 2)`, applied to each channel. Fiddler paints `color == 0` as black and opaque.

### Bit 15

In the sample files, bit 15 appears only in `facet02.mul` (114005 records, including records with `color == 0x8000`). In the other facets it is never set. Its meaning has not been established; Fiddler ignores the bit when converting the color.

### Sample facet table

| File | Bytes | width | height | Pixels without color (color 0) | Map (id) | `map{N}.mul` dimensions |
|---|---:|---:|---:|---:|---|---|
| facet00.mul | 12035882 | 7168 | 4096 | 4425088 | 0 (Felucca) | 7168 x 4096 |
| facet01.mul | 12049697 | 7168 | 4096 | 4535048 | 1 (Trammel) | 7168 x 4096 |
| facet02.mul | 2396882 | 2304 | 1600 | 870822 | 2 (Ilshenar) | 2304 x 1600 |
| facet03.mul | 1536927 | 2560 | 2048 | 789772 | 3 (Malas) | 2560 x 2048 |
| facet04.mul | 821754 | 1448 | 1448 | 19 | 4 (Tokuno) | 1448 x 1448 |
| facet05.mul | 1305131 | 1280 | 4096 | 3441554 | 5 (Ter Mur) | 1280 x 4096 |

The facet number is the map id. The dimensions of each facet are the same as those of the `map{N}.mul` with the same number: `map0.mul` and `map1.mul` are 89915392 bytes (896 x 512 blocks of 196 bytes, 7168 x 4096 cells), `map2.mul` is 11289600 bytes (288 x 200 blocks), `map4.mul` is 6421156 bytes (181 x 181 blocks), and `map3.mul` and `map5.mul` are 16056320 bytes (320 x 256 and 160 x 512 blocks). The byte size of `map3.mul` and `map5.mul` is the same; what tells 2560 x 2048 apart from 1280 x 4096 is the `width x height` of the corresponding facet.

Each facet has a 1:1 scale with the map cells (one pixel per cell).

### Notes

- The sample file `Multimap.rle` is 2560 x 2048, the same dimensions as `facet03.mul`.
- Fiddler requires each line to sum exactly to `width` and the file to end at the last line; otherwise the image is rejected.

Reader in Fiddler: `uo_core::multimap` (`decode_facet`).
