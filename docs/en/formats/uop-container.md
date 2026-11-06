# UOP Container (MYP)

## Purpose

UOP is the file container used by the Ultima Online client to group many resources into a single file (animations, tables, text, images, maps). Each resource is an entry identified by a 64-bit hash computed over a logical path (for example `build/animationlegacyframe/000009/01.bin`). The name itself is not stored in the file.

Files: `*.uop` (for example `AnimationFrame1.uop`, `AnimationSequence.uop`, `MainMisc.uop`, `string_dictionary.uop`, `tileart.uop`).

Little-endian throughout the format.

## General layout

```
[file header           : 40 bytes]
[block 0               : 12 + capacity * 34 bytes]
[entry data of block 0]
[block 1]
[entry data of block 1]
...
```

Blocks are chained by pointer. The first block starts at the offset given in the header (40 in the observed files). Each block is followed by the data of the entries it describes, so the next block sits in the middle of the file (for example, in `AnimationFrame1.uop` block 1 is at 83914468 and block 2 at 89789380).

## File header (40 bytes)

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | char[4] | signature | `"MYP\0"` (`4D 59 50 00`) |
| 4 | u32 | version | 4 in the client files; the Fiddler converter writes 5 |
| 8 | u32 | misc | `0xFD23EC43` in all observed files |
| 12 | u64 | first_block_offset | absolute offset of the first block (40) |
| 20 | u32 | block_capacity | number of slots per block (100) |
| 24 | u32 | file_count | total number of entries in use in the file |
| 28 | u8[12] | reserved | zeros |

`file_count` equals the sum of the entries in use across all blocks (287 in `AnimationFrame1.uop`, which has blocks with 100, 100 and 87 entries).

## Block

A block always occupies `12 + block_capacity * 34` bytes (3412 with capacity 100). Unused slots are zeroed.

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u32 | count | number of slots in use in this block |
| 4 | u64 | next_block | absolute offset of the next block; 0 in the last one |
| 12 | entry[block_capacity] | entries | entry table (34 bytes each) |

To read: start at `first_block_offset`; read `count` entries starting at `block + 12`; follow `next_block` until 0. Skip slots with `offset == 0`.

## Table entry (34 bytes)

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u64 | offset | absolute position of the start of the entry in the file; 0 = empty slot |
| 8 | u32 | header_length | per-entry header bytes before the data (12 in the client files; 0 in the converter-generated ones) |
| 12 | u32 | compressed_length | size of the stored data |
| 16 | u32 | decompressed_length | size of the data after decompression |
| 20 | u64 | name_hash | 64-bit Jenkins hash of the logical path (see below) |
| 28 | u32 | data_hash | Adler-32 of the entry's `header_length` header bytes (client files); 0 in the converter-generated ones |
| 32 | u16 | compression | 0 = no compression, 1 = zlib |

## Data region

An entry's data starts at `offset + header_length`, with `compressed_length` bytes.

- `compression == 0`: the bytes are the content (`compressed_length == decompressed_length`).
- `compression == 1`: the bytes are a zlib stream (RFC 1950) that decompresses to `decompressed_length` bytes.

In the sample files all entries use `compression == 1`. Entries are written in ascending offset order, in the same order as the table.

### Per-entry header (12 bytes)

In the client files each entry begins with 12 bytes before the data (`header_length = 12`):

| Offset | Type | Field | Meaning |
|---:|---|---|---|
| 0 | u16 | - | 3 in all observed entries |
| 2 | u16 | - | 8 in all observed entries |
| 4 | u8[8] | - | varies per entry; not interpreted |

Uninterpreted fields: the last 8 bytes of the per-entry header.

## Name hash (64-bit Jenkins)

The hash is computed over the bytes of the path in lowercase, without a leading slash and with `/` as the separator. The result is `(b << 32) | c`. For the empty string the result is `c << 32`, with `c = 0xDEADBEEF`.

Arithmetic is u32 with overflow (wrapping); `rotl` is a 32-bit left rotation.

```
len = size in bytes
a = b = c = 0xDEADBEEF + len

while len > 12:
    a += u32le(k[0..4]);  b += u32le(k[4..8]);  c += u32le(k[8..12])
    a -= c;  a ^= rotl(c, 4);   c += b
    b -= a;  b ^= rotl(a, 6);   a += c
    c -= b;  c ^= rotl(b, 8);   b += a
    a -= c;  a ^= rotl(c, 16);  c += b
    b -= a;  b ^= rotl(a, 19);  a += c
    c -= b;  c ^= rotl(b, 4);   b += a
    k += 12;  len -= 12

if len == 0: return c << 32

# remainder (1..12 bytes), padded with zeros to 12
a += u32le(rest[0..4]);  b += u32le(rest[4..8]);  c += u32le(rest[8..12])

c ^= b;  c -= rotl(b, 14)
a ^= c;  a -= rotl(c, 11)
b ^= a;  b -= rotl(a, 25)
c ^= b;  c -= rotl(b, 16)
a ^= c;  a -= rotl(c, 4)
b ^= a;  b -= rotl(a, 14)
c ^= b;  c -= rotl(b, 24)

return (b << 32) | c
```

Reference value: `hash("build/animationlegacyframe/000009/01.bin") = 0x994AE0E43E27CB16`.

## Existing name patterns

| File | Logical path pattern | Note |
|---|---|---|
| AnimationFrame1..4.uop | `build/animationlegacyframe/{body:06}/{action:02}.bin` | action 0 to 78 in the samples |
| AnimationSequence.uop | `build/animationsequence/{id:08}.bin` | id 0 to 1247 (sparse) |
| string_dictionary.uop | `build/stringdictionary/string_dictionary.bin` | single entry |
| tileart.uop | `build/tileart/{id:08}.bin` | ids 0 to 39212, 10000000 to 10001247 and 99999999 |
| artLegacyMUL.uop | `build/artlegacymul/{i:08}.tga` | i = index in art.mul |
| gumpartLegacyMUL.uop | `build/gumpartlegacymul/{i:08}.tga` | i = index in gumpart.mul |
| soundLegacyMUL.uop | `build/soundlegacymul/{i:08}.dat` | i = index in sound.mul |
| map{N}LegacyMUL.uop | `build/map{N}legacymul/{i:08}.dat` | i = number of the 0xC4000-byte chunk |
| MultiCollection.uop | `build/multicollection/{id:06}.bin` and `build/multicollection/housing.bin` | |
| MainMisc.uop | no name pattern established | identify by hash (see `uop-files.md`) |

The files `*LegacyMUL.uop`, `MultiCollection.uop` and `GameData.uop` follow the conventions above, but they do not exist in the sample directory.

The resource map in `MainMisc.uop` lists, for each resource type, the directory and the suffix in the packed format (for example `Build/AnimationSequence/` + 8 digits + `.bin`, `Build/Paperdoll/` + 6 digits + `/` + 2 digits + `.bin`). Path names are always converted to lowercase before hashing.

## Enumerating unnamed entries

The file does not store names, only `name_hash`. To obtain a name:

1. Read all entries (walking the blocks) and build a dictionary `name_hash -> entry`.
2. Generate candidate names from the file's pattern (for example `body` from 0 to 1999 and `action` from 0 to 79), compute the hash of each one and look it up in the dictionary.
3. Entries with no match remain identified only by `name_hash` (display as `{hash:016x}`) and by their position in the table.

Points of attention:

- The hash is case-sensitive: the path must be in lowercase (`build/stringdictionary/string_dictionary.bin`, not `Build/StringDictionary/...`).
- In `tileart.uop`, the entry `build/tileart/99999999.bin` is outside the normal id ranges.
- In `MainMisc.uop` no name pattern has been established; the 4 entries are identified by hash (see `uop-files.md`).

## Conversion to MUL

The `*LegacyMUL.uop` files map one-to-one to the classic `.mul/.idx` files. See `uop-files.md`, section "Legacy content UOPs", for the layout of each type and the `uop_convert` tool.

## Uninterpreted fields

- Last 8 bytes of the 12-byte per-entry header.
- Algorithm for `data_hash` for entries with `header_length == 0` (the converter writes 0).
- Meaning of the `misc` field (`0xFD23EC43`).

Reader in Fiddler: `uo_core::client_files` (`read_uop`, `read_uop_entry`, `resolve_uop_names`) and `uo_core::hash` (`hash_file_name`); writer in `uo_core::uop_convert` (`write_uop`).
