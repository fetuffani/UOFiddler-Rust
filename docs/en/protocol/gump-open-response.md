# Gump: opening, response, switches and text fields

Flow of a generic gump: the server describes the window (layout + text lines), the client displays it and returns, in a single packet, the button clicked, the switches turned on (radios and checkboxes) and the content of the text fields. The layout syntax is in [gump-layout.md](gump-layout.md).

Conventions: integers are big-endian; text of lines and fields is UTF-16 big-endian without a terminator; offsets are counted from byte 0 (id).

## 1. Overview and variants

| Packet | Direction | Use |
|---|---|---|
| `0xB0` Gump Menu Dialog | S->C | Layout in ASCII and uncompressed lines |
| `0xDD` Compressed Gump | S->C | Same content, with layout and lines compressed (zlib) |
| `0xB1` Gump Menu Response | C->S | Client response |
| `0xBF/0x04` Close Gump | S->C | Closes a gump by type (optionally simulating a button) |

ServUO chooses `0xDD` for clients with the `Unpack` flag (>= 5.0.0a) and `0xB0` for earlier ones; ModernUO always emits `0xDD`.

## 2. Sequence

1. S->C `0xDD` (or `0xB0`): gump `serial` (unique per instance), `typeId` (identifies the class), position `(x, y)`, layout and text lines. The server stores the gump in the `NetState` list, together with the count of switches and text fields declared in the layout.
2. The player interacts. The client locally stores the state of each radio/checkbox and the text of each field.
3. C->S `0xB1`: when a button of the "response" type is clicked, the client sends `serial`, `typeId`, the `buttonId` and the lists of switches turned on and text fields. Button `0` means close/cancel.
4. The server locates the gump by `(serial, typeId)`, validates the response, removes the gump from the list and calls the handler (`OnResponse`).
5. Optional: S->C `0xBF/0x04` with `typeId` and `buttonId` to close the gump from the server. The client closes the corresponding gump and, if `buttonId != 0`, simulates a click on that button (and therefore sends a `0xB1`).

Page-switch buttons (`type 0`) are handled only on the client and do not generate `0xB1`.

## 3. Packets

### 3.1 `0xB0` (S->C, `var`)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xB0` | |
| 1 | u16 | total size | |
| 3 | u32 | gump serial | |
| 7 | u32 | `typeId` | |
| 11 | u32 | x | |
| 15 | u32 | y | |
| 19 | u16 | `layoutLength` | bytes of the layout; ServUO's default variant includes the `0x00` terminator in the count, the "fast" variant does not write a terminator |
| 21 | char[`layoutLength`] | ASCII layout | `{ cmd args }` repeated |
| ... | u16 | `lineCount` | |
| ... | per line | `u16 length` (characters) + `length * 2` bytes UTF-16BE | |

### 3.2 `0xDD` (S->C, `var`)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xDD` | |
| 1 | u16 | total size | |
| 3 | u32 | gump serial | |
| 7 | u32 | `typeId` | |
| 11 | u32 | x | |
| 15 | u32 | y | |
| 19 | u32 | `4 + compressed layout length` | |
| 23 | u32 | uncompressed layout size | includes the final `0x00` of the layout |
| 27 | u8[] | compressed layout (zlib) | |
| ... | u32 | `lineCount` | |
| ... | u32 | `4 + compressed lines length` | with `lineCount == 0` the server writes only a `u32 0` in place of this block |
| ... | u32 | uncompressed lines size | |
| ... | u8[] | compressed lines (zlib) | each line: `u16 length` (characters) + UTF-16BE |

If the block to be compressed is empty, the server writes a single `u32 0` in place of the pair of fields. If compression fails, ModernUO writes `u32 4` and `u32 0`.

### 3.3 `0xB1` Gump Menu Response (C->S, `var`)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xB1` | |
| 1 | u16 | total size | |
| 3 | u32 | gump serial | |
| 7 | u32 | `typeId` | |
| 11 | u32 | `buttonId` | `0` = close |
| 15 | u32 | `switchCount` | |
| 19 | u32[`switchCount`] | `switchId` | only the radios and checkboxes that are **checked** |
| ... | u32 | `textCount` | |
| ... | per field | `u16 entryId`, `u16 length` (characters), `length * 2` bytes UTF-16BE | all text fields of the gump, even empty ones |

The client limits each field to 239 characters.

### 3.4 `0xBF/0x04` Close Gump (S->C, 13 bytes)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x000D` |
| 3 | u16 | `0x0004` |
| 5 | u32 | `typeId` |
| 9 | u32 | `buttonId` |

## 4. Layout -> response mapping

| Layout element | What comes back in the `0xB1` |
|---|---|
| `button` (type 1) | `buttonId` = the command's `param2` |
| `radio`, `checkbox` | `switchId` included in `switches` only if the element is checked |
| `textentry`, `textentrylimited` | pair `(entryId, current text)` always present |
| `button` (type 0) | no packet (local page switch) |
| Virtue gump (`typeId = 461`, `0x1CD`) | special case with no gump registered on the server: `serial` = target player, `buttonId` = virtue id (ModernUO also accepts `buttonId = 1` with a switch to request the virtues gump) |

## 5. Server rules

1. Look up the gump by `serial` and `typeId` in the `NetState` list; if it does not exist, ignore the response (except for the virtue case above).
2. `buttonId == 0` is always valid. Any other `buttonId` must exist as a response button in the gump (`GumpButton` or `GumpImageTileButton`); otherwise the response is invalid.
3. `switchCount` must not exceed the number of radios and checkboxes declared in the layout (counted when assembling the packet); `textCount` must not exceed the number of text fields declared; each field has at most 239 characters.
4. Invalid response: ServUO closes the client's connection; ModernUO logs the exception and discards the response.
5. Valid response: remove the gump from the list before calling `OnResponse`, so that the handler can resend the same gump.
6. To reuse the `serial` of an open gump, close the previous one first; the `typeId` is per class, the `serial` is per instance.
7. The server must store the number of switches and text fields of each gump so that it can validate the response.

## 6. Example

Gump `serial = 0x00000001`, `typeId = 0x000000A5`, at `(50, 50)`, with layout `{ page 0 }{ textentry 10 10 100 20 0 1 0 }{ radio 10 40 208 209 0 7 }{ button 10 70 4005 4007 1 0 5 }` and the text line `"x"`.

Client response: button 5, radio 7 checked, field 1 with the text `"Oi"`:

```
B1 00 23                  size = 35
00 00 00 01               serial
00 00 00 A5               typeId
00 00 00 05               buttonId
00 00 00 01               switchCount
00 00 00 07               switchId 7
00 00 00 01               textCount
00 01 00 02               entryId 1, 2 characters
00 4F 00 69               "Oi" in UTF-16BE
```

Response to close without choosing anything: `B1 00 17 | serial | typeId | 00 00 00 00 | 00 00 00 00 | 00 00 00 00`.

## 7. Pitfalls

- `switchId` and `entryId` are the indices declared in the layout, not the position in the list; the client sends only the checked radios/checkboxes.
- The character count of each text field is in units of 2 bytes; using bytes breaks the alignment of the following field.
- The server's gump keeps counters of switches and fields; a gump reopened with a different layout reusing the serial must recalculate these counters.
- In `0xDD`, `lineCount == 0` is not followed by the size pair: only a `u32 0`. The client does not read this field.
- In `0xB0`, the `layoutLength` count varies between implementations (with or without `0x00`); the client reads exactly that many bytes.
- Forgetting to remove the gump before `OnResponse` allows duplicate responses to be accepted.

## 8. Implementation reference

| Component | File / symbol |
|---|---|
| ServUO | `Server/Gumps/Gump.cs` (`Compile`, `OnResponse`), `Server/Network/Packets.cs` (`DisplayGump`, `DisplayGumpFast`, `DisplayGumpPacked`, `CloseGump`), `Server/Network/PacketHandlers.cs` (`DisplayGumpResponse`), `Server/Gumps/RelayInfo.cs` |
| ModernUO | `Projects/UOContent/Gumps/Base/Legacy/Gump.cs`, `StaticGump.cs`, `DynamicGump.cs`, `OutgoingGumpPackets.cs` (`WritePacked`), `GumpSystem.IncomingPackets.cs` (`DisplayGumpResponse`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenGump`, `OpenCompressedGump`, `0xBF` case `4`), `src/ClassicUO.Client/Game/UI/Gumps/Gump.cs` (`OnButtonClick`), `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_GumpResponse`) |
