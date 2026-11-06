# Custom House Designer (house customization)

Multi-packet flow of the custom house editor (AOS onward). The server puts the client into "design mode", sends the entire house structure in one compressed packet (`0xD8`), and receives each edit as a short command (`0xD7`). This document describes the packets, the data structure of `0xD8`, the validation rules, and the expected responses.

Conventions: integers are big-endian on the wire; offsets are counted from byte 0 (the id byte). `S->C` = server to client, `C->S` = client to server. Sizes and ids of higher-level packets are in [packets.md](packets.md).

## 1. Overview and enablement

| Element | Packet | Direction | Note |
|---|---|---|---|
| Enter/leave design mode | `0xBF` subcommand `0x20` | S->C | Type `0x04` = begin, `0x05` = end |
| House revision number | `0xBF` subcommand `0x1D` | S->C | Client cache key |
| Request for house details | `0xBF` subcommand `0x1E` | C->S | Sent by the client when the revision differs |
| Full house contents | `0xD8` | S->C | Compressed (zlib), per plane |
| Edit commands | `0xD7` | C->S | "Encoded" envelope with a 16-bit subcommand |

Enablement:

- Sphere (`CItemMultiCustom`): `0xBF/0x20` and `0xD8` are only sent to a client with version >= 4.0.0 (`MINCLIVER_CUSTOMMULTI = 4000000`), KR or Enhanced Client. Without this, `BeginCustomize` returns without doing anything.
- ServUO/ModernUO: there is no client-version filter in the house packets; which building piece is valid depends on the server's expansion flag (`ExpansionInfo.CustomHousingFlag`, `HousingFlags` values: AOS `0x10`, SE `0x40`, ML `0x80`, Crystal `0x200`, SA `0x10000`, HS `0x20000`, Gothic `0x40000`, Rustic `0x80000`, Jungle `0x100000`, Shadowguard `0x200000`, TOL `0x400000`).
- The roof command (`0xD7/0x13`) is only accepted from a player if the server runs SE or higher (`Core.SE`); GMs can always use it.
- The client only processes `0xD8` if it already knows the foundation item (multi) and its dimensions in `multi.mul`; `0xD8` for an unknown serial is discarded. Send the multi object (`0xF3`/`0x1A`) before `0xD8`.

## 2. Envelopes

### 2.1 `0xBF` (General Information)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | total size |
| 3 | u16 | subcommand |
| 5 | ... | subcommand data |

### 2.2 `0xD7` (Encoded Command, C->S)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xD7` | |
| 1 | u16 | total size | `var` packet |
| 3 | u32 | serial of the target entity | the client sends the serial of its own character; the server resolves it with `World.FindEntity` and passes it to the handler |
| 7 | u16 | subcommand | see 3.5 |
| 9 | ... | "encoded" arguments | see below |
| end | u8 | `0x0A` | terminator sent by the client; the server does not read it |

"Encoded" arguments, read in sequence by `EncodedReader`:

| Tag (u8) | Value | Use |
|---|---|---|
| `0x00` | i32 | integer (used by all house commands) |
| `0x02` | u16 length + UTF-16 string | text (not used by the designer) |
| `0x03` | i16 x, i16 y, u8 z | point (not used by the designer) |

If the tag read is not the expected one, the reader returns 0/empty. Negative values travel in 32-bit two's complement (`x = -2` becomes `FF FF FF FE`).

The level command (`0x12`) is sent by the client as `u32 0` followed by `u8 floor`: read as tag `0x00` + i32, the result is the floor number.

## 3. Packets

### 3.1 `0xBF/0x20` Begin/End customization (S->C, 17 bytes)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xBF` | |
| 1 | u16 | `0x0011` | |
| 3 | u16 | `0x0020` | subcommand |
| 5 | u32 | foundation serial | |
| 9 | u8 | type | `0x04` begin, `0x05` end |
| 10 | u16 | `0x0000` | graphic |
| 12 | u16 | `0xFFFF` | x |
| 14 | u16 | `0xFFFF` | y |
| 16 | u8 | `0xFF` | z |

Types `1` (update), `2` (remove) and `3` (update multi pos) are read by the client with the same layout, but have no effect. On receiving `0x04` the client opens the designer gump (if it is not already open); on receiving `0x05` it closes the gump for the indicated serial.

### 3.2 `0xBF/0x1D` Design state general / revision (S->C, 13 bytes)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x000D` |
| 3 | u16 | `0x001D` |
| 5 | u32 | house serial |
| 9 | u32 | revision |

Client rule: if it does not have the house, if the house is not "custom", or if `revision != cached revision`, the client schedules a `0xBF/0x1E`. If the revision matches, it only regenerates the house from the local cache and requests nothing.

### 3.3 `0xBF/0x1E` Query design details (C->S, 9 bytes)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x0009` |
| 3 | u16 | `0x001E` |
| 5 | u32 | house serial |

Response: one `0xD8` with the visible state (`CurrentState`) for spectators, or the design state (`DesignState`) if the requester is the player editing this foundation. The server only responds if the player is on the same map, within the foundation's update range, and `CanSee`.

### 3.4 `0xD8` Custom House (S->C, `var`)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xD8` | |
| 1 | u16 | total size | |
| 3 | u8 | compression type | `0x03` = zlib per plane (only value emitted) |
| 4 | u8 | response flag | `0x01` or `0x00`; ServUO uses `0x01` in responses to commands and `0x00` on commit/close; ModernUO and Sphere always `0x00`. The client reads and ignores it |
| 5 | u32 | house serial | |
| 9 | u32 | revision | |
| 13 | u16 | total component count (items) | sum of the items across all planes and blocks |
| 15 | u16 | buffer length | `1 + sum(4 + compressed length of each block)`; equals `total size - 17` |
| 17 | u8 | block count | planes used + offset blocks |
| 18 | ... | blocks | repeated `block count` times |

Each block:

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | block index | see 4.2 |
| 1 | u8 | uncompressed size (bits 0-7) | |
| 2 | u8 | compressed size (bits 0-7) | |
| 3 | u8 | `((uncompressed >> 4) & 0xF0) \| ((compressed >> 8) & 0x0F)` | high nibble = bits 8-11 of the uncompressed size; low nibble = bits 8-11 of the compressed size |
| 4 | u8[] | zlib data | `compressed` bytes |

Both sizes are 12 bits (maximum 4095). As a reader, the client computes `dlen = b1 | ((b3 & 0xF0) << 4)` and `clen = b2 | ((b3 & 0x0F) << 8)`; byte `b0` is split into a high nibble (mode) and a low nibble (plane or block number).

### 3.5 `0xD7` designer subcommands (C->S)

| Sub | Name | Arguments (tag `0x00` + i32 each) | Effect |
|---|---|---|---|
| `0x02` | Backup | none | Copies the current design to the backup slot |
| `0x03` | Restore | none | Replaces the current design with the backup |
| `0x04` | Commit | none | Opens the cost confirmation gump (ServUO) or applies (Sphere) |
| `0x05` | Delete component | `itemId`, `x`, `y`, `z` | Removes a component |
| `0x06` | Build component | `itemId`, `x`, `y` | Adds a wall, floor, window, door or other item |
| `0x0A` | (action) | none | The client sends it; ServUO registers it as a no-op |
| `0x0C` | Close | none | Leaves design mode without commit |
| `0x0D` | Stairs | `multiId`, `x`, `y` | Adds a staircase (multi) |
| `0x0E` | Sync | none | Requests a full resend (`0xD8`) |
| `0x0F` | (action) | none | ServUO registers it as a no-op |
| `0x10` | Clear | none | Restores the empty foundation |
| `0x12` | Level | `floor` (1 to 4) | Floor change |
| `0x13` | Roof | `itemId`, `x`, `y`, `z` | Adds a roof piece |
| `0x14` | Roof delete | `itemId`, `x`, `y`, `z` | Removes a roof piece |
| `0x1A` | Revert | none | Returns to the currently visible state (discards uncommitted edits) |

`x` and `y` are signed offsets relative to the foundation multi's origin (the position of the item). `itemId` is the item id (art) except in `0x0D`, where it is the multi index (without the `0x4000` offset; Sphere adds `ITEMID_MULTI` internally).

Size examples: `0x06` = 25 bytes (`1+2+4+2 + 3*5 + 1`), `0x05` and `0x13`/`0x14` = 30 bytes, `0x0D` = 25 bytes, `0x12` = 15 bytes, commands without arguments = 10 bytes.

## 4. Data structure of `0xD8`

### 4.1 Coordinate system

- The origin is the foundation's multi. `xMin`, `yMin`, `xMax`, `yMax` are the limits of the `MultiComponentList` of the state being sent. The server's foundation has one extra row to the south (`Height + 1`) compared to the multi in `multi.mul`: this is where the sign and signpost are. `width = xMax - xMin + 1`, `height = yMax - yMin + 1` (already including the extra row).
- Height (Z) per floor `L` (1 to `MaxLevels`): `z = (L - 1) * 20 + 7`, that is `7, 27, 47, 67`. The extra southern row and the foundation floor are at `z = 0`.
- `MaxLevels` = 4 if the multi's `width >= 14` or `height >= 14`, otherwise 3 (ServUO). Sphere computes the limit through `GetLevelCount`.

### 4.2 Planes and blocks (ServUO/ModernUO emission)

Each component `(itemId, ox, oy, oz)` gets `x = ox - xMin`, `y = oy - yMin` and is classified:

| `oz` | Base plane |
|---|---|
| 0 | 0 |
| 7 | 1 |
| 27 | 2 |
| 47 | 3 |
| 67 | 4 |
| other | offset block |

`floor = (Height of the item's tiledata <= 0)`. Then:

| Plane | Condition | `size` column dimension | Adjustment | Block index (`b0`) | Uncompressed bytes |
|---|---|---|---|---|---|
| 0 | `oz = 0` | `height` | none | `0x20 \| 0` | `width * height * 2` |
| 1 to 4 | `oz` in 7/27/47/67 and floor | `height - 2` | `x -= 1; y -= 1` | `0x20 \| plane` | `(width-1) * (height-2) * 2` |
| 5 to 8 | `oz` in 7/27/47/67 and not floor | `height - 1` | `plane += 4` | `0x20 \| plane` | `width * (height-1) * 2` |

Grid position: `index = (x * size + y) * 2`, writing `itemId` as big-endian u16. The vector is column-major (x outer, y inner). Empty cell = `0x0000`.

A component goes to the offset block (instead of the grid) when `oz` is not one of the five values, `x < 0`, `y < 0`, `y >= size`, or `index + 1 >= 0x400` (1024-byte limit per grid).

Offset block (stairs/pieces outside the grid): each entry occupies 5 bytes and the block holds at most 750 entries (3750 raw bytes); extra entries open another block.

| Offset | Type | Field |
|---|---|---|
| 0 | u16 | `itemId` |
| 2 | i8 | x (relative to the multi's origin) |
| 3 | i8 | y |
| 4 | i8 | z |

Block index for these blocks: `9 + n` (`n` = 0, 1, ...), without the `0x20` bit.

The client interprets `b0` as follows: high nibble = mode (`2` = grid, `0` = list of 5-byte entries, `1` = list of 4-byte `id,x,y` entries with z derived from the plane); low nibble = plane. Plane 0 uses z = 0; planes 1 to 4 and 5 to 8 use `z = ((plane - 1) mod 4) * 20 + 7`. For mode 2 it rebuilds `x = i / columnHeight + offX` and `y = i % columnHeight + offY`, with `columnHeight` derived from the dimensions of the local multi (plane 0: `maxY - minY + 2`; 1 to 4: `maxY - minY`; 5 to 8: `maxY - minY + 1`) and offX/offY = `minX/minY` (+1 in planes 1 to 4). This requires the server to use the same multi dimensions as the client (plus the extra southern row).

### 4.3 Compression

Zlib with the standard header (`78 9C` at the default level). Each plane and each offset block is compressed separately. If compression fails, ServUO writes the block with size 0 (keeps the block in the packet); Sphere omits the plane and logs an error.

Plane blocks are in ascending order (0..8) and the offset blocks come after them. Planes with no components in the grid are not emitted.

### 4.4 Which ids are "multi" and which are "stair"

- **Multi**: the `0x0D` command carries the index of a stair multi (`stairs.txt`, columns `MultiNorth`, `MultiEast`, `MultiSouth`, `MultiWest`). The server expands the multi into items from the `MultiComponentList` with the multi's offsets plus `(x, y, z of the current floor)`, ignoring entries with `itemId == 1`. Multis `7668..7675` (TOL) do not exist in `MultiData` and are expanded by internal tables.
- **Stair (item)**: ids of stair blocks and steps in fixed lists (`m_BlockIDs`, `m_StairSeqs`, `m_StairIDs`). Sequence N, W, S, E; `IsStair(id, ref dir)` returns the direction (0 = N, 1 = W, 2 = S, 3 = E). They are only used to remove the entire staircase (4 steps) when the client deletes one step.
- Stair items sit at `z` values other than 7/27/47/67 (offsets in steps of 5) and therefore travel in the offset block, not in the grid.
- **Fixture**: doors and teleporters (id ranges in `DesignState.IsFixture`). During design they are normal components of `0xD8`; on commit the server removes them from the multi and creates them as real items.

### 4.5 Revision

- Each foundation keeps `LastRevision` (persisted). Any modification of the design state does `Revision = ++LastRevision` and invalidates the cached `0xD8` packet of the state.
- The client only checks the revision in `0xBF/0x1D`. Same revision = reuse the cache; different = `0xBF/0x1E`.
- For the player who is editing, the server announces the revision of the design state; for everyone else, that of the visible state.
- Sphere keeps four drawings (`main`, `working`, `backup`, `revert`), each with `m_iRevision`; the commit only happens if `working.revision != main.revision`; backup/restore/revert are ignored if the revisions are equal (or if the slot's revision is 0).

## 5. Message sequences

### 5.1 Entering design mode

1. C->S `0x06` (double-click on the sign) or `0xB1` (the "Customize This House" button of the house gump; server-specific).
2. Server validates (alive, not in combat, owner, no rented vendors, etc.), relocates the house's items and mobiles outside (`BanLocation`) and creates the design context (level 1; the player is hidden and moved to `foundation + (0, 0, 7)`).
3. S->C `0x1D` (Delete Object) for fixtures, signpost, sign hanger and the house sign (ServUO).
4. S->C `0xBF/0x20` type `0x04`.
5. S->C foundation multi object + `0xBF/0x1D` with the design revision.
6. S->C `0xD8` with the design state (`response = 0x01`).
7. Client opens the designer gump. If the revision differs from the cache, it sends `0xBF/0x1E` and receives a new `0xD8`.

Sphere sends `0xBF/0x20`, then the multi, then `0xD8`, and only then moves/hides the player and removes the house's dynamic items from the client's view.

### 5.2 Editing (repeated)

1. C->S `0xD7` (`0x06` build, `0x05` delete, `0x0D` stairs, `0x13`/`0x14` roof, `0x12` level, `0x02` backup, etc.).
2. The server applies it to the design state and increments the revision. No return packet if the command was valid (the client already shows the result locally).
3. If the command is invalid, the server resends the full S->C `0xD8` so the client undoes its local prediction.

### 5.3 Exit

| Path | Sequence |
|---|---|
| Commit | C->S `0xD7/0x04` -> S->C confirmation gump (price, balance) -> C->S `0xB1` button 1 -> S->C `0xBF/0x20` type `0x05`, updated multi, `0xBF/0x1D`, `0xD8` with `response = 0` |
| Close | C->S `0xD7/0x0C` -> S->C `0xBF/0x20` type `0x05`, multi, `0xD8` (visible state, `response = 0`) |
| Disconnect | context removed on the disconnect event; nothing is sent |

In both cases the server re-displays fixtures/sign, ejects all of the house's items and mobiles to `BanLocation` and restores the relocated entities.

## 6. Server rules

1. **Mandatory context.** Every `0xD7` handler (except verify) returns without doing anything if the player is not in design context.
2. **States.** The foundation keeps three states: visible (`CurrentState`), design (`DesignState`) and backup (`BackupState`). Edits only go to the design. Commit copies the design to the visible state.
3. **Build (`0x06`).** If the player is not a GM and the item is not in the table of valid components (`Data/Components/*.txt` filtered by the expansion flag) or is a roof piece, it resends the state and returns. `z` is calculated by the server from the context's floor; for cells in the extra southern row (`y + center.Y == height - 1`) it uses `z = 0`. The client does not send `z`.
4. **Delete (`0x05`).** If `z == 0` and `(x, y)` is inside the foundation grid (`ay < height - 1`), the component is not removable: it resends the state. If the item is part of a staircase, it removes the 4 steps of the staircase and resends the state. If the interior cell (`ax >= 1`, `ay >= 1`, `ay < height - 1`) is left without a floor at `z = 7`, the server inserts the dirt tile `0x31F4`.
5. **Stairs (`0x0D`).** Validates the index in `stairs.txt` (multi column); if invalid, resends the state. Sends nothing back on success.
6. **Roof (`0x13`).** Requires SE+ or GM. `z` must be a multiple of 3 between `-3` and `12`; outside that range it becomes `-3`; the final `z` adds the floor's `z`. Before inserting, it removes roof pieces already present in the same cell and floor. Validates `TileFlag.Roof` and the `roof.txt` table.
7. **Roof delete (`0x14`).** The item must have `TileFlag.Roof`; otherwise it resends the state.
8. **Level (`0x12`).** A floor outside `1..MaxLevels` becomes 1 (ServUO) or is capped at the maximum (Sphere). The server moves the player to `foundation.Z + zOfFloor` and resends the multi + revision.
9. **Clear (`0x10`).** New state = empty foundation; revision incremented; resends multi + `0xBF/0x1D` + `0xD8`.
10. **Restore (`0x03`) / Revert (`0x1A`).** Replaces the design with a copy of the backup (restore) or of the visible state (revert, with "frozen" fixtures put back as components); increments the revision; resends multi + `0xBF/0x1D` + `0xD8`.
11. **Backup (`0x02`).** Copies the design to the backup slot; no response.
12. **Sync (`0x0E`).** Resends the design's `0xD8`.
13. **Commit.** Cost = `CustomizationCost + (design components - (visible components + fixtures)) * 500` (the first term is 0 under AOS). If positive, it is debited from the player's bank; if negative, it is deposited. GMs do not pay. Insufficient funds: message `1061903` and the commit is refused, leaving the player in design mode.
14. **Restrictions during design.** Speech is blocked (message `1061925`), movement actions are limited to the foundation area with `Z` locked to the current floor, other commands receive `1062206`, the player is hidden and cannot reveal themselves.
15. **Entities inside the house.** They are moved outside when entering and when leaving design; mannequins/stewards are re-deeded.
16. **Disconnect.** The server removes the context and ejects everyone from the house. The design state is not discarded (it remains in `DesignState` and is shown again on the next entry).
17. **Cache.** The serialized `0xD8` can be reused while the state's revision does not change.
18. **Time/frequency.** There is no rate limit in the handlers; ServUO only compresses on a separate thread and uses the cached packet.

## 7. Example

House with `xMin = -3`, `yMin = -3`, `xMax = 3`, `yMax = 4` (`width = 7`, `height = 8`), serial `0x40000123`, revision 5, and three components:

- floor `0x31F4` at `(-2, -2, 7)`: floor (height <= 0) -> plane 1, `x = 1, y = 1`, then `x = 0, y = 0`, `size = 6`, `index = 0`; plane with `6 * 6 * 2 = 72` bytes.
- wall `0x0006` at `(-3, -3, 7)` (assuming height > 0): plane `1 + 4 = 5`, `x = 0, y = 0`, `size = 7`, `index = 0`; plane with `7 * 7 * 2 = 98` bytes.
- item `0x0751` at `(0, 0, 12)`: `z = 12` is not a floor plane -> offset block `07 51 00 00 0C`.

Packet (`zlib` level 6):

```
D8 00 45                  id, total size = 69
03 01                     compression 0x03, response flag 0x01
40 00 01 23               serial
00 00 00 05               revision 5
00 03                     3 components
00 34                     buffer length = 52 (= 69 - 17)
03                        3 blocks
21 48 0D 00  78 9C 33 FC C2 40 15 00 00 51 BC 01 26   plane 1 (0x20|1), 72 -> 13 bytes
25 62 0D 00  78 9C 63 60 63 A0 31 00 00 02 A8 00 07   plane 5 (0x20|5), 98 -> 13 bytes
09 05 0D 00  78 9C 63 0F 64 60 E0 01 00 01 78 00 65   offset block (9), 5 -> 13 bytes
```

Block header `21 48 0D 00`: index `0x21`, uncompressed `0x048 = 72`, compressed `0x00D = 13`, last byte `00` (high bits of both = 0).

Entering design mode (revision 5, same serial):

```
BF 00 11 00 20 40 00 01 23 04 00 00 FF FF FF FF FF   begin
BF 00 0D 00 1D 40 00 01 23 00 00 00 05               revision
D8 ...                                               design state
```

Client builds a wall `0x0006` at `(1, -2)` and then switches to floor 2:

```
D7 00 19 00 00 00 01 00 06 00 00 00 00 06 00 00 00 00 01 00 FF FF FF FE 0A
D7 00 0F 00 00 00 01 00 12 00 00 00 00 02 0A
```

Client closes without commit and the server ends it:

```
D7 00 0A 00 00 00 01 00 0C 0A
BF 00 11 00 20 40 00 01 23 05 00 00 FF FF FF FF FF
```

## 8. Pitfalls

- The grid is column-major `(x * size + y)`, not `(y * width + x)`.
- The per-plane sizes are not equal: plane 0 is `width * height`, floors `(width-1) * (height-2)` and walls `width * (height-1)`. A wrong size misaligns everything without any zlib error.
- Include the extra southern row in `height`; without it the dimensions diverge from those the client calculates.
- Offset blocks use index `9 + n` without `0x20`; planes use `0x20 | plane`.
- The two 12-bit sizes share the 4th byte; writing `(size >> 8)` and `(clen >> 8)` into swapped nibbles is a common mistake.
- A negative `y` in `0xD7` arrives as a two's complement i32; read it as a signed integer.
- The level command carries the floor as `u32 0, u8 floor`; read as tag + i32 it yields the floor, but read as a plain u32 it yields 0.
- The response to an invalid edit must be the full `0xD8`; resending only `0xBF/0x1D` does not make the client undo the edit.
- Do not send `0xD8` before the foundation multi exists on the client.
- Forgetting to resend fixtures, sign and signpost when leaving design leaves the house without a door/plaque.
- Sphere and ServUO differ in how the floor is calculated (see 9); do not assume the client sends `z` in `0x06`.

## 9. ServUO x Sphere differences

| Aspect | ServUO / ModernUO | Sphere (Source-X) |
|---|---|---|
| States | visible, design, backup; revert = copy of the visible state | `main`, `working`, `backup`, `revert` |
| Floor | design context stores `Level`; `0x12` updates it | derived from the player's Z (`GetPlane`) |
| `0xD8` planes | planes 0..8 (floors 1-4, walls 5-8) + blocks 9+ | generates planes 0..4 only with high items aligned to the plane; floors, items outside the grid and items outside the level go to blocks 9+ |
| Grid size | `width*height*2` etc., fixed per plane | `(maxIndex + 1) * 2` per plane |
| Commit | confirmation gump with price and bank | applies directly; script triggers (`HouseDesignCommitItem`) |
| Validation failure | resends the `0xD8` | resends the `0xD8` (`SendStructureTo`) |
| Stairs success | no resend | resends the `0xD8` |
| Client gating | none at the network level; `Core.SE` for roof | version >= 4.0.0, KR or Enhanced |
| Restore/Revert when identical | always applies | ignored if the revisions are equal |

## 10. Unspecified

- Effect of the "response flag" byte of `0xD8` and of the subcommands `0xD7/0x0A` and `0xD7/0x0F`: the client reads or sends them and the server ignores them.
- Semantics of types `1`, `2` and `3` of `0xBF/0x20`.
- Exact value of `MaxLevels` in Sphere (`GetLevelCount` function).
- Mode `1` (list of 4-byte `id,x,y` entries) of blocks: it exists in the client, and no server examined emits it.

## 11. Implementation reference

| Part | File / symbol |
|---|---|
| ServUO: handlers, states, context, packets | `Scripts/Multis/HouseFoundation.cs`: `HouseFoundation.Initialize`, `BeginCustomize`, `Designer_Build/Delete/Stairs/Roof/RoofDelete/Level/Clear/Restore/Revert/Backup/Close/Commit/Sync`, `EndConfirmCommit`, `QueryDesignDetails`, `DesignState`, `DesignContext`, `BeginHouseCustomization`, `EndHouseCustomization`, `DesignStateGeneral`, `DesignStateDetailed`, `ConfirmCommitGump` |
| ServUO: validation | `Scripts/Multis/ComponentVerification.cs`, `Data/Components/*.txt`, `Server/ExpansionInfo.cs` (`HousingFlags`) |
| ServUO: player restrictions | `Scripts/Mobiles/PlayerMobile.cs` (`CheckMovement`, `OnLocationChange`, `EventSink_Disconnected`, `RevealingAction`) |
| ServUO: encoded envelope | `Server/Network/PacketHandlers.cs` (`EncodedCommand`, `ExtendedCommand`), `Server/Network/EncodedReader.cs` |
| ServUO: entry | `Scripts/Gumps/HouseGumpAOS.cs` (button "Customize This House" -> `BeginCustomize`) |
| ModernUO | `Projects/UOContent/Multis/Houses/HouseFoundation.cs` (same `Designer_*`, `DesignState`); `Projects/UOContent/Multis/Houses/HousePackets.cs` (`SendBeginHouseCustomization`, `SendEndHouseCustomization`, `SendDesignStateGeneral`, `CreateHouseDesignStateDetailed`); `Projects/UOContent/Network/Packets/IncomingHousePackets.cs` (`0xFB`) |
| Sphere | `src/game/items/CItemMultiCustom.cpp` (`BeginCustomize`, `EndCustomize`, `SwitchToLevel`, `CommitChanges`, `AddItem`, `AddStairs`, `AddRoof`, `RemoveItem`, `SendStructureTo`, `BackupStructure`, `RestoreStructure`, `RevertChanges`, `ResetStructure`, `IsValidItem`); `src/network/send.cpp` (`PacketHouseDesignVersion`, `PacketHouseBeginCustomise`, `PacketHouseEndCustomise`, `PacketHouseDesign`); `src/network/receive.cpp` (`PacketHouseDesignReq`, `PacketHouseDesignBackup`, `...Restore`, `...Commit`, `...DestroyItem`, `...PlaceItem`, `...Exit`, `...PlaceStair`, `...Sync`, `...Clear`, `...Switch`, `...PlaceRoof`, `...DestroyRoof`, `...Revert`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CustomHouse` for `0xD8`, `ReadUnsafeCustomHouseData`, `0xBF` cases `0x1D` and `0x20`); `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CustomHouse*`); `src/ClassicUO.Client/Game/Managers/HouseCustomizationManager.cs`; `src/ClassicUO.Client/Game/UI/Gumps/HouseCustomizationGump.cs`; `src/ClassicUO.Client/Game/Data/CustomHouse.cs` |
