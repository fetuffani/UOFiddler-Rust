# Protocol packets (client 7.0.x)

Packet tables for the Ultima Online 7.0.x client. Size column: number of bytes in the whole packet (including the id byte), or `var` for variable size. `—` indicates a size not fixed by the client table.

## Server to client

| ID | Size | Name | Format |
|---|---|---|---|
| 0x0B | 7 | Damage | [format](packets/s2c-0x0b.md) |
| 0x11 | var | Mobile Status (HP/stats) | [format](packets/s2c-0x11.md) |
| 0x15 | 9 | Follow | [format](packets/s2c-0x15.md) |
| 0x16 | var | Healthbar update (SA/EC) | [format](packets/s2c-0x16.md) |
| 0x17 | var | Health Bar Status Update | [format](packets/s2c-0x17.md) |
| 0x1A | var | Object Info (item on the ground) | [format](packets/s2c-0x1a.md) |
| 0x1B | 37 | Login Confirm (character entered the world) | [format](packets/s2c-0x1b.md) |
| 0x1C | var | Speech ASCII | [format](packets/s2c-0x1c.md) |
| 0x1D | 5 | Delete Object | [format](packets/s2c-0x1d.md) |
| 0x20 | 19 | Draw Game Player (own position/direction) | [format](packets/s2c-0x20.md) |
| 0x21 | 8 | Move Reject | [format](packets/s2c-0x21.md) |
| 0x22 | 3 | Move Acknowledge | [format](packets/s2c-0x22.md) |
| 0x23 | 26 | Dragging Item (animation) | [format](packets/s2c-0x23.md) |
| 0x24 | 9 | Draw/Open Container | [format](packets/s2c-0x24.md) |
| 0x25 | 21 | Add Item To Container | [format](packets/s2c-0x25.md) |
| 0x27 | 2 | Reject Move Item | [format](packets/s2c-0x27.md) |
| 0x28 | 5 | Drop Item Failed / Clear Square | [format](packets/s2c-0x28.md) |
| 0x29 | 1 | Drop Item Approved | [format](packets/s2c-0x29.md) |
| 0x2C | 2 | Resurrection Menu / Death | [format](packets/s2c-0x2c.md) |
| 0x2D | 17 | Mobile Attributes | [format](packets/s2c-0x2d.md) |
| 0x2E | 15 | Equip/Worn Item | [format](packets/s2c-0x2e.md) |
| 0x2F | 10 | Fight Occurring | [format](packets/s2c-0x2f.md) |
| 0x30 | 5 | Attack OK | [format](packets/s2c-0x30.md) |
| 0x31 | var | Attack Ended | [format](packets/s2c-0x31.md) |
| 0x33 | 2 | Pause/Resume Client | [format](packets/s2c-0x33.md) |
| 0x38 | 7 | Pathfinding | [format](packets/s2c-0x38.md) |
| 0x39 | 9 | Remove (Group) | [format](packets/s2c-0x39.md) |
| 0x3A | var | Skills List/Update | [format](packets/s2c-0x3a.md) |
| 0x3B | var | End Vendor Buy | [format](packets/s2c-0x3b.md) |
| 0x3C | var | Add Multiple Items To Container (bag contents) | [format](packets/s2c-0x3c.md) |
| 0x3E | 37 | Versions (God client) | [format](packets/s2c-0x3e.md) |
| 0x3F | var | Update Statics (God client; UltimaLive command) | [format](packets/s2c-0x3f.md) |
| 0x40 | 201 | Update Terrain (God client; UltimaLive) | [format](packets/s2c-0x40.md) |
| 0x41 | var | Update Tiledata (God client) | [format](packets/s2c-0x41.md) |
| 0x42 | var | Update Art (God client) | [format](packets/s2c-0x42.md) |
| 0x43 | 553 | Update Animation (God client) | [format](packets/s2c-0x43.md) |
| 0x44 | 713 | Update Hues (God client) | [format](packets/s2c-0x44.md) |
| 0x45 | 5 | Version OK | [format](packets/s2c-0x45.md) |
| 0x4E | 6 | Personal Light Level | [format](packets/s2c-0x4e.md) |
| 0x4F | 2 | Overall Light Level | [format](packets/s2c-0x4f.md) |
| 0x53 | 2 | Reject Character Logon / Popup message | [format](packets/s2c-0x53.md) |
| 0x54 | 12 | Play Sound Effect | [format](packets/s2c-0x54.md) |
| 0x55 | 1 | Login Complete | [format](packets/s2c-0x55.md) |
| 0x56 | 11 | Map Packet (cartography/treasure) | [format](packets/s2c-0x56.md) |
| 0x57 | 110 | Update Regions | [format](packets/s2c-0x57.md) |
| 0x5B | 4 | Time | [format](packets/s2c-0x5b.md) |
| 0x65 | 4 | Set Weather | [format](packets/s2c-0x65.md) |
| 0x66 | var | Book Pages | [format](packets/s2c-0x66.md) |
| 0x69 | var | Change Text/Emote Colors (Options) | [format](packets/s2c-0x69.md) |
| 0x6A | 3 | Not documented (3 bytes) | [format](packets/s2c-0x6a.md) |
| 0x6C | 19 | Target Cursor | [format](packets/s2c-0x6c.md) |
| 0x6D | 3 | Play Music (MIDI) | [format](packets/s2c-0x6d.md) |
| 0x6E | 14 | Character Animation | [format](packets/s2c-0x6e.md) |
| 0x6F | var | Secure Trading | [format](packets/s2c-0x6f.md) |
| 0x70 | 28 | Graphical Effect | [format](packets/s2c-0x70.md) |
| 0x71 | var | Bulletin Board | [format](packets/s2c-0x71.md) |
| 0x72 | 5 | War Mode | [format](packets/s2c-0x72.md) |
| 0x73 | 2 | Ping | [format](packets/s2c-0x73.md) |
| 0x74 | var | Open Buy Window (shop list) | [format](packets/s2c-0x74.md) |
| 0x76 | 16 | New Subserver | [format](packets/s2c-0x76.md) |
| 0x77 | 17 | Update Mobile (another mobile's move/animation) | [format](packets/s2c-0x77.md) |
| 0x78 | var | Mobile Incoming (mobile appears + equipment) | [format](packets/s2c-0x78.md) |
| 0x7B | 2 | Sequence | [format](packets/s2c-0x7b.md) |
| 0x7C | var | Open Dialog Box (menu) | [format](packets/s2c-0x7c.md) |
| 0x81 | var | Change Character | [format](packets/s2c-0x81.md) |
| 0x82 | 2 | Login Denied | [format](packets/s2c-0x82.md) |
| 0x85 | 2 | Character Delete Result | [format](packets/s2c-0x85.md) |
| 0x86 | var | Character List Update | [format](packets/s2c-0x86.md) |
| 0x88 | 66 | Open Paperdoll | [format](packets/s2c-0x88.md) |
| 0x89 | var | Corpse Clothing | [format](packets/s2c-0x89.md) |
| 0x8B | var | Display Sign Gump | [format](packets/s2c-0x8b.md) |
| 0x8C | 11 | Connect To Game Server | [format](packets/s2c-0x8c.md) |
| 0x90 | 19 | Map Message | [format](packets/s2c-0x90.md) |
| 0x92 | var | Not documented | [format](packets/s2c-0x92.md) |
| 0x93 | 99 | Book Header (old) | [format](packets/s2c-0x93.md) |
| 0x94 | var | Not documented | [format](packets/s2c-0x94.md) |
| 0x95 | 9 | Dye Window | [format](packets/s2c-0x95.md) |
| 0x97 | 2 | Move Player | [format](packets/s2c-0x97.md) |
| 0x98 | var | Mobile Name | [format](packets/s2c-0x98.md) |
| 0x99 | 30 | Multi Placement (26 bytes in clients before 7.0.9.0) | [format](packets/s2c-0x99.md) |
| 0x9A | var | Console Entry Prompt | [format](packets/s2c-0x9a.md) |
| 0x9E | var | Sell List | [format](packets/s2c-0x9e.md) |
| 0xA1 | 9 | Update Current HP | [format](packets/s2c-0xa1.md) |
| 0xA2 | 9 | Update Current Mana | [format](packets/s2c-0xa2.md) |
| 0xA3 | 9 | Update Current Stamina | [format](packets/s2c-0xa3.md) |
| 0xA5 | var | Open Web Browser | [format](packets/s2c-0xa5.md) |
| 0xA6 | var | Tip/Notice | [format](packets/s2c-0xa6.md) |
| 0xA8 | var | Game Server List | [format](packets/s2c-0xa8.md) |
| 0xA9 | var | Characters + Starting Locations | [format](packets/s2c-0xa9.md) |
| 0xAA | 5 | Attack Target (current) | [format](packets/s2c-0xaa.md) |
| 0xAB | var | Gump Text Entry Dialog | [format](packets/s2c-0xab.md) |
| 0xAE | var | Unicode Speech | [format](packets/s2c-0xae.md) |
| 0xAF | 13 | Display Death Action | [format](packets/s2c-0xaf.md) |
| 0xB0 | var | Gump Menu Dialog (gump) | [format](packets/s2c-0xb0.md) |
| 0xB2 | var | Chat Message | [format](packets/s2c-0xb2.md) |
| 0xB3 | var | Chat Text | [format](packets/s2c-0xb3.md) |
| 0xB4 | var | Target Object List | [format](packets/s2c-0xb4.md) |
| 0xB5 | 64 | Open Chat Window | [format](packets/s2c-0xb5.md) |
| 0xB7 | var | Help/Tip Data | [format](packets/s2c-0xb7.md) |
| 0xB8 | var | Character Profile | [format](packets/s2c-0xb8.md) |
| 0xB9 | 5 | Enable Locked Client Features | [format](packets/s2c-0xb9.md) |
| 0xBA | 10 | Quest Arrow | [format](packets/s2c-0xba.md) |
| 0xBB | 9 | Ultima Messenger | [format](packets/s2c-0xbb.md) |
| 0xBC | 3 | Seasonal Info | [format](packets/s2c-0xbc.md) |
| 0xBD | var | Client Version request | [format](packets/s2c-0xbd.md) |
| 0xBE | var | Assist Version | [format](packets/s2c-0xbe.md) |
| 0xBF | var | General Information (subcommands) | [format](packets/bf-framing.md) |
| 0xC0 | 36 | Hued Effect | [format](packets/s2c-0xc0.md) |
| 0xC1 | var | Localized Message (cliloc) | [format](packets/s2c-0xc1.md) |
| 0xC2 | var | Unicode Prompt | [format](packets/s2c-0xc2.md) |
| 0xC3 | var | GQ Request | [format](packets/s2c-0xc3.md) |
| 0xC4 | 6 | Semivisible | [format](packets/s2c-0xc4.md) |
| 0xC6 | 1 | Invalid Map Enable | [format](packets/s2c-0xc6.md) |
| 0xC7 | 49 | 3D Particle Effect | [format](packets/s2c-0xc7.md) |
| 0xC8 | 2 | Update Range | [format](packets/s2c-0xc8.md) |
| 0xC9 | 6 | Trip Time | [format](packets/s2c-0xc9.md) |
| 0xCA | 6 | UTrip Time | [format](packets/s2c-0xca.md) |
| 0xCB | 7 | Global Queue Count | [format](packets/s2c-0xcb.md) |
| 0xCC | var | Localized Message Affix | [format](packets/s2c-0xcc.md) |
| 0xCD | 1 | Not documented (1 byte) | [format](packets/s2c-0xcd.md) |
| 0xCE | var | Not documented | [format](packets/s2c-0xce.md) |
| 0xD0 | var | Configuration File | [format](packets/s2c-0xd0.md) |
| 0xD1 | 2 | Logout Status | [format](packets/s2c-0xd1.md) |
| 0xD2 | 25 | Extended 0x20 (Mobile Update) | [format](packets/s2c-0xd2.md) |
| 0xD3 | var | Extended 0x78 (Mobile Incoming) | [format](packets/s2c-0xd3.md) |
| 0xD4 | var | Book Header (new) | [format](packets/s2c-0xd4.md) |
| 0xD6 | var | Mega Cliloc (properties/tooltips) | [format](packets/s2c-0xd6.md) |
| 0xD8 | var | Custom House | [format](packets/s2c-0xd8.md) |
| 0xDA | var | Mahjong (game packets) | [format](packets/s2c-0xda.md) |
| 0xDB | var | Character Transfer Log | [format](packets/s2c-0xdb.md) |
| 0xDC | 9 | SE Introduced Revision (props hash) | [format](packets/s2c-0xdc.md) |
| 0xDD | var | Compressed Gump | [format](packets/s2c-0xdd.md) |
| 0xDE | var | Update Mobile Status | [format](packets/s2c-0xde.md) |
| 0xDF | var | Buff/Debuff | [format](packets/s2c-0xdf.md) |
| 0xE2 | 10 | New Character Animation | [format](packets/s2c-0xe2.md) |
| 0xE5 | var | Display Waypoint | [format](packets/s2c-0xe5.md) |
| 0xE6 | 5 | Remove Waypoint | [format](packets/s2c-0xe6.md) |
| 0xE7 | 12 | Highlight UI Continue | [format](packets/s2c-0xe7.md) |
| 0xE9 | 75 | Highlight UI Toggle | [format](packets/s2c-0xe9.md) |
| 0xEA | 3 | Toggle Hotbar | [format](packets/s2c-0xea.md) |
| 0xF2 | 25 | Time Sync Response | [format](packets/s2c-0xf2.md) |
| 0xF3 | 26 | Object Info (SA/new) | [format](packets/s2c-0xf3.md) |
| 0xF5 | 21 | New Map Message | [format](packets/s2c-0xf5.md) |
| 0xF6 | var | Boat Moving | [format](packets/s2c-0xf6.md) |
| 0xF7 | var | Packet Container (groups 0xF3 entries of the same movement) | [format](packets/s2c-0xf7.md) |

## Client to server

| ID | Size | Name | Format |
|---|---|---|---|
| 0x00 | 104 | Create Character | [format](packets/c2s-0x00.md) |
| 0x01 | 5 | Disconnect / Logout | [format](packets/c2s-0x01.md) |
| 0x02 | 7 | Move Request | [format](packets/c2s-0x02.md) |
| 0x03 | var | Talk (ASCII) | [format](packets/c2s-0x03.md) |
| 0x04 | 2 | GodModeRequest | [format](packets/c2s-0x04.md) |
| 0x05 | 5 | Attack Request | [format](packets/c2s-0x05.md) |
| 0x06 | 5 | Double Click (use) | [format](packets/c2s-0x06.md) |
| 0x07 | 7 | Pick Up Item | [format](packets/c2s-0x07.md) |
| 0x08 | 15 | Drop Item | [format](packets/c2s-0x08.md) |
| 0x09 | 5 | Single Click (look) | [format](packets/c2s-0x09.md) |
| 0x0A | 11 | Edit | [format](packets/c2s-0x0a.md) |
| 0x12 | var | Use Skill / Cast / Action | [format](packets/c2s-0x12.md) |
| 0x13 | 10 | Equip Item Request | [format](packets/c2s-0x13.md) |
| 0x14 | 6 | ChangeZ | [format](packets/c2s-0x14.md) |
| 0x22 | 3 | Resynchronize | [format](packets/c2s-0x22.md) |
| 0x2C | 2 | DeathStatusResponse | [format](packets/c2s-0x2c.md) |
| 0x34 | 10 | Get Player Status | [format](packets/c2s-0x34.md) |
| 0x3A | var | Set Skill Lock | [format](packets/c2s-0x3a.md) |
| 0x3B | var | VendorBuyReply | [format](packets/c2s-0x3b.md) |
| 0x47 | 11 | NewTerrain | [format](packets/c2s-0x47.md) |
| 0x48 | 73 | NewAnimData | [format](packets/c2s-0x48.md) |
| 0x4B | 9 | Check Client Version | [format](packets/c2s-0x4b.md) |
| 0x52 | var | Board Post Message | [format](packets/c2s-0x52.md) |
| 0x58 | 106 | NewRegion | [format](packets/c2s-0x58.md) |
| 0x5D | 73 | Play Character (select) | [format](packets/c2s-0x5d.md) |
| 0x61 | 9 | DeleteStatic | [format](packets/c2s-0x61.md) |
| 0x6B | 9 | Not documented | [format](packets/c2s-0x6b.md) |
| 0x6C | 19 | Target Response | [format](packets/c2s-0x6c.md) |
| 0x6F | var | SecureTrade | [format](packets/c2s-0x6f.md) |
| 0x72 | 5 | Request War Mode | [format](packets/c2s-0x72.md) |
| 0x73 | 2 | Ping | [format](packets/c2s-0x73.md) |
| 0x75 | 35 | RenameRequest | [format](packets/c2s-0x75.md) |
| 0x79 | 9 | ResourceQuery | [format](packets/c2s-0x79.md) |
| 0x7D | 13 | MenuResponse | [format](packets/c2s-0x7d.md) |
| 0x7E | 2 | GodviewQuery | [format](packets/c2s-0x7e.md) |
| 0x80 | 62 | Login Request | [format](packets/c2s-0x80.md) |
| 0x83 | 39 | DeleteCharacter | [format](packets/c2s-0x83.md) |
| 0x84 | 69 | Change Password | [format](packets/c2s-0x84.md) |
| 0x8D | var | ECCreateCharacter | [format](packets/c2s-0x8d.md) |
| 0x91 | 65 | Game Server Login | [format](packets/c2s-0x91.md) |
| 0x95 | 9 | HuePickerResponse | [format](packets/c2s-0x95.md) |
| 0x96 | var | GameCentralMoniter | [format](packets/c2s-0x96.md) |
| 0x98 | var | MobileNameRequest | [format](packets/c2s-0x98.md) |
| 0x9A | var | AsciiPromptResponse | [format](packets/c2s-0x9a.md) |
| 0x9B | 258 | Help Request | [format](packets/c2s-0x9b.md) |
| 0x9D | 51 | GMSingle | [format](packets/c2s-0x9d.md) |
| 0x9F | var | Sell Reply | [format](packets/c2s-0x9f.md) |
| 0xA0 | 3 | Select Server | [format](packets/c2s-0xa0.md) |
| 0xA4 | 149 | System Info | [format](packets/c2s-0xa4.md) |
| 0xA7 | 4 | Request Tip | [format](packets/c2s-0xa7.md) |
| 0xAC | var | Gump Text Entry Dialog Reply | [format](packets/c2s-0xac.md) |
| 0xAD | var | Unicode Speech Request | [format](packets/c2s-0xad.md) |
| 0xB1 | var | Gump Menu Response | [format](packets/c2s-0xb1.md) |
| 0xB6 | 9 | Object Help Request | [format](packets/c2s-0xb6.md) |
| 0xB8 | var | ProfileReq | [format](packets/c2s-0xb8.md) |
| 0xBB | 9 | AccountID | [format](packets/c2s-0xbb.md) |
| 0xBD | var | Client Version | [format](packets/c2s-0xbd.md) |
| 0xBE | var | AssistVersion | [format](packets/c2s-0xbe.md) |
| 0xBF | var | General Information | [format](packets/bf-framing.md) |
| 0xC2 | var | UnicodePromptResponse | [format](packets/c2s-0xc2.md) |
| 0xC8 | 2 | SetUpdateRange | [format](packets/c2s-0xc8.md) |
| 0xC9 | 6 | TripTime | [format](packets/c2s-0xc9.md) |
| 0xCA | 6 | UTripTime | [format](packets/c2s-0xca.md) |
| 0xCF | 78 | AccountLogin | [format](packets/c2s-0xcf.md) |
| 0xD0 | var | ConfigurationFile | [format](packets/c2s-0xd0.md) |
| 0xD1 | 2 | LogoutReq | [format](packets/c2s-0xd1.md) |
| 0xD6 | var | BatchQueryProperties | [format](packets/c2s-0xd6.md) |
| 0xD7 | var | Encoded Command (AOS) | [format](packets/d7-framing.md) |
| 0xD9 | 268 | Spy on Client (hardware survey) | [format](packets/c2s-0xd9.md) |
| 0xE1 | var | ClientType | [format](packets/c2s-0xe1.md) |
| 0xEC | var | EquipMacro | [format](packets/c2s-0xec.md) |
| 0xED | var | UnequipMacro | [format](packets/c2s-0xed.md) |
| 0xEF | — | Login Seed | [format](packets/c2s-0xef.md) |
| 0xF0 | var | Movement Request (batch) | [format](packets/c2s-0xf0.md) |
| 0xF1 | 9 | Time Sync Request | [format](packets/c2s-0xf1.md) |
| 0xF4 | var | CrashReport | [format](packets/c2s-0xf4.md) |
| 0xF8 | — | CreateCharacter70160 | [format](packets/c2s-0xf8.md) |
| 0xFA | — | Unhandled | [format](packets/c2s-0xfa.md) |
| 0xFB | 2 | PublicHouseContent (id + flag) | [format](packets/c2s-0xfb.md) |

## Notes

- Packets of size `var` start with the id (1 byte) followed by a 2-byte (u16) size field holding the total packet length. Example: `0xB4` has the layout `[id][u16 len]...`.
- The client's size table covers ids `0x00` to `0xF6`.
- `0xBF` (General Information) groups subcommands.
- `0x73` (Ping) and `0x72` (War Mode) exist in both directions with the same id.
- `0xEF` (Login Seed) is handled outside the size table.
- The ids `0x6A`, `0x92`, `0x94`, `0xCD`, `0xCE` have no documented layout. Client 7.0.10.3 discards `0x6A` and `0xCD`.

### 0xB4 Target Object List

Server to client, size `var`:

| Field | Type |
|---|---|
| id | u8 (`0xB4`) |
| length | u16 |
| allowGround | u8 |
| serial | u32 |
| x, y, z | u16, u16, u16 |
| count | u16 |
| entries (`count`) | `itemId`, `hue`, `xOff`, `yOff`, `zOff`: 5 x u16 |

Items are shown as "ghosts" that follow the target cursor; the click is answered with `0x6C` (Target Response).

### 0x84 Change Password

Client to server, 69 bytes: id, `char[30]` current password, `char[30]` new password, u32, local IPv4 address.

### 0xF1 / 0xF2 Time Sync

`0xF1` (client to server, 9 bytes): id + 8 bytes. The `0xF2` response (server to client, 25 bytes) contains id + 3 x 8 bytes.
