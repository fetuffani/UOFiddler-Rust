# Client .def files

Text tables that redirect identifiers (animation body, gump, sound, art) or describe interface resources. All of them are in the client directory. Except where noted, they are ASCII text with CRLF line endings and no BOM.

## Common syntax

Redirect line format (Body.def, Corpse.def, gump.def, Sound.def, Music.def, Anim1.def to Anim4.def, art.def, TexTerr.def):

```
<ORIGINAL_ID> {<NEW_ID>[, <NEW_ID2>...]} <HUE>
```

| Field | Description |
|---|---|
| `ORIGINAL_ID` | decimal integer; key of the line |
| `{...}` | group in braces, with one or more integers separated by commas (and optional space). The first value is the main target; the others are alternatives |
| `HUE` | decimal integer (hue) applied to the result; `0` = no hue |

Reading rules:

- Separators between fields are spaces and/or tabs, in variable amounts (`2500  {3} 1645`, `2500<TAB>{3}<TAB>1645`).
- `#` starts a comment until the end of the line; it also applies in the middle of a line (`401 866 867 60867 0 # text`). Lines starting with `#` are full-line comments. Commented lines may contain disabled entries (`#679 {528} 1249`).
- Long comment lines appear in quotes (`"# text..."`); they must be ignored like the others.
- Blank lines are ignored; there may be leftover spaces and tabs at the end of a line.
- For a duplicated key, Body.def and gump.def use the first occurrence; Equipconv.def uses the last one for the same pair (`bodyType`, `equipmentID`).
- The last record may not have a line ending.

Example of a group with several values (Body.def):

```
46 {12, 59} 1106
114 {200, 226, 228, 204} 1109
```

## Body.def

Redirects the original animation body to another body and hue before the animation is fetched. File header: `# Format is: <ORIG BODY> {<NEW BODY>} <NEW HUE>`. Contains 237 entries (IDs 11 to 1762), of which 227 have one value in the group, 5 have two, 2 have three and 3 have four.

```
11 {28} 1401
38 {9} 0
55 {53, 54} 1154
```

Application: the requested body is replaced by the first value of the group; the line's hue (if different from 0) becomes the body's hue. After that, Bodyconv.def is applied. Comment blocks mark eras (`# Begin AoS Stuff` / `# End AoS Stuff`).

Reader in Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver::parse`, `resolve`)

## Bodyconv.def

Indicates in which animation file (`anim2.mul` to `anim5.mul`) the body is stored and with which body number inside it. Bodies without a line are in `anim.mul`.

Format (tabs, 5 numeric columns):

```
<BODY> <anim2> <anim3> <anim4> <anim5>
```

| Column | Meaning |
|---|---|
| 1 | original body (after Body.def) |
| 2 | body inside `anim2.mul` (LBR version) |
| 3 | body inside `anim3.mul` (AoS version) |
| 4 | body inside `anim4.mul` (AoW version) |
| 5 | body inside `anim5.mul` (Mondain version) |

`-1` means "not in this file". In all 547 active lines with a target, exactly one column is >= 0 (95 in column 2, 34 in 3, 131 in 4, 287 in 5); 4 lines have all four columns at `-1`. If more than one column is >= 0, the first one applies, in the order anim2, anim3, anim4, anim5. Trailing comments after the fifth column are allowed (`877 -1 -1 661 -1 # Book 01`). The file header is a comment block (some lines in quotes) that describes the ranges: 0-199 monsters, 200-399 animals, 400+ humans/elves/equipment; maximum index 2048.

Examples:

```
157	1	-1	-1	-1
942	-1	-1	-1	492
483	-1	-1	-1	345	#	Horns_Gargoyle_Male_Legacy
```

Position of the body inside each `animN.idx` (animation 0, direction 0), used after conversion:

| File | Body | Base index |
|---|---|---|
| anim.mul, anim4.mul | < 200 | `body * 110` |
| | 200..399 | `22000 + (body - 200) * 65` |
| | >= 400 | `35000 + (body - 400) * 175` |
| anim2.mul | < 200 | `body * 110` |
| | >= 200 | `22000 + (body - 200) * 65` |
| anim3.mul | < 300 | `body * 65` |
| | 300..399 | `33000 + (body - 300) * 110` |
| | >= 400 | `35000 + (body - 400) * 175` |
| anim5.mul | < 200 and different from 34 | `body * 110` |
| | 200..399 (and 34) | `22000 + (body - 200) * 65` |
| | >= 400 | `35000 + (body - 400) * 175` |

Final index = base + `action * 5` + direction (directions 0 to 4; directions 5 to 7 mirror 3 to 1).

Reader in Fiddler: `crates/uo_core/src/anim_convert.rs` (`ClassicAnimResolver`)

## Equipconv.def

Converts the equipment ID (animation) to another one depending on the body type of the wearer, and may change the paperdoll gump and the hue.

Format (tabs; `#` comment at the end of the line):

```
<bodyType> <equipmentID> <convertToID> <GumpIDToUse> <hue>
```

| Field | Description |
|---|---|
| `bodyType` | body type of the wearer. Values present: 401 (female human), 605 (male elf), 606 (female elf), 666 (male gargoyle), 667 (female gargoyle), 745 |
| `equipmentID` | original equipment/animation ID |
| `convertToID` | equipment ID it is converted to |
| `GumpIDToUse` | paperdoll gump (rule below) |
| `hue` | equipment hue (all active lines present use 0) |

Rule for `GumpIDToUse`:

| Value | Resulting gump |
|---|---|
| `0` | `equipmentID + 50000` |
| `-1` | `convertToID + 50000` |
| other | the value itself is the gump ID |

Count in the active lines: 48 with `0`, 23 with `-1`, 15 with a literal value.

Rule for a gump without an entry: if there is no line for (`bodyType`, `equipmentID`), the male gump is `equipmentID + 50000`.

Female rule (+10000): when the wearer is female and the resolved gump is in 50000..59999, `gump + 10000` (range 60000) is used, provided that this gump exists in `gumpart`; otherwise the male gump remains. A literal value already in the 60000 range (e.g. `60867`) is not added to again.

Once resolved, the gump goes through gump.def (below): if there is a redirect, the gump and (when different from 0) the hue are replaced. The final hue comes from the `hue` of Equipconv or, if gump.def sets a hue different from 0, from the latter.

Reverse lookup (gump to animation): first the explicit Equipconv targets; otherwise 50000..59999 corresponds to the male animation `gump - 50000` and 60000..69999 to the female `gump - 60000`.

Examples:

```
401	866	867	60867	0	#	Helm of swiftness M to F
401	538	986	0	0			# female chain substitution
606	968	977	-1	0			#hide chest
666	865	-1	50885	0			#Male Garge
```

Notes: the header is a comment block (it includes a quoted line `"#   -GumpID (0 = ...`); lines starting with `#` followed by numbers are disabled entries; blocks are separated by section comments (`#female human`, `# female elf`, `#Gargoyle Talon Swaps`).

Reader in Fiddler: `crates/uo_core/src/equip_gump.rs` (`EquipGumpMap::parse`, `gump_for_anim`, `anim_for_gump`)

## gump.def

Redirects a gump (used only in the paperdoll) to another gump and hue. Same format as the redirects (`<GUMP> {<NEW>} <HUE>`), with IDs in the 50000 range (male) and 60000 (female). For equipment both ranges must be declared, otherwise the female paperdoll uses the male art. Contains 32 entries.

```
50759	{50468}	0
60759	{60468}	0
60482 {60477} 35
```

Application: after resolving the gump (Equipconv or `ID + 50000`), if the gump is listed in gump.def it is replaced by the new one and, if the line's hue is different from 0, it replaces the hue.

Reader in Fiddler: `crates/uo_core/src/equip_gump.rs`

## Corpse.def

Same format and header as Body.def (`<ORIG BODY> {<NEW BODY>} <NEW HUE>`), applied to the body used when drawing corpses. Contains 240 distinct keys (IDs 11 to 1762); 230 of them also exist in Body.def (220 with identical target and hue), and there are lines of its own (for example `13 {51} 16385`). Section comments: `#Dragon Armor`, `#Begin AoS Stuff`, `# Male Vampire`.

```
13 {51} 16385
46 {12, 59} 1106
```

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Anim1.def, Anim2.def, Anim3.def, Anim4.def

Body redirect tables in the format `<ORIG> {<NEW>} <HUE>`. They have no comments. The third field is always 0.

| File | Entries | IDs | Target |
|---|---|---|---|
| Anim1.def | 9 | 13 to 21 | always >= 0 (e.g.: `13 {5} 0`) |
| Anim2.def | 237 | 35 to 271 | 182 with `-1` (e.g.: `35 {-1} 0`) and 55 with a body (e.g.: `43 {23} 0`) |
| Anim3.def | 0 | empty file (0 bytes) | |
| Anim4.def | 0 | empty file (0 bytes) | |

`-1` indicates that there is no target body.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Sound.def

Redirects a sound ID to another. Format `<SOUND> {<NEW>} <HUE>`; the third field is always 0. Contains 786 entries (IDs 654 to 1440): 435 with a target >= 0 (e.g.: `654 {487} 0`) and 351 with `-1` (no target sound, e.g.: `778 {-1} 0`). One line is missing its closing brace (`805 {331 0`); a reader must ignore or tolerate malformed lines.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Music.def

Same format as the redirects, with a single neutral entry and no line ending: `0 {0} 0`.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## art.def

Redirects an art index to another, with hue. Format `<ID> {<NEW>} <HUE>`; tab or space separators, both present in the same file. Contains 620 entries (IDs 2500 to 26319), with one duplicated key. Section comments: `#Begin AoS Stuff`, `# Samurai stuff`, `# Ninja stuff`, `# BOD system`, `# Mahjong Game`.

```
2500	{3}	1645
25073	{24863}	1161
25349	{21824} 0
```

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## TexTerr.def

Same format as art.def (`<ID> {<NEW>} <HUE>`, space or tab separators). Contains 285 entries (IDs 2500 to 26288), 284 of which have an ID that is also present in art.def; it has no comments.

```
2500  {3} 1645
25940 {20320} 1645
```

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## entitled.def

Binary file of 4 bytes: `01 00 00 00` (32-bit little-endian integer with value 1). It is not text.

Reader in Fiddler: not interpreted (hex only in the Files tab)

## Intrface.def

Interface layout script (client interface gumps). UTF-16 little-endian text with BOM `FF FE`, CRLF line endings, 7940 lines. The header (`// NOTICE...`) asks that the file not be edited.

Observed syntax:

- `//` starts a line comment.
- `# name` opens a named block and `#enddef` (or `# enddef`) closes it. Blocks are referenced by name elsewhere (`include MessageBoxBackground`, and as the last argument of `interface`, `layoutbutton`, etc.).
- One command per line, with arguments separated by spaces: `rect x y w h`, `background gump x y ...`, `button id x y gumpNormal gumpPressed ...`, `textsimple`, `text`, `textarea`, `toggle`, `container`, `decoration`, `statusbar`, `vlist`, `colorpicker`, among others. The most frequent are `sizablebackground` (375 lines), `text` (288), `rect` (254), `textarea` (229), `layoutbutton` (221), `background` (212).
- Lines starting with a tab are properties of the immediately preceding element (`fontnumber 2`, `textoutline 25297`, `emboss`, `rotation 3`, `settooltip 1062709`, `setbuttonholdable`).
- Coordinates may be expressions: `R-170` (relative to the right edge), `B-(238+0*gw11321)` (relative to the bottom edge), `gw11321`/`gh11264` (width/height of the indicated gump).
- `settooltip` and `tooltip` take cliloc numbers (e.g.: `settooltip 1062709`).

Example:

```
# mahjongboard
rect 100 100 gw11264 gh11264
fadeininterface 500 100
background 11264 0 0 0 0
textsimple 1606501 258 499 200 20 0
	fontnumber 2
	textoutline 25297
```

Unparsed fields: the individual meaning of each positional argument and of each command.

Reader in Fiddler: not interpreted (text only in the Files tab)

## stitchin.def

Appearance composition rules (hair, beard, clothing) for the 3D character. ASCII text, CRLF, 9202 lines. First line `ver 1`. Comments use `//`.

Each definition starts at `# <ID>` (675 blocks) and ends at `# enddef`/`#enddef`. Directives inside the block:

| Directive | Arguments |
|---|---|
| `covers` | body parts covered |
| `coveredBy` | parts that, if covered, affect this item |
| `remove` | list of removed IDs |
| `replace A with B` | replaces ID A with B |
| `whenhuedreplacewith` | ID used when there is a hue |

Body parts: `HEAD FACE EARS NECK TORSO UPPER_ARMS_TOP UPPER_ARMS_BOTTOM LOWER_ARMS_TOP LOWER_ARMS_BOTTOM HANDS PELVIS UPPER_LEGS_TOP UPPER_LEGS_BOTTOM LOWER_LEGS_TOP LOWER_LEGS_BOTTOM FEET`.

Example:

```
# 990
covers HEAD FACE EARS NECK TORSO ... FEET
 remove 703 903 701 710 902 702 901 700 900
# enddef
```

Unparsed fields: the meaning of the referenced IDs (3D models).

Reader in Fiddler: not interpreted (text only in the Files tab)
