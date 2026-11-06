# Boats and Smooth Sailing

Multi-packet boat movement flow. The boat is a multi (`BaseMulti`) with items and mobiles "on board". A boat step has to move the multi, the tiller man, the planks, the hold, all items and all mobiles at the same time. Smooth Sailing (High Seas clients) does this with a single `0xF6` packet; earlier clients receive an individual update per entity.

Conventions: big-endian integers; offsets counted from byte 0 (id). `S->C` = server to client, `C->S` = client to server. Ids and general sizes are in [packets.md](packets.md).

## 1. Overview and who receives what

| Client | Boat movement | Mouse piloting |
|---|---|---|
| Before 7.0.9.0 | Individual update of each entity (items and mobiles) on every step | No |
| 7.0.9.0 or later (High Seas, flag `HighSeas`) | Single `0xF6` per step | `0xBF/0x33` (C->S) |
| Enhanced / KR | Treated as High Seas (Sphere) | Same |

Detection:

- ServUO/ModernUO: `NetState.HighSeas`, enabled with client version `>= 7.0.9.0` (`ProtocolChanges.Version7090`).
- Sphere: `MINCLIVER_HS = 7000900` (7.0.9.0) or Enhanced client, and the option `OF_NoSmoothSailing` turned off.
- ModernUO uses the single-step path (`Teleport`) for everyone when the server runs with an expansion below HS (`NewBoatMovement => Core.HS`).
- ServUO keeps `NewBoatMovement = true` fixed; when the displacement of one step exceeds 1 tile on some axis (for example, a map wrap), it uses `Teleport`.

The ClassicUO client always processes `0xF6`; smooth animation only happens if the option `UseSmoothBoatMovement` is on (default off). When off, the client only repositions the multi and each entity in the list to their final coordinates.

## 2. Sequences

### 2.1 Movement step (server, High Seas)

1. The boat timer fires (`MoveTimer`) and `Move` computes the movement direction `d` and the displacement `(xOffset, yOffset)`; each tile of the step goes through `CanFit`.
2. If the step is at most 1 tile per axis, the server builds the `0xF6` once, with the final positions (current position + displacement) of the boat and of all entities on board (except the boat itself).
3. S->C `0xF6` to each High Seas client that `CanSee(boat)` and is within the boat's update range (`InRange(Location, GetUpdateRange(m))`).
4. Only after that does the server change the real positions of the entities and the boat. During this change, a per-entity flag (`NoMoveHS`) suppresses individual movement packets for High Seas clients.
5. Non-High Seas clients receive the normal individual updates (item: world packet `0x1A`/`0xF3`; mobile: `0x77`/`0x78`/`0xD3`; the player on board: `0x20` and a reset of the movement sequence).
6. ServUO also calls `SendContainerPacket`: for High Seas clients outside the boat's update range but within the global radar radius, it sends `0x1D` for the boat and its items, and an `0xF7` with all entities (each one as an `0xF3`).

### 2.2 Legacy step (client < 7.0.9.0 or server without HS)

1. Same computation as `Move`.
2. `Teleport(xOffset, yOffset, 0)`: each entity on board (except components) and the boat have their `Location` changed.
3. The `Location` change generates the usual per-entity packets for each client in range; for the player on board, `0x20` (or the old format) and `Sequence = 0`.
4. In ModernUO without HS the intervals are longer and move 3 tiles per step at fast speed (see 5).

### 2.3 Helmsman (HS) and mouse piloting

1. C->S `0x06` (double-click on the wheel: `TillerMan`, `ShipWheel`).
2. The server validates: player on board, not flying, not mounted (unless the mount is the boat's), boat not scuttled (`Scuttled`). If there is already a pilot and he is the boat's owner, another player does not take control; if the pilot is another player, whoever uses the wheel takes control; if whoever uses it is already the pilot, he stops piloting.
3. S->C virtual item equipped on the player: graphic `0x3E96` on the mount layer (`Layer.Mount`), linked to the boat (`BoatMountItem`); the player's `Direction` becomes equal to the boat's `Facing`; cliloc message `1116727`.
4. The client detects graphic `0x3E96` on the mount layer and considers itself "piloting": mouse clicks stop generating `0x02` (walk) and generate `0xBF/0x33`.
5. C->S `0xBF/0x33` repeated while the mouse button is held; releasing the button = `0xBF/0x33` with speed 0.
6. End: another double-click on the wheel, disconnection or death (`ForceRemovePilot`); the server removes the virtual item and sends cliloc message `1149592`.

### 2.4 Boat placement (deed or docked boat)

1. C->S `0x06` (double-click on the deed).
2. S->C direction selection gump (ServUO, `BoatPlacementGump`): buttons 1 = west, 2 = north, 3 = south, 4 = east.
3. C->S `0xB1` with the chosen button.
4. S->C `0x99` Multi Placement (target with the boat's multi in that direction).
5. C->S `0x6C` Target Response with the chosen point.
6. The server validates the region (dungeon, house, Champion spawn) and `CanFit`; creates the boat and sends the normal objects (`0xF3`/`0x1A`, mobiles).

## 3. Packets

### 3.1 `0xF6` Boat Moving (S->C, `var`)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xF6` | |
| 1 | u16 | total size | `18 + 10 * count` |
| 3 | u32 | boat serial | |
| 7 | u8 | speed | see 5.1 |
| 8 | u8 | movement direction | direction enum (0 = N, 1 = NE, 2 = E, 3 = SE, 4 = S, 5 = SW, 6 = W, 7 = NW) |
| 9 | u8 | direction the boat points to (facing) | only 0, 2, 4, 6 |
| 10 | i16 | boat x | final position |
| 12 | i16 | boat y | final position |
| 14 | i16 | boat z | |
| 16 | u16 | `count` | entities below (the boat is not included) |
| 18 | entry x `count` | | 10 bytes each |

Entry:

| Offset | Type | Field |
|---|---|---|
| 0 | u32 | serial (item or mobile) |
| 4 | i16 | final x |
| 6 | i16 | final y |
| 8 | i16 | final z |

The list includes all mobiles and items on board, including the boat's own parts (tiller man, planks, hold). The values are final positions, not deltas.

Client behavior (ClassicUO): ignores the packet if the boat serial is not a known item; entries whose serial is not known are skipped; the entry for the player's own serial updates the center of the view range. In smooth mode it stacks the step (discards the oldest when there are more than 5 pending per boat) and animates the entities as an offset relative to the boat; in normal mode it applies `SetInWorldTile(x, y, z)` to the multi and to each entity and regenerates the house/multi.

### 3.2 `0xF7` Packet container (S->C, `var`)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0xF7` |
| 1 | u16 | total size |
| 3 | u16 | `count` |
| 5 | ... | `count` complete `0xF3` packets (id included), 26 bytes each |

Each inner `0xF3` uses the normal Object Info layout (SA/new): `u8 0xF3`, `u16 0x0001`, `u8 type` (`0x00` item, `0x01` mobile, `0x02` multi, `0x03` damageable item), `u32 serial`, `u16 graphic`, `u8 0`, `u16 quantity`, `u16 quantity`, `u16 x & 0x7FFF`, `u16 y & 0x3FFF`, `i8 z`, `u8 light`, `u16 hue`, `u8 flags`, `u16 0`. The client only accepts `0xF3` inside `0xF7`; any other id stops processing. `0xF7` is not listed in [packets.md](packets.md) (the table covers up to `0xF6`).

### 3.3 `0xBF/0x33` Multi mouse movement (C->S, 12 bytes)

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xBF` | |
| 1 | u16 | `0x000C` | |
| 3 | u16 | `0x0033` | subcommand |
| 5 | u32 | player serial | |
| 9 | u8 | direction | desired absolute direction |
| 10 | u8 | direction (repeated) | |
| 11 | u8 | speed | `0` = stop, `1` = slow (walk), `2` = fast (run) |

ClassicUO sends the direction twice and uses speed 2 when the mouse distance from the center is >= 190 pixels; when the button is released it sends speed 0 with the character's current direction. ServUO ignores the second direction byte; Sphere reads both (the first as the bow direction, the second as the movement direction) and limits speeds above 2 to 2.

The server only accepts the command if the player is the current pilot (ServUO: is mounted on `BaseBoat`; Sphere: is in the boat's region and is the `m_Pilot`).

### 3.4 `0x99` Multi Placement (S->C)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0x99` |
| 1 | u8 | `allowGround` |
| 2 | u32 | cursor id (`TargetID`) |
| 6 | u8 | flags |
| 7 | u8[11] | zeros |
| 18 | u16 | multi index |
| 20 | i16 | x offset |
| 22 | i16 | y offset |
| 24 | i16 | z offset |
| 26 | u8[4] | zeros (High Seas clients only) |

Size: 30 bytes for High Seas clients, 26 bytes for earlier ones (ServUO `MultiTargetReqHS` and `MultiTargetReq`). The response is the 19-byte `0x6C`.

### 3.5 `0x6C` Target Response (C->S, 19 bytes)

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | `0x6C` |
| 1 | u8 | type (`0` object, `1` position) |
| 2 | u32 | cursor id |
| 6 | u8 | flags |
| 7 | u32 | serial |
| 11 | i16 | x |
| 13 | i16 | y |
| 15 | i16 | z |
| 17 | u16 | graphic |

Cancellation: `x = y = -1` and an invalid serial.

### 3.6 `0xAD` Unicode Speech Request: voice commands (C->S)

The boat commands (`forward`, `stop`, `turn left`, etc.) arrive as client keywords (`speech.mul` table), not as text interpreted by the server.

| Offset | Type | Field | Notes |
|---|---|---|---|
| 0 | u8 | `0xAD` | |
| 1 | u16 | size | |
| 3 | u8 | type | bit `0xC0` (`Encoded`) indicates keywords |
| 4 | u16 | hue | |
| 6 | u16 | font | |
| 8 | char[4] | language | |
| 12 | u16 | `(count << 4) \| (first 4 bits of the first id)` | only if `Encoded` |
| 14 | ... | packed 12-bit ids | alternating u8 and u16, 1.5 bytes per id |
| end | UTF-8 (Encoded) or null-terminated UTF-16BE | text | |

The server accepts at most 50 keywords, and text from 1 to 128 characters.

### 3.7 Auxiliary packets

| Packet | Use |
|---|---|
| `0x1D` Delete Object (5 bytes) | removes the boat and items from the client before `0xF7` (ServUO) |
| `0x2E` Equip (15 bytes) | item equipped on a layer; the pilot's virtual item `0x3E96` reaches the client through the common equip path (`0x2E` for the owner, `0x78`/`0xD3` for the others) |
| `0xB0`/`0xDD` | placement gump and dry dock gump |
| `0x9A`/`0xC2` | text prompt to rename the boat |
| `0xBF/0x19` type 5 (19 bytes) | "BoatDriverLock" defined in ServUO, not used in the flow |

## 4. Boat state

- **Facing** (bow): N, E, S or W. Each orientation has its own multi id (`NorthID`, `EastID`, `SouthID`, `WestID`); turning swaps the boat's `ItemID`.
- **Movement direction**: without a pilot, `(facing + relativeDirection) & 7`; with a pilot (mouse), absolute direction.
- **Relative directions**: `Forward` = N (0), `ForwardRight` = NE (1), `Right` = E (2), `BackwardRight` = SE (3), `Backward` = S (4), `BackwardLeft` = SW (5), `Left` = W (6), `ForwardLeft` = NW (7).
- **Drift**: movement in a direction other than `Forward`, `ForwardLeft`, `ForwardRight`: the boat slides without turning. Uses the same step time as normal movement in ModernUO HS; in ServUO it uses the drift interval (see 5).
- **Turning**: multiples of 90 degrees (`+2`, `-2`, `-4` in direction units). The command starts a 0.5 s timer; if the boat was moving and there is no pilot, it resumes movement in the same direction after turning. With a pilot, the `Turn` direction is absolute. With the mouse, the server only turns if the speed is 2 and the requested direction differs from facing, facing+1 and facing-1; for diagonal directions it uses the previous cardinal direction.
- **Stopping at an obstacle**: if the first tile fails `CanFit`, the step is cancelled (cliloc `501424`); if only a later tile fails, the step is shortened.
- **Rotation**: `SetFacing` relocates the tiller man, hold, planks and everyone on board by rotating coordinates; this path does **not** use `0xF6`: each entity is resent through the common update path, and the boat swaps its `ItemID`.
- **Map wrap**: if the boat leaves the map rectangle, the coordinates wrap and the step gets a displacement greater than 1: it uses `Teleport`, without `0xF6`.

## 5. Speed, interval and displacement

### 5.1 Speed byte of `0xF6`

| Value | Meaning | Client interval (ClassicUO) |
|---|---|---|
| `0x01` | one step (`OneMove` command) | 500 ms (default value) |
| `0x02` | slow / rowboat / slow drift | 1000 ms |
| `0x03` | normal | 500 ms |
| `0x04` | fast | 250 ms |
| > `0x04` | | `value * 10` ms |

These values do not match the speed byte of the `0xBF/0x33` request (0/1/2).

### 5.2 ServUO (High Seas default)

Intervals: `FastInterval = 250 ms`, `NormalInterval = 500 ms`, `SlowInterval = 1000 ms`. All steps move 1 tile.

| Situation | Interval | `0xF6` byte |
|---|---|---|
| Fast, forward | 250 ms | `0x04` |
| Fast, drift | 500 ms | `0x03` |
| Slow, drift | 1000 ms | `0x02` |
| Slow, forward | 500 ms | `0x03` |
| `OneMove` | 250 ms (forward) or 500 ms (drift) | `0x01` |
| Mouse, speed 1 | 1000 ms | `0x02` |
| Mouse, speed 2 | 250 ms | `0x04` |
| Rowboat, damaged boat (`DamageTaken >= Moderately`) or in emergency repair | 1000 ms | `0x02` |

Mouse command with speed 0 (or any interval 0): `StopMove`.

### 5.3 ModernUO

| Mode | Slow | Fast | Drift slow | Drift fast | Tiles per step (fast) |
|---|---|---|---|---|---|
| HS | 0.50 s | 0.25 s | 0.50 s | 0.25 s | 1 |
| Non-HS | 0.75 s | 0.75 s | 1.50 s | 0.75 s | 3 |

`0xF6` byte: `0x04` for fast, `0x03` for slow, `0x01` for `OneMove`.

### 5.4 Sphere

`_eSpeedMode` (1 to 4) is the byte sent in `0xF6`: `0x01` one tile, `0x02` rowboat, `0x03` slow, `0x04` fast. The delay to the next step is `period * 100 ms` in slow mode and half of that in the others; `period` and `tiles` come from the boat definition (`ShipSpeed`).

## 6. Server rules

1. Send `0xF6` before changing the real positions; otherwise the HS client also receives the individual packets and sees the boat move twice.
2. Only send `0xF6` to clients that already know the boat and its entities. Entities that enter the field of view with the step (including when the client is on board) still need the normal world packet.
3. A step with a displacement greater than 1 on some axis goes through the `Teleport` path, without `0xF6`.
4. With `0xF6` it is mandatory to suppress individual movement packets for HS clients while the positions change (ServUO/ModernUO: `NoMoveHS`). Earlier clients keep receiving the individual ones.
5. For clients before 7.0.9.0, those on board must receive `0x20` (or the old format) and have the movement sequence reset on every step.
6. `0xBF/0x33` requests from someone who is not the pilot must be ignored.
7. Sphere ignores a mouse request while the boat timer is armed (`fWheelMove && IsTimerSet()`), which limits the step rate to the boat's interval.
8. When stopping (speed 0), cancel the movement timer; when receiving a new direction, replace the timer.
9. Removing the boat, or the pilot's death or disconnection, removes the virtual mount item.
10. Under High Seas a boat only exists once per player (message `1116758`), except rowboats.

## 7. Client commands and interfaces

### 7.1 Voice commands (keyword ids handled in `BaseBoat.OnSpeech`)

Accepted only from someone on board, who has command over the boat and when there is no pilot.

| Id | Action |
|---|---|
| `0x42` | set name |
| `0x43` | remove name |
| `0x44` | say the name |
| `0x45`, `0x68` | go forward (fast) |
| `0x46` | backward |
| `0x47` | left |
| `0x48` | right |
| `0x4B` | forward-left |
| `0x4C` | forward-right |
| `0x4D` | backward-left |
| `0x4E` | backward-right |
| `0x4F`, `0x69` | stop |
| `0x50` to `0x57` | same movements as `0x47`, `0x48`, `0x45`, `0x46`, `0x4B`, `0x4C`, `0x4E`, `0x4D` at slow speed |
| `0x58` to `0x5F` | single step (`OneMove`) |
| `0x49`, `0x65` | turn right |
| `0x4A`, `0x66` | turn left |
| `0x67` | turn around |
| `0x6A` | drop anchor (non-HS only) |
| `0x6B` | raise anchor (non-HS only) |
| `0x60` | next navigation point |
| `0x61` | start the course |
| `0x62` | continue the course |
| `0x63` | go to the point indicated in the text (`goto`) |
| `0x64` | single step to the point (`single`) |
| `0x0F` | track vessels (`start` / `stop` in the text; HS only) |

The order of `0x50` to `0x57` in ServUO is: left, right, forward, backward, forward-left, forward-right, backward-right, backward-left.

Sphere exposes the same behavior as script verbs (`SHIPFORE`, `SHIPBACK`, `SHIPTURN`, `SHIPSTOP`, `SHIPANCHORDROP`, `SHIPANCHORRAISE`, `SHIPFACE`, `SHIPMOVE`, `SHIPGATE` and others), linked to speech definitions.

### 7.2 Gumps, targets and prompts

| Interface | Packets | Note |
|---|---|---|
| Placement direction | gump (`0xB0`/`0xDD`) + `0xB1` | ServUO `BoatPlacementGump`, buttons 1 to 4 (west, north, south, east); button 0 cancels |
| Placement position | `0x99` + `0x6C` | multi target, `InternalTarget` |
| Dry dock | gump `ConfirmDryDockGump` + `0xB1` | button 2 = continue, 1 = cancel; the option appears in the helmsman's context menu |
| Rename | text prompt (`0x9A`/`0xC2`) | `RenameBoatPrompt` -> `EndRename` |
| Plank | double-click (`0x06`) | `Plank.OnDoubleClick` opens/closes; blocked while piloting |

## 8. Example

Boat serial `0x40000010`, at `(1500, 1600, -5)`, facing north, moving north at fast speed; on board, player `0x00000007` and tiller man `0x40000011`. The step shifts `y` by -1:

```
F6 00 26                  size = 18 + 2*10 = 38
40 00 00 10               boat serial
04                        fast speed
00                        movement direction: N
00                        facing: N
05 DC                     x = 1500
06 3F                     y = 1599
FF FB                     z = -5
00 02                     2 entities
00 00 00 07  05 DC 06 3E  FF FE      player at (1500, 1598, -2)
40 00 00 11  05 DC 06 3D  FF FB      tiller man at (1500, 1597, -5)
```

Pilot request (mouse, run, direction set by the client):

```
BF 00 0C 00 33  00 00 00 07  00 00 02
              serial   dir dir spd
```

Stop on button release:

```
BF 00 0C 00 33  00 00 00 07  00 00 00
```

## 9. Pitfalls

- The `0xF6` list does not include the boat; `count` and the total size must not count it. ModernUO and ServUO, on the other hand, include the tiller man, planks and hold in the list.
- Coordinates are final, not deltas. Sending the old ones makes the client walk backward.
- The speed byte of `0xF6` and that of the `0xBF/0x33` request use different scales.
- Sending `0xF6` without the client knowing an entity makes it silently ignore it: the entity stays still.
- Mixing `0xF6` and individual movement packets for the same client produces a double step.
- Steps larger than 1 tile per axis (or with a wrap) must not use `0xF6`; use removal/recreation (`Teleport`).
- Turning does not go through `0xF6`: the multi's `ItemID` changes and all entities are repositioned.
- Without the virtual item `0x3E96` on the mount layer, the client keeps sending `0x02` and not `0xBF/0x33`.
- The client assumes the center of the range follows the player's entry in `0xF6`; omitting the player from the list leaves the view stuck.
- The speed in `0xF6` determines the duration of the animation on the client; if it does not match the server's real interval there are jerks or delay.
- `0xF7` is a container of `0xF3`, not of arbitrary packets.

## 10. ServUO x Sphere

| Aspect | ServUO / ModernUO | Sphere |
|---|---|---|
| Who receives `0xF6` | HS (7.0.9.0+) in update range | HS (7.0.9.0+) or Enhanced, unless `OF_NoSmoothSailing` |
| Listed entities | everything on board except the boat | all objects from `ListObjs` except the first (the boat) |
| Boat position in the packet | `boat + offset` | already updated position |
| Order | sends before moving | moves before, then sends to each client |
| Non-HS client | `Teleport` (individual item/mobile) | `addItem`/`addCharMove`/`addChar` per entity |
| Mouse request | `BoatMovementRequest` registered on `0xBF/0x33`; uses the direction from the first byte | `PacketWheelBoatMove`; reads bow and movement; ignores while the timer is armed |
| Pilot | item `0x3E96` on `Layer.Mount` | memory `ITEMID_SHIP_PILOT` (`0x3E96`) on `LAYER_HORSE` |
| Speed | `clientSpeed` 1 to 4 depending on the fast/slow/drift combination | `_eSpeedMode` 1 to 4 |
| Commands | speech keywords in `OnSpeech` | `SHIP*` script verbs |
| `0xF7` | sent to HS clients out of range | defined (`PacketContainer`), not used in the observed movement route |

## 11. Not specified

- Direction table used by ClassicUO for the direction byte of `0xBF/0x33` (the server treats the byte as a direction enum).
- Meaning of subcommand `0xBF/0x19` type 5 with the value emitted by `BoatDriverLock`: the client treats it as a mobile animation; ServUO does not use it.
- Packet `0x2E` `BoatEquipPacket` (defined in ServUO, never sent).
- Maximum number of entities on board in Sphere (`MAX_MULTI_LIST_OBJS`).

## 12. Implementation reference

| Part | File / symbol |
|---|---|
| ServUO: boat logic | `Scripts/Multis/Boats/BaseBoat.cs`: `Move`, `DoMovement`, `StartMove`, `StopMove`, `OneMove`, `StartTurn`, `Turn`, `SetFacing`, `Teleport`, `OnSpeech`, `GetMovementInterval`, `OnMousePilotCommand`, `LockPilot`, `RemovePilot`, `ForceRemovePilot`, `SendContainerPacket`, `MoveBoatHS`, `PacketContainer`, `GetEntitiesOnBoard` |
| ServUO: mouse request | `Scripts/Services/Expansions/High Seas/Network/PacketHandlers.cs` (`BoatMovementRequest.MultiMouseMovementRequest`) |
| ServUO: HS packets | `Scripts/Services/Expansions/High Seas/Network/Packets.cs` (`BoatDriverLock`, `BoatEquipPacket`) |
| ServUO: pilot and interfaces | `Scripts/Services/Expansions/High Seas/Multis/BoatMountItem.cs`, `ShipWheel.cs`, `Scripts/Multis/Boats/TillerMan.cs`, `Plank.cs`, `ConfirmDryDockGump.cs`, `RenameBoatPrompt.cs`, `BaseBoatDeed.cs`, `Scripts/Services/Expansions/High Seas/Gumps/ShipPlacementGump.cs` |
| ServUO: flags and suspension | `Server/Network/NetState.cs` (`ProtocolChanges.HighSeas`, `Version7090`), `Server/Mobile.cs` (`SetLocation`, `NoMoveHS`), `Server/Item.cs` (`NoMoveHS`), `Server/Targeting/MultiTarget.cs`, `Server/Network/Packets.cs` (`MultiTargetReq`, `MultiTargetReqHS`), `Server/Network/PacketHandlers.cs` (`UnicodeSpeech`, `TargetResponse`) |
| ModernUO | `Projects/UOContent/Multis/Boats/BoatPackets.cs` (`CreateMoveBoatHS`, `SendMoveBoatHS`, `SendMoveBoatHSUsingCache`, `SendDisplayBoatHS`), `Projects/UOContent/Multis/Boats/BaseBoat.cs` (`Move`, `StartMove`, `OneMove`, `OnSpeech`, `GetMovingEntities`, `Teleport`, `SetFacing`), `Projects/Server/Network/Packets/PacketContainerBuilder.cs`, `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateWorldEntity`), `Projects/Server/Network/NetState/ProtocolChanges.cs` |
| Sphere | `src/network/send.cpp` (`PacketMoveShip`, `PacketContainer`), `src/network/receive.cpp` (`PacketWheelBoatMove`), `src/game/components/CCMultiMovable.cpp` (`MoveDelta`, `SetMoveDir`, `SetNextMove`, `SetPilot`, `ListObjs`, `r_Verb`), `src/game/components/CCMultiMovable.h`, `src/game/uo_files/uofiles_enums.h` (`ShipMovementType`, `ShipMovementSpeed`), `src/common/sphereproto.h` (`MINCLIVER_HS`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BoatMoving`, `PacketList`), `src/ClassicUO.Client/Game/Managers/BoatMovingManager.cs`, `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MultiBoatMoveRequest`), `src/ClassicUO.Client/Game/GameObjects/Mobile.cs` (`IsDrivingBoat`), `src/ClassicUO.Client/Game/Scenes/GameSceneInputHandler.cs` |
