# Animations (anim.mul and anim2 to anim5)

Creature, human and equipment animations are stored in five `.idx`/`.mul` pairs: `anim.mul` and `anim2.mul` to `anim5.mul`. Each pair uses the generic 12-byte index described in [index-mul.md](index-mul.md). The index entry number is computed from the body, the action and the direction. Little-endian in all fields.

The `AnimationFrame*.uop` and `AnimationSequence.uop` files (UOP format) are not covered here.

## Files and indexes

| File (index / data) | Entries in the `.idx` | Valid entries | Size of the `.mul` |
|---|---|---|---|
| `anim.idx` / `anim.mul` | 148810 | 35703 | 194950053 |
| `anim2.idx` / `anim2.mul` | 22455 | 5575 | 138407878 |
| `anim3.idx` / `anim3.mul` | 129850 | 3455 | 26107085 |
| `anim4.idx` / `anim4.mul` | 84700 | 20036 | 60543047 |
| `anim5.idx` / `anim5.mul` | 79275 | 11812 | 91405236 |

Valid entry: `lookup >= 0` and `length > 0`; `extra` is always 0. Invalid entries are `(-1, -1, -1)` or have `length = 0`.

Each animation (body, action) occupies 5 consecutive entries, one per stored direction (0 to 4). The entry index is:

```
index = base(file, body) + action * 5 + stored_direction
```

### Base per file

The number of entries reserved for a body depends on the body range: 110 (22 actions), 65 (13 actions) or 175 (35 actions).

| File | Body | `base(body)` | Entries per body |
|---|---|---|---|
| `anim.mul` and `anim4.mul` | 0 to 199 | `body * 110` | 110 |
| | 200 to 399 | `22000 + (body - 200) * 65` | 65 |
| | 400 and up | `35000 + (body - 400) * 175` | 175 |
| `anim2.mul` | 0 to 199 | `body * 110` | 110 |
| | 200 and up | `22000 + (body - 200) * 65` | 65 |
| `anim3.mul` | 0 to 299 | `body * 65` | 65 |
| | 300 to 399 | `33000 + (body - 300) * 110` | 110 |
| | 400 and up | `35000 + (body - 400) * 175` | 175 |
| `anim5.mul` | 0 to 199, except 34 | `body * 110` | 110 |
| | 200 to 399 and body 34 | `22000 + (body - 200) * 65` | 65 |
| | 400 and up | `35000 + (body - 400) * 175` | 175 |

Notes:

- In `anim5.mul`, body 34 follows the formula of the 200 to 399 range (`22000 + (34 - 200) * 65`, indexes 11210 to 11274, contiguous and valid in the reference file).
- In `anim3.mul`, the 300 to 399 and 400 and up ranges overlap from index 35000 on; the range is chosen only by the body number, in the order of the table.
- In `anim.mul`, bodies 0 to 199 have 22 actions in the layout, but each body uses only some of them (for example, body 12 has actions 0 to 6, 10 to 12 and 15 to 21 filled).

Reader in Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::base_index`, `index`)

## Animation entry (frames of one direction)

The block pointed to by an index entry contains all frames of one (body, action, stored direction).

Header:

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | u16[256] | `palette` | palette of 256 16-bit colors (bit 15 is 0; the reader sets bit 15) |
| 512 | u32 | `frame_count` | number of frames (the reader accepts 1 to 100; observed values up to 15) |
| 516 | u32[frame_count] | `frame_offset` | position of each frame, counted from offset 512 of the block (`position = 512 + frame_offset`) |

A `frame_offset` equal to 0 marks a missing frame. Entries shorter than 516 bytes or with `frame_count` outside 1 to 100 must be discarded.

Frame (at position `512 + frame_offset`):

| Offset | Type | Field | Meaning |
|---|---|---|---|
| 0 | i16 | `center_x` | image column where the horizontal anchor point is |
| 2 | i16 | `center_y` | vertical offset of the anchor point (usually negative) |
| 4 | u16 | `width` | width (1 to 1024) |
| 6 | u16 | `height` | height (1 to 1024) |
| 8 | u32[] | `runs` | runs up to the terminator |

Each run is a `u32` header followed by `run` bytes, each one an index into the palette (index into the block's `palette`):

- The frame ends when the `u32` read equals `0x7FFF7FFF` (compared before the XOR below).
- For any other value: `h = value XOR 0x80200000`. The constant is `(0x200 << 22) | (0x200 << 12)`.
- Fields of `h`: `x = (h >> 22) & 0x3FF`, `y = (h >> 12) & 0x3FF`, `run = h & 0xFFF`.
- Position of the first pixel of the run: column `(center_x - 0x200) + x`, row `(center_y + height - 0x200) + y`. The following `run` pixels occupy consecutive columns on the same row. The XOR turns the signed 10-bit offset into a value with bias `0x200`; the effective offset is `dx = x - 0x200` and `dy = y - 0x200`.
- Pixels outside the image are discarded. Pixels not covered by any run are transparent.

Anchor point: the offset pixel (0, 0) falls on column `center_x` and row `height + center_y` of the image. To draw, the top-left corner of the image goes at `anchor - (center_x, height + center_y)`.

Example (`anim.mul`, body 400, action 0, direction 0): `frame_count = 10`, first frame with `center_x = 12`, `center_y = -11`, `width = 24`, `height = 64`.

In the samples of each of the 5 reference files, all frames decode with pixels inside the image and end at the `0x7FFF7FFF` marker.

Reader in Fiddler: `crates/uo_core/src/animations.rs` (`decode_animation_mul`)

## Directions

Each animation stores 5 directions (0 to 4). Directions 5 to 7 are the horizontal mirror of the stored ones:

| Direction | Label in Fiddler | Entry read | Mirrored |
|---|---|---|---|
| 0 | South | 0 | no |
| 1 | South-West | 1 | no |
| 2 | West | 2 | no |
| 3 | North-West | 3 | no |
| 4 | North | 4 | no |
| 5 | North-East | 3 | yes |
| 6 | East | 2 | yes |
| 7 | South-East | 1 | yes |

Rule: for `direction > 4`, the entry read is `8 - direction` and the frame is mirrored horizontally (column `x` becomes `width - 1 - x`); the `center_x` of the mirrored frame is `width - center_x`.

## Actions by animation type

The action number is the second factor of the index (`action * 5`). The number of actions reserved for a body depends on the body range (section "Base per file"), not on the type. The animation type (monster, sea, animal, human) defines the name of each number in Fiddler. The type comes from `mobtypes.txt` (or from `Animationlist.xml`), and the numeric identifiers used by Fiddler are 0 = monster, 1 = sea, 2 = animal, 3 = human/equipment.

### Monster (type 0)

| Action | Name in Fiddler |
|---|---|
| 0 | Walk |
| 1 | Stand / Idle |
| 2 | Die 1 |
| 3 | Die 2 |
| 4 | Attack 1 |
| 5 | Attack 2 |
| 6 | Attack 3 |
| 7 | Cast / Special 1 |
| 8 | Cast / Special 2 |
| 9 | Get Hit |
| 10 | Block Right |
| 11 | Block Left |
| 12 | Idle |
| 13 | Fidget |
| 14 | Fly |
| 15 | Take Off |
| 16 | Get Hit In Air |

The monster layout in `anim.mul` reserves 22 actions; numbers 17 to 21 exist in the index, but Fiddler does not name them. In body 12 (dragon), actions 0 and 1 have 10 and 1 frames.

### Animal (type 2)

Layout with 13 actions (65 entries).

| Action | Name in Fiddler | Frames (body 200, dir. 0) |
|---|---|---|
| 0 | Walk | 5 |
| 1 | Run | 5 |
| 2 | Idle 1 | 1 |
| 3 | Eat | 5 |
| 4 | Alert | 3 |
| 5 | Attack 1 | 5 |
| 6 | Attack 2 | 5 |
| 7 | Get Hit | 5 |
| 8 | Die 1 | 6 |
| 9 | Idle 2 | 5 |
| 10 | Fidget | 3 |
| 11 | Lie Down | 6 |
| 12 | Die 2 | 6 |

### Sea (type 1)

| Action | Name in Fiddler |
|---|---|
| 0 | Walk |
| 1 | Run |
| 2 | Idle 1 |
| 3 | Idle 2 |
| 4 | Fidget |
| 5 | Attack 1 |
| 6 | Attack 2 |
| 7 | Get Hit |
| 8 | Die 1 |

### Human and equipment (type 3)

Layout with 35 actions (175 entries), actions 0 to 34, used by bodies 400 and up in `anim.mul`, `anim3.mul`, `anim4.mul` and `anim5.mul`. Bodies 400 and 401 of the reference file have all 35 actions with 5 directions.

| Action | Label in Fiddler | Frames (body 400, dir. 0) |
|---|---|---|
| 0 | Walk_01 | 10 |
| 1 | WalkStaff_01 | 10 |
| 2 | Run_01 | 10 |
| 3 | RunStaff_01 | 10 |
| 4 | Idle_01 | 1 |
| 5 | Idle_02 | 5 |
| 6 | Fidget_Yawn_Stretch_01 | 5 |
| 7 | CombatIdle1H_01 | 1 |
| 8 | CombatIdle2H_01 | 1 |
| 9 | AttackSlash1H_01 | 7 |
| 10 | AttackPierce1H_01 | 7 |
| 11 | AttackBash1H_01 | 7 |
| 12 | AttackBash2H_01 | 7 |
| 13 | AttackSlash2H_01 | 7 |
| 14 | AttackPierce2H_01 | 7 |
| 15 | CombatAdvance_1H_01 | 10 |
| 16 | Spell1 | 7 |
| 17 | Spell2 | 7 |
| 18 | AttackBow_01 | 7 |
| 19 | AttackCrossbow_01 | 7 |
| 20 | GetHit_Fr_Hi_01 | 5 |
| 21 | GetHit_Fr_Med_01 | 6 |
| 22 | GetHit_Fr_Low_01 | 6 |
| 23 | Block_Shield_01 | 5 |
| 24 | Punch_01 | 5 |
| 25 | Bow_01 | 1 |
| 26 | Salute_01 | 5 |
| 27 | Ingest_Eat_01 | 5 |
| 28 | Horse_Walk_01 | 7 |
| 29 | Horse_Run_01 | 5 |
| 30 | Horse_Idle_01 | 5 |
| 31 | Horse_Attack1H_01 | 7 |
| 32 | Horse_AttackBow_01 | 5 |
| 33 | Horse_AttackCrossbow_01 | 5 |
| 34 | Horse_Spell2_01 | 5 |

The Fiddler table also lists actions 35 (`Die_Hard_Fwd_01`) and 36 (`Die_Hard_Back_01`), which fall outside the 35 actions of the 175-entry layout.

Fields not interpreted: the labels of the human actions are the names shown by Fiddler; the file only contains the action number and the frame count. The frame counts in the right-hand column are there to check the numbering and do not come from Fiddler.

Reader in Fiddler: `crates/uo_gui/src/main.rs` (`ACTIONS_MONSTER`, `ACTIONS_ANIMAL`, `ACTIONS_SEA`, `ACTIONS_HUMAN`, `DIRECTIONS`)

## Body resolution to file

Given a requested body, the client (and Fiddler) decides in which file and with which body number the animation is, using `Body.def` and `Bodyconv.def`. Both are text; the general syntax is in [def-files.md](def-files.md).

### Body.def

Lines `<ORIGINAL_BODY> {<NEW_BODY>[, ...]} <HUE>`. The body is replaced by the first value in braces and the hue of the line goes with the result. For a duplicate key, the first occurrence applies.

```
11 {28} 1401
46 {12, 59} 1106
```

### Bodyconv.def

Lines with five numeric columns separated by tab or space: `<BODY> <anim2> <anim3> <anim4> <anim5>`. Each column is the body number inside the corresponding `animN.mul` file, or `-1` if the body is not in that file. The first column (in the order anim2, anim3, anim4, anim5) with a non-negative value applies. Without a line, or with all columns `-1`, the body stays in `anim.mul`.

```
157	1	-1	-1	-1
```

### Algorithm (Fiddler order)

1. Apply `Body.def`: `(body, hue) = body_def[body]`, if it exists.
2. Apply `Bodyconv.def`: if a line exists, `(file, body) = (anim2..anim5, column value)`; otherwise `file = anim.mul`.
3. Compute the index with the formula in section "Base per file" for `(file, body, action, direction)` and read the block.
4. If the block does not exist or does not decode, try the animation in UOP.
5. If it still does not exist, try the original body (without conversion) in each of `anim.mul` to `anim5.mul`, in order.

Reader in Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::parse`, `resolve`, `index`, `load_animation_frames`)

### Corpse.def

Same format as `Body.def`. Applies to the body used to draw corpses.

Reader in Fiddler: not interpreted (hex only in the Files tab)

### Anim1.def to Anim4.def

Text tables in the format `<ORIG> {<NEW>} <HUE>` (see [def-files.md](def-files.md)). `Anim1.def` has 9 lines (13 to 21); `Anim2.def` has bodies 35 to 271 with targets such as `{23}` or `{-1}`; `Anim3.def` and `Anim4.def` are empty.

Fields not interpreted: the meaning of the numbers (body or action) in these files.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## mobtypes.txt

Purpose: animation type of each body.

Text, one line per body; lines starting with `#` are comments and lines without a number at the start are ignored. Fields separated by tabs/spaces:

| Field | Meaning |
|---|---|
| `ID` | body (decimal) |
| `TYPE` | `MONSTER`, `SEA_MONSTER`, `ANIMAL`, `HUMAN` or `EQUIPMENT` |
| `FLAGS` | hexadecimal; Fiddler interprets only bit `0x10000` (body animated by the UOP files) |

Text after `#` on the same line is a comment. The reference file has 1210 lines (with comments and blank lines); by type: 297 `MONSTER`, 97 `ANIMAL`, 25 `HUMAN`, 4 `SEA_MONSTER` and 732 `EQUIPMENT`. Some lines have an extra empty field (double tab) or one more numeric column before the comment.

Conversion of `TYPE` to the Fiddler animation type: `MONSTER` = 0, `SEA_MONSTER` = 1, `ANIMAL` = 2, `HUMAN` and `EQUIPMENT` = 3 (`EQUIPMENT` also marks the body as equipment).

Fields not interpreted: the other bits of `FLAGS` (for example `1008`, `2A`, `20000`).

Reader in Fiddler: `crates/uo_core/src/animations.rs` (`parse_mobtypes_txt`, `load_animation_defs`)
