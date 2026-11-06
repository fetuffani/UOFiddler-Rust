# 0xDD - Compressed Gump
Direction: server -> client. Size: variable.
Purpose: Opens a gump with a layout and text lines compressed with zlib.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDD |
| 1 | 2 | u16 | length | Total size |
| 3 | 4 | u32 | serial | Serial of the gump's owner |
| 7 | 4 | u32 | gumpId | TypeID of the gump |
| 11 | 4 | u32 | x | X position |
| 15 | 4 | u32 | y | Y position |
| 19 | 4 | u32 | layoutCompressedLen | Compressed bytes + 4 (the data block has `value - 4` bytes) |
| 23 | 4 | u32 | layoutRawLen | Size of the decompressed layout |
| 27 | n | zlib | layoutData | Compressed layout |
| ... | 4 | u32 | lineCount | Number of text lines |
| ... | 4 | u32 | textCompressedLen | Compressed bytes + 4 (only present if `lineCount > 0` on the client) |
| ... | 4 | u32 | textRawLen | Size of the decompressed text block |
| ... | m | zlib | textData | Compressed lines |

Decompressed layout: ASCII text of the gump commands in sequence (`{ ... }{ ... }`), terminated by a 0x00 byte (ServUO and Source-X include the terminator in the length). The command grammar is in [gump-layout.md](../gump-layout.md).

Decompressed texts: a sequence of `lineCount` entries, each `u16 chars` (number of characters) + `chars` UTF-16BE characters, without a terminator.

Notes:
- With an empty layout, ServUO/ModernUO write only `u32 0` in place of the (length, data) pair. When there are no lines, they write `lineCount = 0` followed by `u32 0`.
- If `lineCount = 0`, ClassicUO does not read the text length pair.
- The compression is standard zlib (header `78 9C`).

Implementation reference:
- ServUO `Server/Network/Packets.cs` (`DisplayGumpPacked`)
- ModernUO `Projects/UOContent/Gumps/Base/DynamicGump.cs`, `StaticGump.cs`, `OutgoingGumpPackets.cs` (`WritePacked`)
- Source-X `send.cpp` (`PacketGumpDialog::writeCompressedControls`)
- ClassicUO `PacketHandlers.cs` (`OpenCompressedGump`)

## Example

Gump with serial 0x40001234, TypeID 1, position (100, 50), layout `{ page 0 }{ text 10 10 0 0 }` + NUL (29 bytes, 32 compressed) and one text line `Hi` (6 bytes, 14 compressed), 85 bytes in total:

```
DD 00 55
40 00 12 34 00 00 00 01 00 00 00 64 00 00 00 32
00 00 00 24 00 00 00 1D
78 9C AB 56 28 48 4C 4F 55 30 50 A8 AD 56 28 49 AD 28 51 30 34 00 21 10 AC 65 00 00 84 15 07 C5
00 00 00 01
00 00 00 12 00 00 00 06
78 9C 63 60 62 F0 60 C8 04 00 01 51 00 B4
```

| Excerpt | Meaning |
|---|---|
| `DD 00 55` | id and length = 85 |
| `40 00 12 34` | serial |
| `00 00 00 01` | gumpId |
| `00 00 00 64` / `00 00 00 32` | x = 100, y = 50 |
| `00 00 00 24` | compressed + 4 = 36 (32 bytes of zlib) |
| `00 00 00 1D` | raw layout = 29 bytes |
| `78 9C ...` | zlib of the layout |
| `00 00 00 01` | lineCount = 1 |
| `00 00 00 12` | compressed + 4 = 18 (14 bytes of zlib) |
| `00 00 00 06` | raw block = 6 bytes (`00 02 00 48 00 69`: 2 characters, "H", "i") |
| `78 9C ...` | zlib of the text block |
