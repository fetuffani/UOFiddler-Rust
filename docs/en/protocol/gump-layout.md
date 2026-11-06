# Gump layout commands (client 7.0.x)

The layout of a gump (packets `0xB0` Gump Menu Dialog and `0xDD` Compressed Gump) is a sequence of text blocks enclosed in braces, `{ command arg arg ... }`. The client reads the first word of the block and compares it by exact equality with the commands below; blocks with an unknown command are ignored.

All arguments are decimal integers, except where indicated. The coordinates `x` and `y` are relative to the corner of the gump.

## Commands

| Command | Arguments | Meaning | Example |
|---|---|---|---|
| `page` | `n` | Starts page `n`; the following elements belong to it (page 0 is always visible) | `{ page 1 }` |
| `nodispose` | — | Disables disposal of the gump (`Disposable = false` in ServUO) | `{ nodispose }` |
| `nomove` | — | The gump cannot be dragged | `{ nomove }` |
| `noclose` | — | Disables closing of the gump (`Closable = false` in ServUO) | `{ noclose }` |
| `group` | `n` | Starts group `n` of radio buttons | `{ group 1 }` |
| `endgroup` | — | Ends the current group | `{ endgroup }` |
| `gumppic` | `x y gumpId [hue=N] [class=NAME]` | Gump image. `hue=` applies a hue; `class=VirtueGumpItem` creates the virtue item (double-click sends `0xB1`) | `{ gumppic 0 0 2600 hue=33 }` |
| `tilepic` | `x y itemId` | Item image (art) | `{ tilepic 10 10 3823 }` |
| `tilepichue` | `x y itemId hue` | Item image with hue | `{ tilepichue 10 10 3823 33 }` |
| `text` | `x y hue textId` | Static text; `textId` indexes the packet's text list | `{ text 40 20 0 0 }` |
| `croppedtext` | `x y width height hue textId` | Text cropped to the indicated box | `{ croppedtext 40 20 200 20 0 0 }` |
| `textentry` | `x y width height hue entryId textId` | Text input field with initial value `textId` | `{ textentry 40 60 200 20 0 1 2 }` |
| `textentrylimited` | `x y width height hue entryId textId limit` | Text field with a character limit | `{ textentrylimited 40 60 200 20 0 1 2 30 }` |
| `radio` | `x y gumpIdOff gumpIdOn initialState switchId` | Radio button | `{ radio 10 100 208 209 0 1 }` |
| `button` | `x y gumpIdUp gumpIdDown type param1 param2` | Button. `type` 1 = sends a response; 0 = page change (`param1` = page); `param2` = `buttonId` | `{ button 10 140 4005 4007 1 0 1 }` |
| `buttontileart` | `x y gumpIdUp gumpIdDown type param1 param2 itemId hue width height` | Button with an overlaid item image | `{ buttontileart 10 140 4005 4007 1 0 1 3823 0 40 40 }` |
| `checkbox` | `x y gumpIdOff gumpIdOn initialState switchId` | Checkbox | `{ checkbox 10 180 210 211 0 2 }` |
| `resizepic` | `x y gumpId width height` | Resizable background made of 9 parts of gump `gumpId` | `{ resizepic 0 0 9200 300 200 }` |
| `htmlgump` | `x y width height textId background scrollbar` | HTML text area; `background` and `scrollbar` are 0 or 1 | `{ htmlgump 10 10 280 100 3 0 1 }` |
| `xmfhtmlgump` | `x y width height cliloc background scrollbar` | HTML area with text taken from the cliloc | `{ xmfhtmlgump 10 10 280 100 1060635 0 1 }` |
| `xmfhtmlgumpcolor` | `x y width height cliloc background scrollbar color` | Same as `xmfhtmlgump`, with text color | `{ xmfhtmlgumpcolor 10 10 280 100 1060635 0 1 32767 }` |
| `xmfhtmltok` | `x y width height background scrollbar color cliloc [@args@]` | Cliloc with arguments separated by `@` | `{ xmfhtmltok 10 10 280 100 0 1 32767 1060635 @a@b@ }` |
| `checkertrans` | `x y width height` | Translucent region | `{ checkertrans 0 0 300 200 }` |
| `gumppictiled` | `x y width height gumpId` | Gump image repeated as a tiled pattern | `{ gumppictiled 0 0 300 200 2624 }` |
| `tooltip` | `cliloc` | Tooltip (cliloc) of the previous element | `{ tooltip 1042971 }` |
| `itemproperty` | `serial` | Associates the properties (tooltip) of item `serial` with the previous element | `{ itemproperty 1073741825 }` |
| `mastergump` | `id` | Gumps with the same `mastergump` share the saved position when reopened | `{ mastergump 100 }` |

## Notes

- `noresize`, `picinpic`, `gumppichued`, `gumppicphued` and `virtue*` do not exist in client 7.0.10.3 and are ignored.
- The only `class` recognized in `gumppic` is `VirtueGumpItem`. Double-click sends `0xB1` (Gump Menu Response) with the player's serial, the gump id `0x1CD` and the virtue id, with no switches or texts.
- Without `mastergump`, each gump type remembers its own position.
- The text for `text`, `croppedtext`, `textentry`, `htmlgump` etc. is outside the layout block: `textId` is the index into the list of text lines sent together with the layout.
