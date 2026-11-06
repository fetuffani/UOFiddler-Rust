# Text tables, configuration and small client files

Reading convention for all text files: accept UTF-8 BOM (`EF BB BF`), UTF-16 with BOM (`FF FE` little-endian or `FE FF` big-endian) and, without BOM, ASCII/Latin-1; accept CRLF and LF. Unless stated otherwise, the files below are ASCII with CRLF.

## mobtypes.txt

Type and flags of each animation body. ASCII, CRLF, 1210 lines.

```
<ID> <TYPE> <FLAGS>
```

| Column | Description |
|---|---|
| `ID` | body, decimal |
| `TYPE` | `MONSTER`, `ANIMAL`, `SEA_MONSTER`, `HUMAN` or `EQUIPMENT` (uppercase) |
| `FLAGS` | integer in hexadecimal, without prefix (`0`, `2A`, `1008`, `10000`) |

Separators: tab or spaces, in variable amounts. The first two lines are comments (`# Animation types for animation lookups`, `# ID TYPE FLAGS`). `#` comments also appear at the end of a line and on whole lines (disabled entries, e.g.: `# 130 MONSTER 8`). Some lines have a fourth column `0` before the comment (`753 MONSTER 10000 0 # Gargoyle Slave`), and two lines (`492`, `427`) have the comment in place of the flags. There are 1099 data lines, with 1048 distinct IDs; the IDs are not in ascending order and some repeat. Count by type: EQUIPMENT 723, MONSTER 255, ANIMAL 96, HUMAN 21, SEA_MONSTER 4.

Bit in use: `0x10000` (flag `10000`, 144 lines) marks animation stored in the UOP containers.

Example:

```
1	MONSTER	0
5	ANIMAL	2A
666	HUMAN 		10000	0 #	Gargoyle Male
```

Fields not interpreted: remaining bits of `FLAGS`.

Reader in Fiddler: `crates/uo_core/src/animations.rs` (`parse_mobtypes_txt`)

## Tile256.txt and Tile1024.txt

Tables of `<ID><TAB><VALUE>` pairs, both decimal, with no header or comments, CRLF.

| File | Lines | ID range | Value range | Distinct values |
|---|---|---|---|---|
| Tile256.txt | 1604 | 2 to 5000 | 0 to 162 | 163 |
| Tile1024.txt | 1581 | 2 to 2799 | 0 to 10 | 11 |

The IDs are unique and almost all in ascending order. Example (Tile256.txt): `16<TAB>1`, `26<TAB>2`, `5000<TAB>157`.

Fields not interpreted: meaning of the value.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## House construction tables: walls, floors, roof, doors, stairs, teleprts, misc

Catalogs of parts for the house editor. ASCII, tab as separator, CRLF (teleprts.txt uses `CR CR LF`).

Common structure:

1. Line 1: type of each column (`int` ... `string`).
2. Line 2: name of each column.
3. Data lines: one category/style per line; the last column (`Comment`) is the free-text name.

doors.txt has blank lines at positions 2 and 4; all files end with CRLF.

| File | Columns | Data lines |
|---|---|---|
| walls.txt | `Category Style TID South1 South2 South3 Corner East1 East2 East3 Post WindowS AltWindowS WindowE AltWindowE SecondAltWindowS SecondAltWindowE FeatureMask Comment` | 171 |
| floors.txt | `Category F1 ... F16 FeatureMask Comment` | 55 |
| roof.txt | `Category Style TID North East South West NSCrosspiece EWCrosspiece NDent EDent SDent WDent NTPiece ETPiece STPiece WTPiece XPiece Extra Piece FeatureMask Comment` | 34 |
| doors.txt | `Category Piece1 ... Piece8 FeatureMask Comment` | 36 |
| stairs.txt | `Category Block North East South West Squared1 Squared2 Rounded1 Rounded2 MultiNorth MultiEast MultiSouth MultiWest FeatureMask Comment` | 17 |
| teleprts.txt | `Category F1 ... F16 FeatureMask Comment` | 1 |
| misc.txt | `Category Style TID Piece1 ... Piece8 FeatureMask Comment` | 91 |

Notes:

- Part values are item art IDs (static tiles); `0` = part absent.
- `TID` is a cliloc number with the category name; `0` = no proper name (the line continues the previous style). E.g.: TID 1060054 corresponds to the text "Dark Wood Walls" in Cliloc.enu.
- `FeatureMask` is a bitmask of required features. Values present: 0, 64, 128, 512, 65536, 262144, 524288.
- `walls.txt`, `roof.txt`, `doors.txt`: 18/20/10 numeric columns respectively plus `Comment`.
- misc.txt has one line with 12 columns and a space separator instead of a tab between two parts (`19261   19249`); readers must tolerate it.

Examples:

```
Category	Style	TID	South1	...	FeatureMask	Comment
0	0	1060054	10	7	12	6	13	8	11	9	0	14	15	0	0	0	0	Dark Wood Standard Walls
```

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## suppinfo.txt

Support information (structural support) per building part, one line per tile. ASCII, CRLF, 1783 lines.

- Line 1: types (first column empty, then `int` x15 and `string`); line 2: names.
- Columns: (1) shape symbol, short text (`\`, `./`, `v`, `.`, `o`, `*`, `` ` ``, `*\`, `*/`, `..`, `\\`, `//`); (2) `tileNumber`; (3..16) `top bottom adj_UN adj_LN adj_UE adj_LE adj_US adj_LS adj_UW adj_LW directSupports cango W cango N cango NWC` (integers 0/1); (17) `Comment`, usually empty.
- Some lines use spaces instead of tabs in fields 0/1.

```
	tileNumber	top	bottom	adj_UN	adj_LN	adj_UE	adj_LE	adj_US	adj_LS	adj_UW	adj_LW	directSupports	cango W	cango N	cango NWC	Comment
\	10	1	1	0	0	1	1	1	1	1	1	1	0	1	0	Dark Wood Std
```

Fields not interpreted: semantics of the adjacency flags and of the shape symbol.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Keynames.txt

Key names. ASCII, CRLF, 89 lines. The index line N (starting at 0) is the name of the key for scan code N: line 0 `NULL`, 1 `Esc`, 2 `1`, 14 `Backspace`, 15 `Tab`, 28 `Enter`, 57 `Space`, 59 `F1`, 87 `F11`, 88 `F12`. Codes without a key use `NULL`. Numeric keypad: `Num 7`, `Num 8`, `Num +`, `Num Del`.

The names in this file are the ones used in the first column of the macro files.

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Macros.txt, Umacros.txt, macros2d.txt, Default.mac

Keyboard shortcuts (macros). Each macro is a block ended by a `########` line.

```
<KEY> <ALT> <CTRL> <SHIFT>
<ACTION>
########
```

| Line | Description |
|---|---|
| Header | key name (as in Keynames.txt; may contain a space, e.g. `Num 8`) followed by three 0/1 flags separated by spaces: Alt, Ctrl, Shift. The name is everything before the last three tokens |
| Action | action text (`Open Status`, `War/Peace`, `Bow`, `Say Forward`); `NULL` = no action |
| `########` | end of block (eight `#`) |

Files:

| File | Content |
|---|---|
| Default.mac | 22 default macros; action in plain text; 67 lines, CRLF. Includes `Say Forward`, `Say Stop`, `Say Turn Left`, `Say Raise Anchor` on the 9 `Num` keys (flags `0 1 0`) |
| Macros.txt | 22 macros (13 letters with Ctrl, `B` and `S` with Alt, 7 `Num` keys with `1 1 0`), all with action `NULL ` (the text `NULL` followed by a space) |
| Umacros.txt | empty file (0 bytes) |
| macros2d.txt | 15 macros; header in ASCII and action line formed by `+` (1 byte) followed by the text in UTF-16 little-endian without BOM, terminated by CRLF in one byte (`+O\0p\0e\0n\0 \0S\0t\0a\0t\0u\0s\0`) |

Examples (Default.mac / Macros.txt):

```
S 0 1 0
Open Status
########
B 1 0 0
Bow
########
Num 8 0 1 0
Say Forward
########
```

Actions present in macros2d.txt: `Open Status`, `Open Chat`, `Open MageSpellbook`, `War/Peace`, `Open Paperdoll`, `Open Skills`, `Open Journal`, `Open QuestLog`, `Open SpellWeavingSpellbook`, `Open Backpack`, `Open Overview`, `Open Configuration`, `QuitGame`, `Bow`, `Salute`. The actions accept a trailing space.

Fields not interpreted: a macro with more than one action (all blocks present have a single action); the third flag is never different from 0.

Reader in Fiddler: not interpreted (text only in the Files tab)

## Prof.txt

Definition of the professions for character creation. ASCII, CRLF, 124 lines, no line ending after the last `End`. Indentation with tabs, several tabs between key and value (alignment).

```
Begin
	Name			Samurai
	TrueName		"Samurai"
	NameId			1062948
	DescId			1062950
	Desc			6
	TopLevel		true
	Gump			5591
	Type			Profession
	Skill			Bushido			30
	Skill			Swordsmanship		30
	Skill			Focus		30
	Skill			Parrying	30
	Stat			Str			40
	Stat			Dex			30
	Stat			Int			20
End
```

| Key | Description |
|---|---|
| `Name` | internal name |
| `TrueName` | name in quotes |
| `NameId` | cliloc number of the name (1062948 = "SAMURAI") |
| `DescId` | cliloc number of the description |
| `Desc` | numeric index of the description (1 to 7, unique per profession) |
| `TopLevel` | `true` |
| `Gump` | gump ID of the profession |
| `Type` | `Profession` |
| `Skill <name> <value>` | one per line; the name may contain spaces (`Spirit Speak`, `Evaluate Intelligence`); the value is the last token |
| `Stat <Str|Dex|Int> <value>` | three lines per profession |

There are 7 professions (Samurai, Ninja, Paladin, Necromancer, Warrior, Mage, Blacksmith), each with 4 skills of value 30 and stats summing to 90.

Reader in Fiddler: not interpreted (text only in the Files tab)

## map0.txt

List of cells of map 0 as text, ASCII, CRLF, 482505 lines of three integers separated by commas: `x,y,z`.

- `x` from 0 to 119 and `y` from 0 to 4095; `z` from -16 to 3 (the most common value is -5).
- The order follows blocks of 8x8 cells: the block varies first in Y and then in X (the file covers block columns 0 to 14); within the block the 64 cells go line by line (x from 0 to 7 for each y).
- The last line is truncated (`112,2969,-5` without a complete terminator).

```
0,0,-5
1,0,-5
...
7,7,-5
0,8,-5
```

Reader in Fiddler: not interpreted (text only in the Files tab)

## Version.txt, Version.dat, Verinfo, client.exe.txt

| File | Content |
|---|---|
| Version.txt | client version on one line: `1.25.35` + CRLF (9 bytes) |
| Version.dat | 4 bytes: `xx` + CRLF |
| Verinfo | `Win32_UOSA 65` + CRLF: platform and build number separated by a space |
| client.exe.txt | supporting text of 8 lines: a header of two sentences and the line `Format style is: DLL Name::API Name->Call to API`, followed by lines `DLL::symbol->instruction` (e.g.: `UL.dll::?SendPingMessage@@YA_NPADH00H@Z->call dword ptr [b65180]`) |

Reader in Fiddler: not interpreted (text only in the Files tab)

## metrics.txt and gamegen.txt

metrics.txt is a machine configuration report, ASCII, CRLF, 27 lines, in the format `Key: value`, with blank lines between groups: `Instance Id`, `OS Version`, `CPU Manufacturer`, `CPU Family`, `CPU Model`, `CPU Clock Speed`, `CPU Quantity`, `Physical Memory`, `Desktop Resolution and Color Depth`, `DirectX Version`, `Video Card Description`, `Video Card Vendor Id`, `Video Card Device Id`, `Video Card Memory`, `Distribution`, `Client Running`, `Clients Installed`, `Partial Install`, `Language Preference`, `DVD-ROM drive`.

```
Instance Id: 34
OS Version: 0.0.0
Physical Memory: 0MB
Partial Install: no
```

gamegen.txt is 11 bytes and contains only the text `metrics.txt`, with no line ending.

Reader in Fiddler: not interpreted (text only in the Files tab)

## Login.cfg, Uomax.cfg, Uoreg.cfg, Uoupg.cfg, uo.cfg

Client configuration in `Key=Value` format, one per line, CRLF, no sections. Line comments with `;` (uo.cfg). Boolean values: `on`/`off`, `yes`/`no`, `true`.

| File | Content |
|---|---|
| Login.cfg | `LoginServer=localhost,2593` (host and port separated by a comma; 28 bytes) |
| Uomax.cfg | execution options |
| Uoreg.cfg | execution options (includes `OrderCache`) |
| Uoupg.cfg | subset with only the translation options (9 lines) |
| uo.cfg | execution options and the state of the last session |

Keys present in Uomax.cfg/Uoreg.cfg: `CdRomDataPath`, `CacheSize`, `HdUsage`, `PageFlip`, `Sound`, `FullScreen`, `Music`, `SavePassword`, `Password`, `SoundVol` (0 to 255), `MusicVol` (0 to 255), `FilterPassword`, `ObscenityFilter`, `OrderCache` (Uoreg only), `UnicodeSpeech`, `TranslationServer`, `TranslationServerIPAddress` (host,port), `TranslationServerTranslatorName`, `TranslationServerSystranNFWMarker`, `TranslationServerSystranTopicalGlossaries`, `TranslationServerSystranDocumentType`, `TranslationServerAutomaticTranslation`, `TranslationServerClickTranslation`.

Additional keys in uo.cfg: `ClientFirstLaunch`, `NextLoginKey`, `PlayerName`, `ClientLogin`, `LastTip`, `ShowTips`, `CurrentChatChannel`, `LastServerID`, `DefaultChar`, `AcctID`. `Password` and `FilterPassword` are credential fields and must not be filled in in distributed files.

Example:

```
LoginServer=localhost,2593
```

```
CacheSize=25
HdUsage=max
SoundVol=255
UnicodeSpeech=on
TranslationServerIPAddress=127.0.0.1,28888
```

Reader in Fiddler: not interpreted (text/table only in the Files tab)

## Arttable.cfg

Art redirection table, ASCII, CRLF, 110 lines. Two formats, always with spaces around the braces:

```
<ID> { <NEW> } <FLAG>
<ID> { <A>, <B>, <C> } <FLAG>
```

| Form | Lines | Example |
|---|---|---|
| One target, `FLAG` 0 | 49 | `15821 { 2140 } 0` |
| One target, `FLAG` -1 | 1 | `3 { 3 } -1` (first line) |
| Three targets, `FLAG` 0 | 60 | `16927 { 16927, 24968, 24969 } 0` |

In the 60 three-target lines the first value is always equal to the key ID.

Fields not interpreted: meaning of `FLAG` and of the two additional values.

Reader in Fiddler: not interpreted (text only in the Files tab)

## Helpindx.dat

Index of help topics (the client's Help codex). UTF-8 text (only ASCII present), CRLF, 351 lines.

- `# Title` (comment) precedes each topic (176 comments).
- Topic line, separated by tabs: `[@]<id> <cliloc1> <cliloc2> <cliloc3> [<related id> ...]`.

| Field | Description |
|---|---|
| optional `@` | prefix present in 133 of the 174 topics; meaning not interpreted |
| `id` | topic number, from 1 to 176, unique |
| `cliloc1` | cliloc of the topic title (3010031 = "Paperdoll") |
| `cliloc2` | cliloc of the section (3010030 = "Movement"; 3013030 for the house topics) |
| `cliloc3` | cliloc of the text (3010101 = paperdoll text) |
| remaining | IDs of related topics (zero to 16) |

Cliloc texts may contain hyperlinks `<a href = "?165">` where the number is a topic ID.

```
# Movement
1	3010030	3010030	3010100
# Paperdoll
2	3010031	3010030	3010101	3	4	5	6	7	8	9	10	11	12
# Help Button
@3	3010032	3010030	3010102
```

Reader in Fiddler: not interpreted (text only in the Files tab)

## Credits.dat

Credits text, UTF-8, CRLF, 693 lines (10732 bytes). Lines starting with `! ` are directives; the others are displayed text.

| Line | Effect |
|---|---|
| `! song N [song M ...]` | list of songs |
| `! defaultfont N` | default font |
| `! defaultcolor #RRGGBB` | default color |
| `! color #RRGGBB` | color of the following excerpt (129 directives in total) |
| `---=== título ===---`, `-== seção ==-` | titles |

```
! song 49 song 50
! defaultfont 0
! defaultcolor #9999FF
! color #FFFF00
---=== Ultima Online: High Seas Booster Pack ===---
```

Reader in Fiddler: not interpreted (text only in the Files tab)

## Obscene.lst, uobscene.lst, ignore.lst, conf_ignore.lst

| File | Structure |
|---|---|
| Obscene.lst | 61 bytes, CRLF, one word per line, with the characters shifted by a simple cipher (not readable text) |
| uobscene.lst | 1048 bytes; same content in 16-bit little-endian units, encrypted, with a `0x0017` terminator between words |
| ignore.lst | list of ignored players; 34-byte records: name in 30 bytes, terminated and padded with NUL, followed by a little-endian `uint32` (in the file present, 1 record: name `Nao Ha Vagas`, value 160412) |
| conf_ignore.lst | empty (0 bytes) |

Fields not interpreted: cipher of Obscene.lst/uobscene.lst; meaning of the `uint32` of ignore.lst.

Reader in Fiddler: not interpreted (only hex in the Files tab)

## chardata.mul

Binary file of 20 bytes that begins with the text line ` ver 1` followed by LF (`20 76 65 72 20 31 0A`), followed by 13 binary bytes: five `B2 02` pairs (five 16-bit little-endian values, 0x02B2) and `34 00 25`.

Fields not interpreted: meaning of the 13 bytes after the version line.

Reader in Fiddler: not interpreted (only hex in the Files tab)
