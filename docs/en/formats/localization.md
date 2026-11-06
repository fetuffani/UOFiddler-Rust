# Client localization

## 1. IFF container of localized strings

### Purpose and files

An IFF-style container (chunks with big-endian sizes) that stores an ordered list of interface strings in one language. The file name is `<resource>.<language>`; upper and lower case vary (`Tooltips.enu`, `INTLOC02.ENU`, `Intloc02.cht`).

Language, file suffix and code written in the `INFO` chunk:

| Suffix | Language | Code in `INFO` |
|---|---|---|
| `enu` | English (US) | `enu` |
| `deu` | German | `deu` |
| `esp` | Spanish | `esp` |
| `fra` | French | `fra` |
| `jpn` | Japanese | `jpn` (in `Intro.jpn`: `Jpn`) |
| `kor` | Korean | `kor` |
| `cht` | Traditional Chinese | `cht` (in `Gesture.cht`: `bg5`) |
| `chs` | Simplified Chinese | used only by `Cliloc.chs` (which is not IFF) |

The `INFO` code must not be used to choose the encoding; the character width is in the `INFO` itself (below).

Resource families present (resource name in `INFO`, number of strings in the `enu` version, languages present):

| Files | Resource | Strings | Content |
|---|---|---|---|
| `Tooltips.*` | `tooltips` | 61 | tooltips for the login screen buttons (`Visit www.uo.com`, `View online help`, `Previous screen`) |
| `Gesture.*` | `gesture` | 272 | gesture/emote names; the string index is the gesture number (32 = `bow`, 33 = `salute`); most are empty |
| `Chat.*` | `chat` | 105 | chat window texts (`Create a New Conference`, `Ignore a Player`) |
| `Professn.*` | `professn` | 57 | profession names and descriptions (`ADVANCED`, `ADVENTURER`, `MERCHANT`...) |
| `Skilname.*`, `SKILNAME.*` | `skilname` | 52 in `enu` (49 in the others) | skill names, in skill ID order (index 0 = `Alchemy`); the last item is empty |
| `SKILL16.*` | `skill16` | 9 | help text for a skill, with `POSN` and `COLR` chunks |
| `SKILL30.*` | `skill30` | 4 | same, with `POSN` and `COLR` |
| `Tilehelp.*` | `tilehelp` | 95 (90 in `jpn`) | help texts per tile group, with a `TKEY` chunk |
| `Intloc00.*`, `intloc00.enu` | `intloc00` | 620 | login and connection messages (`Quit Ultima Online?`, `Connecting...`) |
| `Intloc01.*` | `intloc01` | 100 | chat labels (`Participants`, `Whisper`, `WhoIs`) |
| `Intloc02.*`, `INTLOC02.*` | `intloc02` | 150 | `You see:`, character creation and deletion |
| `INTLOC06.*` | `intloc06` | 200 | skill training buttons (`Train Alchemy`...) |
| `INTLOC10.*` | `intloc10` | 210 | Codex of Wisdom interface (`Context Help`, `Index`, `History`) |
| `intloc11.enu` | `intloc11` | 33 | emote words (`yawn`, `faint`, `bow`, `salute`, `applaud`) |
| `Gt_0000`, `Gt_1010`, `Gt_2000`, `Gt_2310`, `Gt_2400`, `Gt_4000`, `Gt_5000`, `Gt_5400` (`.enu` etc.) | `gt_NNNN` | 55, 303, 233, 15, 130, 208, 208, 21 | client gump texts; in the `enu` version almost all are empty (only `Gt_4000.enu` has 32 strings, e.g. `Read Mail`, `Address List`) |
| `Intro.*` | `intro` | 28 (27 in `deu`) | introduction text, with `TIME` and `COLR` chunks |
| `OPTNUOTD.*`, `optuotd.JPN` | `optnuotd` | 208 (212 `enu`, 194 `jpn`, 196 `kor`) | options window and tip of the day; `optuotd.JPN` is identical to `OPTNUOTD.JPN` |
| `options.*` | `options` | 220 to 222 | options window (in `options.deu` the resource name is `Optionen`) |

Languages by family: `Tooltips`, `Chat`, `Gesture`, `Professn`, `Tilehelp`, `Intro`, `Gt_*`, `OPTNUOTD`, `options`: all seven (`cht deu enu esp fra jpn kor`); `Intloc00` and `Intloc01`: seven (the English of `Intloc00` is `intloc00.enu`); `Intloc02`: `cht deu esp fra` plus `INTLOC02.ENU/JPN/KOR`; `Skilname`: `cht deu esp fra` plus `SKILNAME.ENU/JPN/KOR`; `SKILL16`, `SKILL30`, `INTLOC06`, `INTLOC10`: `CHT ENU JPN KOR`; `intloc11`: `enu` only.

### Binary layout

All chunk size integers are big-endian; the integers inside `INFO` and the auxiliary chunks are little-endian. Each chunk payload is padded with a 1-byte NUL when its size is odd (the padding byte is not counted in the declared size).

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `FORM` |
| 4 | 4 | size (BE) = file size - 8 |
| 8 | 4 | form type `DATA` |
| 12 | 4 | `FORM` |
| 16 | 4 | size (BE) = file size - 20 |
| 20 | 4 | form type `LANG` |
| 24 | 4 | chunk ID `INFO` (bytes 20 to 27 form the text `LANGINFO`) |
| 28 | 4 | size of `INFO` (BE) |
| 32 | N | `INFO` payload (below), plus 1 padding byte if N is odd |
| then | 4 + 4 + M | `TEXT` chunk: ID, size M (BE), strings, padding if M is odd |
| then | | optional chunks (`POSN`, `COLR`, `TIME`, `TKEY`) |

`INFO` payload (size = 21 + length of the resource name):

| Field | Type | Description |
|---|---|---|
| language code | ASCII + NUL | `enu`, `deu`... |
| width | `u32` | bytes per character in the strings: `1` (8 bits) or `2` (UTF-16LE) |
| flag | `u32` | `0`; `1` only in the `Tilehelp.*` files |
| resource name | ASCII + NUL | `tooltips`, `intloc00`... |
| constant | `u32` | `1` in all files |
| count | `u32` | number of strings in the `TEXT` chunk |

`TEXT` payload: the strings in order, each ended by a NUL of the width size (1 byte or 2 bytes `00 00`); empty strings are consecutive terminators; the last string is terminated too. The count in `INFO` equals the number of terminators. A string is referenced by its index (0-based).

Encoding according to width:

| Width | Languages observed | Encoding |
|---|---|---|
| 1 | `enu`, `deu`, `esp`, `fra` and `Gt_4000.jpn` | ASCII / Windows-1252 (Latin-1 for accented letters) |
| 2 | `jpn`, `kor`, `cht` | UTF-16 little-endian, no BOM |

All `jpn`, `kor` and `cht` files have width 2, with the only exception of `Gt_4000.jpn` (width 1, English text); all `enu`, `deu`, `esp` and `fra` files have width 1. The seven `Tilehelp.*` files have flag 1.

### Optional chunks

Amounts per string of the list (`n` = count in `INFO`):

| Chunk | Files | Size | Content |
|---|---|---|---|
| `POSN` | `SKILL16.*` (72 bytes, n=9), `SKILL30.*` (32 bytes, n=4) | `n * 8` | `u32` pairs (x, y) with the position of each string |
| `COLR` | `SKILL16.*` (108), `SKILL30.*` (48), `Intro.jpn` (336) | `n * 12` | three `u32` per string, values 0 to 255 (color components) |
| `TIME` | all `Intro.*` (224 bytes; 216 in `Intro.deu`, which has 27 strings) | `n * 8` | two `u32` per string (start and end), increasing; e.g. `Intro.enu`: 680 750, 760 840, 850 945... |
| `TKEY` | `Tilehelp.*` | variable | consecutive records: `u32` string index (0, 1, 2...), `u32` amount K, K x `u32` static item IDs associated with the string. `Tilehelp.enu` has 95 records in 5500 bytes |

The unit of `TIME` and the meaning of the colors are not interpreted.

### Markup inside the options strings

`OPTNUOTD.*` and `options.*` mix text and directives in the list: `##`, `#TAB0`, `#CHECKBOX%Sound`, `#SLIDER[0-255]%SoundVol`; the string before a directive is the label of the control.

### Example (`Tooltips.enu`, 1448 bytes)

```
46 4F 52 4D 00 00 05 A0  "FORM" size=0x5A0
44 41 54 41              "DATA"
46 4F 52 4D 00 00 05 94  "FORM" size=0x594
4C 41 4E 47              "LANG"
49 4E 46 4F 00 00 00 1D  "INFO" size=29
65 6E 75 00              "enu\0"
01 00 00 00              width = 1
00 00 00 00              flag = 0
74 6F 6F 6C 74 69 70 73 00  "tooltips\0"
01 00 00 00              constant = 1
3D 00 00 00              count = 61
00                       padding
54 45 58 54 00 00 05 61  "TEXT" size=0x561
"Visit www.uo.com\0Change account and credit card information\0..."
```

Reader in Fiddler: `crates/uo_core/src/client_files.rs` (`parse_iff_text`; reads the `INFO` and the width, decodes Windows-1252 or UTF-16LE, limits to the declared count; the `POSN`, `COLR`, `TIME` and `TKEY` chunks are not interpreted)

## 2. Langcode.iff

Windows language code table, in IFF. 4612 bytes.

Structure:

```
FORM <size> DATA
  FORM <size> LANG
    FORM <size> WIN3
      CODE <size> <payload>     (135 chunks)
```

The `FORM` sizes follow the same rule as the other files (4604, 4592 and 4580 in the present file). Each `CODE` chunk (ID `CODE`, BE size, payload with even padding) contains:

| Field | Type | Description |
|---|---|---|
| code | 3 bytes ASCII + NUL | language/region code (`ARA`, `ENU`, `DEU`, `ESP`, `FRA`, `JPN`, `KOR`, `CHT`, `CHS`...) |
| value | `u32` little-endian | 80 chunks with `63` (`?`); the rest with values such as 101 (`ENU`), 102 (`FRA`), 103 (`DEU`), 99 (`CHT`, `CHS`), 106 (`JPN`), 115 (`ESP`) |
| language | ASCII + NUL | language name in English (`Arabic`, `English`, `Korean (Extended Wansung)`) |
| region | ASCII + NUL | region name (`Saudi Arabia`, `United States`, `Japan`, `PRC`) |

The 135 records have 133 distinct codes.

Examples:

```
ARA  63   Arabic    Saudi Arabia
ENU  101  English   United States
DEU  103  German    Germany
JPN  106  Japanese  Japan
KOR  63   Korean (Extended Wansung)  Korea
CHT  99   Chinese   Taiwan
CHS  99   Chinese   PRC
```

Fields not interpreted: meaning of the `value` field.

Reader in Fiddler: not interpreted (hex only, in the Files tab)

## 3. Cliloc.*

### Purpose and files

Table of numbered texts (client localization) used by tooltips, gumps, help and professions. One file per language: `Cliloc.enu`, `Cliloc.deu`, `Cliloc.esp`, `Cliloc.fra`, `Cliloc.jpn`, `Cliloc.kor`, `Cliloc.cht`, `Cliloc.chs`. The suffixes are those from section 1.

| File | Entries | Lowest / highest number |
|---|---|---|
| Cliloc.enu | 110292 | 500000 / 3011032 |
| Cliloc.jpn | 47526 | 500000 / 3011032 |
| Cliloc.cht | 42122 | 500000 / 3011032 |
| Cliloc.kor | 33193 | 500000 / 3011032 |
| Cliloc.deu | 18628 | 500001 / 3010161 |
| Cliloc.esp | 18247 | 500000 / 3010161 |
| Cliloc.fra | 18191 | 500000 / 3010161 |
| Cliloc.chs | 453 | 500012 / 3010161 |

The files of other languages are subsets; a missing number must be looked up in `Cliloc.enu`.

### Binary layout (little-endian)

| Offset | Type | Field |
|---|---|---|
| 0 | `u32` | `2` (header) |
| 4 | `u16` | `1` (header) |
| 6 | | sequence of entries up to the end of the file |

Entry:

| Field | Type | Description |
|---|---|---|
| number | `u32` | cliloc number |
| flag | `u8` | `0` in all entries of the present files |
| length | `u16` | size of the text in bytes |
| text | `length` bytes | UTF-8, without a trailing NUL |

The entries are in ascending order of number and the file ends exactly after the last one. Empty texts exist (60864 in `Cliloc.enu`). The present files are not compressed; Fiddler treats a file whose first 4 bytes are all non-zero as compressed (Mythic compression).

### Text content

- Argument markers `~N_name~`, where N (starting at 1) is the index of the argument replaced by the caller: `~1_name~ misses the target altogether.` (500604), `~1_VAL~` (1114778). There are 1570 texts with markers in `Cliloc.enu`.
- Some texts contain gump HTML (`<BASEFONT COLOR=#DFDFC0>`, `<DIV ALIGN=RIGHT>`, `<a href = "?165">`).

### Use of the cliloc number

| Where | How |
|---|---|
| Gumps (`xmfhtmlgump x y w h cliloc background scrollbar`, `AddHtmlLocalized`) | the text of the HTML element comes from the cliloc instead of a gump string (in Fiddler's gump editor, the value 0 is equivalent to using literal text) |
| Item tooltips | the server sends the number and the arguments; the client builds the text with the `~N_name~` markers |
| `Prof.txt` | `NameId` and `DescId` (e.g. 1062948 = `SAMURAI`) |
| `Helpindx.dat` | three numbers per topic (e.g. 3010031 = `Paperdoll`) |
| `walls.txt`, `roof.txt`, `misc.txt` | `TID` column (e.g. 1060054 = `Dark Wood Walls`) |
| `Intrface.def` | `settooltip`, `tooltip` (e.g. 1062709) |

Example (`Cliloc.enu`):

```
1062948  SAMURAI
3010030  Movement
1060054  Dark Wood Walls
500000   Reputation aversion triggered.
```

Reader in Fiddler: `crates/uo_core/src/cliloc.rs` (`ClilocManager::from_file`); Mythic in `crates/uo_core/src/mythic.rs`; use in gumps in `crates/uo_gui/src/gumpstudio_tab.rs`

## 4. speech.mul

### Purpose

List of speech keywords recognized by the client, each associated with a numeric identifier. Single file, 119714 bytes, 6125 entries, no header.

### Layout (big-endian)

Consecutive entries up to the end of the file:

| Field | Type | Description |
|---|---|---|
| id | `i16` BE | keyword identifier (0 to 512; 513 distinct values; several records may have the same id) |
| length | `i16` BE | size of the text in bytes (maximum 65 in the present file) |
| text | `length` bytes | UTF-8, without a trailing NUL |

Notes:

- The texts appear in several languages, with asterisks delimiting the term in many entries (`*barrier*`, `*バリア*`; 4988 entries contain `*`).
- Zero-length entries exist.
- E.g. id 0 repeats for `*提領*`; id 512 groups `*barrier*`, `*バリア*` and `ばりあ*`.

Example (first bytes):

```
00 00 00 08 2A E6 8F 90 E9 A0 98 2A   id=0 len=8 "*提領*"
00 01 00 08 2A E7 B5 90 E5 AD 98 2A   id=1 len=8 "*結存*"
```

Reader in Fiddler: `crates/uo_core/src/speech.rs` (`SpeechManager::from_file`; stops reading if a length exceeds 256)
