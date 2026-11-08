# Gump layout commands of client 7.0.10.3 (extracted from the binary)

Parser: 0x4cb744..0x4cc917. Flow: copies the text between `{` and `}`, extracts the 1st word (0x56c800) and compares it with each command using `repe cmpsb`
(ECX = len+1, i.e. exact equality). If none matches, the block is ignored. Command strings: .rdata 0x66e594..0x66e768. Args via sscanf (0x5fd94f).

| # | Command | Args (sscanf) | Creation / handler | Notes |
|---|---|---|---|---|
| 1 | page | 1 integer (0x5ff60a) | — | changes the current page |
| 2 | nodispose | — | [gump+0xd0]=0 | |
| 3 | nomove | — | [gump+0xcc]=0 | |
| 4 | noclose | — | [gump+0xd4]=0 | |
| 5 | group | 1 integer | 0x4c7810 | |
| 6 | endgroup | — | clears the current group | |
| 7 | gumppic | own parser (tokens) | 0x4cb2f0 -> 0x4c79b0 | accepts `hue=N` and `class=NAME` |
| 8 | tilepic | `%d %d %s` | 0x4c7cd0 | 3rd arg read as a string |
| 9 | tilepichue | `%d %d %d %d` | 0x4c7cd0 | x y id hue |
| 10 | text | `%d %d %d %d` | 0x4c7ea0 | |
| 11 | croppedtext | 6 integers | 0x4c7fa0 | |
| 12 | textentry | 7 integers | 0x4c6460/… | |
| 13 | textentrylimited | 8 integers | 0x4c8080 | |
| 14 | radio | 6 integers | 0x4c84d0 | |
| 15 | button | 7 integers | 0x4c81a0 | creates a 0x7c B object (same base as gumppic) |
| 16 | buttontileart | 11 integers | 0x4c82c0 | |
| 17 | checkbox | 6 integers | 0x4c83e0 | |
| 18 | resizepic | 5 integers | 0x4c7b10 | |
| 19 | htmlgump | 7 integers | 0x4c7bf0 | |
| 20 | xmfhtmlgump | 7 integers | 0x4c7bf0 + cliloc | |
| 21 | xmfhtmlgumpcolor | 8 integers | same | |
| 22 | xmfhtmltok | own parser | same | cliloc with arguments |
| 23 | checkertrans | 4 integers | 0x4c7dd0 | |
| 24 | gumppictiled | 5 integers | 0x4c78d0 | |
| 25 | tooltip | 1 integer (cliloc) | — | |
| 26 | itemproperty | 1 integer (serial) | 0x43f570/0x440b90 | |
| 27 | mastergump | `%d` (1 integer) | 0x4ed570 | stores map1[gumpType] = master (map @0xa0e634); the master shares the saved position (map2 @0xa0e640) |

Gump defaults: "generic gump" (default name, [gump+8]); fields [gump+0xbc..0xe8] initialized at 0x4cb6eb.
These do NOT exist in this client: noresize, picinpic, gumppichued, virtue* (commands of newer clients).

## gumppic: parameters
- `hue=N` (string at 0x6a2e8c = "hue")
- `class=NAME` (word "class" at 0x66e594). Only recognized value: **VirtueGumpItem** (0x66e564).
  - Without class (or another value): GumpPic object (0x7c bytes, ctor 0x4c6920, vtable 0x66e4f0).
  - class=VirtueGumpItem: 0x84-byte object, ctor 0x4c7560 (calls 0x4c6920 and adds [+0x7c]=0, [+0x80]=0), vtable 0x66e540.
    Overridden methods: [0] dtor 0x4c9020; [4] click 0x4c8c90 (double click via GetDoubleClickTime; calls 0x4c7010); [5] 0x4c70b0 (stub, returns 0);
    [6] hover 0x4c8a40 (tooltip per level: table 0x6cf780, 16 B/entry {id 0x69..0x70, art1, art2, art3}); [7] 0x4c8c60 (mouse leaves).
  - Double click (0x4c7010) sends packet **0xB1** (Gump Menu Response): u32 player serial ([0xab48e4]+0xa0), u32 **0x1CD** (gump id), u32 [element+0x24] (virtue id), no switches/texts.

## mastergump (0x4cc8d5)
- Parse: sscanf("%d"); calls 0x4ed570(gumpKey=[parser arg], master) -> std::map<int,int> map1 @0xa0e634: map1[key] = master.
- At the end of the parse (0x4cc95c): 0x4eae50 resolves master = map1.find(key) (if missing, returns the key itself) and looks it up in map2 @0xa0e640; if there is a saved position (x != 0xFFFF) it applies it to [gump+0x34],[gump+0x38].
- When the gump is moved (0x4c6800..0x4c6849): 0x4ed590 stores map2[master(key)] = (x,y).
- Effect: gumps of different types with the same `mastergump N` reopen at the same remembered position. Without mastergump, each type remembers its own. (deduced from the code; not tested at runtime)
- ClassicUO keeps MasterGumpSerial, but its use is commented out ("does not work as expected").
