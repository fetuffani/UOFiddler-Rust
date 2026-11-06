# Packet format

One file per packet, with size, byte-by-byte fields, variations by client version and implementation reference. Conventions: offsets counted from the first byte of the packet (the id); integers of more than one byte are big-endian; `u8/u16/u32` are unsigned and `i8/i16/i32` are signed.

Packets whose layout could not be confirmed in the reference implementations are marked as "Not documented" or "Not specified" in the file itself.

## Server -> client packets

| Packet | Name | Size |
|---|---|---|
| [0x0B](s2c-0x0b.md) | Damage | 7 bytes |
| [0x11](s2c-0x11.md) | Mobile Status (HP/stats) | variable (id + u16 length + payload) |
| [0x15](s2c-0x15.md) | Follow | 9 bytes |
| [0x16](s2c-0x16.md) | Healthbar update (SA/EC) | variable (id + u16 length + payload) |
| [0x17](s2c-0x17.md) | Health Bar Status Update | variable (id + u16 length + payload) |
| [0x1A](s2c-0x1a.md) | Object Info (item on the ground) | variable (id + u16 length + payload) |
| [0x1B](s2c-0x1b.md) | Login Confirm (character entered) | 37 bytes |
| [0x1C](s2c-0x1c.md) | Speech ASCII | variable (id + u16 length + payload) |
| [0x1D](s2c-0x1d.md) | Delete Object | 5 bytes |
| [0x20](s2c-0x20.md) | Draw Game Player (own position/direction) | 19 bytes |
| [0x21](s2c-0x21.md) | Move Reject | 8 bytes |
| [0x22](s2c-0x22.md) | Move Acknowledge | 3 bytes |
| [0x23](s2c-0x23.md) | Dragging Item (animation) | 26 bytes |
| [0x24](s2c-0x24.md) | Draw/Open Container | 7 or 9 bytes (9 in High Seas clients, 7 |
| [0x25](s2c-0x25.md) | Add Item To Container | 21 bytes (20 bytes before client 6 |
| [0x27](s2c-0x27.md) | Reject Move Item | 2 bytes |
| [0x28](s2c-0x28.md) | Drop Item Failed / Clear Square | 5 bytes |
| [0x29](s2c-0x29.md) | Drop Item Approved | 1 byte |
| [0x2C](s2c-0x2c.md) | Resurrection Menu / Death | 2 bytes |
| [0x2D](s2c-0x2d.md) | Mobile Attributes | 17 bytes |
| [0x2E](s2c-0x2e.md) | Equip/Worn Item | 15 bytes |
| [0x2F](s2c-0x2f.md) | Fight Occurring | 10 bytes |
| [0x30](s2c-0x30.md) | Attack OK | 5 bytes |
| [0x31](s2c-0x31.md) | Attack Ended / Pet Window | variable (id + u16 length + payload) |
| [0x33](s2c-0x33.md) | Pause/Resume Client | 2 bytes |
| [0x38](s2c-0x38.md) | Pathfinding | 7 bytes |
| [0x39](s2c-0x39.md) | Remove (Group) | 9 bytes |
| [0x3A](s2c-0x3a.md) | Skills List/Update | variable (id + u16 length + payload) |
| [0x3B](s2c-0x3b.md) | End Vendor Buy | variable (id + u16 length + payload) |
| [0x3C](s2c-0x3c.md) | Add Multiple Items To Container (bag contents) | variable (id + u16 length + payload) |
| [0x3E](s2c-0x3e.md) | Versions (God client) | 37 bytes |
| [0x3F](s2c-0x3f.md) | Update Statics (God client; UltimaLive command) | variable (id + u16 length + payload) |
| [0x40](s2c-0x40.md) | Update Terrain (God client; UltimaLive) | 201 bytes |
| [0x41](s2c-0x41.md) | Update Tiledata (God client) | variable (id + u16 length + payload) |
| [0x42](s2c-0x42.md) | Update Art (God client) | variable (id + u16 length + payload) |
| [0x43](s2c-0x43.md) | Update Animation (God client) | 553 bytes |
| [0x44](s2c-0x44.md) | Update Hues (God client) | 713 bytes |
| [0x45](s2c-0x45.md) | Version OK | 5 bytes |
| [0x4E](s2c-0x4e.md) | Personal Light Level | 6 bytes |
| [0x4F](s2c-0x4f.md) | Overall Light Level | 2 bytes |
| [0x53](s2c-0x53.md) | Reject Character Logon / Popup message | 2 bytes |
| [0x54](s2c-0x54.md) | Play Sound Effect | 12 bytes |
| [0x55](s2c-0x55.md) | Login Complete | 1 byte |
| [0x56](s2c-0x56.md) | Map Packet (cartography/treasure) | 11 bytes |
| [0x57](s2c-0x57.md) | Update Regions | 110 bytes |
| [0x5B](s2c-0x5b.md) | Time | 4 bytes |
| [0x65](s2c-0x65.md) | Set Weather | 4 bytes |
| [0x66](s2c-0x66.md) | Book Pages | variable (id + u16 length + payload) |
| [0x69](s2c-0x69.md) | Change Text/Emote Colors (Options) | variable (id + u16 length + payload) |
| [0x6A](s2c-0x6a.md) | Not documented | 3 bytes |
| [0x6C](s2c-0x6c.md) | Target Cursor | 19 bytes |
| [0x6D](s2c-0x6d.md) | Play Music (MIDI) | 3 bytes |
| [0x6E](s2c-0x6e.md) | Character Animation | 14 bytes |
| [0x6F](s2c-0x6f.md) | Secure Trading | variable (id + u16 length + payload) |
| [0x70](s2c-0x70.md) | Graphical Effect | 28 bytes |
| [0x71](s2c-0x71.md) | Bulletin Board | variable (id + u16 length + payload) |
| [0x72](s2c-0x72.md) | War Mode | 5 bytes |
| [0x73](s2c-0x73.md) | Ping | 2 bytes |
| [0x74](s2c-0x74.md) | Open Buy Window (shop list) | variable (id + u16 length + payload) |
| [0x76](s2c-0x76.md) | New Subserver | 16 bytes |
| [0x77](s2c-0x77.md) | Update Mobile (move/animation of another mobile) | 17 bytes |
| [0x78](s2c-0x78.md) | Mobile Incoming (mobile appears + equip) | variable (id + u16 length + payload) |
| [0x7B](s2c-0x7b.md) | Sequence | 2 bytes |
| [0x7C](s2c-0x7c.md) | Open Dialog Box (menu) | variable (id + u16 length + payload) |
| [0x81](s2c-0x81.md) | Change Character | variable (id + u16 length + payload; length is the total size |
| [0x82](s2c-0x82.md) | Login Denied | 2 bytes |
| [0x85](s2c-0x85.md) | Character Delete Result | 2 bytes |
| [0x86](s2c-0x86.md) | Character List Update | variable (id + u16 length + payload; length is the total size |
| [0x88](s2c-0x88.md) | Open Paperdoll | 66 bytes |
| [0x89](s2c-0x89.md) | Corpse Clothing | variable (id + u16 length + payload; length is the total size |
| [0x8B](s2c-0x8b.md) | Display Sign Gump | variable (id + u16 length + payload; length is the total size |
| [0x8C](s2c-0x8c.md) | Connect To Game Server | 11 bytes |
| [0x90](s2c-0x90.md) | Map Message | 19 bytes |
| [0x92](s2c-0x92.md) | Not documented | variable (id + u16 length + payload; length is the total size |
| [0x93](s2c-0x93.md) | Book Header (old) | 99 bytes |
| [0x94](s2c-0x94.md) | Not documented | variable (id + u16 length + payload; length is the total size |
| [0x95](s2c-0x95.md) | Dye Window | 9 bytes |
| [0x97](s2c-0x97.md) | Move Player | 2 bytes |
| [0x98](s2c-0x98.md) | Mobile Name | variable (id + u16 length + payload; length is the total size |
| [0x99](s2c-0x99.md) | Multi Placement | 30 bytes (HS clients) | 26 bytes (previous format) |
| [0x9A](s2c-0x9a.md) | Console Entry Prompt | variable (id + u16 length + payload; length is the total size |
| [0x9E](s2c-0x9e.md) | Sell List | variable (id + u16 length + payload; length is the total size |
| [0xA1](s2c-0xa1.md) | Update Current HP | 9 bytes |
| [0xA2](s2c-0xa2.md) | Update Current Mana | 9 bytes |
| [0xA3](s2c-0xa3.md) | Update Current Stamina | 9 bytes |
| [0xA5](s2c-0xa5.md) | Open Web Browser | variable (id + u16 length + payload; length is the total size |
| [0xA6](s2c-0xa6.md) | Tip/Notice | variable (id + u16 length + payload; length is the total size |
| [0xA8](s2c-0xa8.md) | Game Server List | variable (id + u16 length + payload; length is the total size |
| [0xA9](s2c-0xa9.md) | Characters + Starting Locations | variable (id + u16 length + payload; length is the total size |
| [0xAA](s2c-0xaa.md) | Attack Target (current) | 5 bytes |
| [0xAB](s2c-0xab.md) | Gump Text Entry Dialog | variable (id + u16 length + payload; length is the total size |
| [0xAE](s2c-0xae.md) | Unicode Speech | variable (id + u16 length + payload; length is the total size |
| [0xAF](s2c-0xaf.md) | Display Death Action | 13 bytes |
| [0xB0](s2c-0xb0.md) | Gump Menu Dialog (gump) | variable (id + u16 length + payload; length is the total size |
| [0xB2](s2c-0xb2.md) | Chat Message | variable (id + u16 length + payload; length is the total size |
| [0xB3](s2c-0xb3.md) | Chat Text | variable (id + u16 length + payload; length is the total size |
| [0xB4](s2c-0xb4.md) | Target Object List | variable (id + u16 length + payload; length is the total size |
| [0xB5](s2c-0xb5.md) | Open Chat Window | 64 bytes |
| [0xB7](s2c-0xb7.md) | Help/Tip Data | variable (id + u16 length + payload; length is the total size |
| [0xB8](s2c-0xb8.md) | Character Profile | variable (id + u16 length + payload; length is the total size |
| [0xB9](s2c-0xb9.md) | Enable Locked Client Features | 5 bytes (clients 6 |
| [0xBA](s2c-0xba.md) | Quest Arrow | 10 bytes (HS) | 6 bytes (previous) |
| [0xBB](s2c-0xbb.md) | Message/Account ID | fixed 9 bytes total |
| [0xBC](s2c-0xbc.md) | Seasonal Info | fixed 3 bytes total |
| [0xBD](s2c-0xbd.md) | Client Version request | variable (id + u16 length + payload; length is the total size |
| [0xBE](s2c-0xbe.md) | Assist Version | variable (id + u16 length + payload; length is the total size |
| [0xC0](s2c-0xc0.md) | Hued Effect | 36 bytes |
| [0xC1](s2c-0xc1.md) | Localized Message (cliloc) | variable (id + u16 length + payload; length is the total size |
| [0xC2](s2c-0xc2.md) | Unicode Prompt | variable (id + u16 length + payload; length is the total size |
| [0xC3](s2c-0xc3.md) | GQ Request | variable (id + u16 length + payload; length is the total size |
| [0xC4](s2c-0xc4.md) | Semivisible | 6 bytes |
| [0xC6](s2c-0xc6.md) | Invalid Map Enable | 1 byte |
| [0xC7](s2c-0xc7.md) | 3D Particle Effect | 49 bytes |
| [0xC8](s2c-0xc8.md) | Update Range | 2 bytes |
| [0xC9](s2c-0xc9.md) | Trip Time | 6 bytes |
| [0xCA](s2c-0xca.md) | UTrip Time | 6 bytes |
| [0xCB](s2c-0xcb.md) | Global Queue Count | 7 bytes |
| [0xCC](s2c-0xcc.md) | Localized Message Affix | variable (id + u16 length + payload; length is the total size |
| [0xCD](s2c-0xcd.md) | Not documented (1 byte) | 1 byte |
| [0xCE](s2c-0xce.md) | Not documented | variable (id + u16 length + payload; length is the total size |
| [0xD0](s2c-0xd0.md) | Configuration File | variable (id + u16 length + payload; length is the total size |
| [0xD1](s2c-0xd1.md) | Logout Status | 2 bytes |
| [0xD2](s2c-0xd2.md) | Extended 0x20 (Mobile Update) | 25 bytes |
| [0xD3](s2c-0xd3.md) | Extended 0x78 (Mobile Incoming) | variable (id + u16 length + payload; length is the total size |
| [0xD4](s2c-0xd4.md) | Book Header (new) | variable (id + u16 length + payload; length is the total size |
| [0xD6](s2c-0xd6.md) | Mega Cliloc (object property list) | variable |
| [0xD8](s2c-0xd8.md) | Custom House | variable (id + u16 length + payload; length is the total size |
| [0xDA](s2c-0xda.md) | Mahjong (game packets) | variable (id + u16 length + payload; length is the total size |
| [0xDB](s2c-0xdb.md) | Character Transfer Log | variable (id + u16 length + payload; length is the total size |
| [0xDC](s2c-0xdc.md) | SE Introduced Revision | fixed 9 bytes total |
| [0xDD](s2c-0xdd.md) | Compressed Gump | variable |
| [0xDE](s2c-0xde.md) | Update Mobile Status | variable (id + u16 length + payload; length is the total size |
| [0xDF](s2c-0xdf.md) | Buff/Debuff | variable (id + u16 length + payload; length is the total size |
| [0xE2](s2c-0xe2.md) | New Character Animation | 10 bytes |
| [0xE5](s2c-0xe5.md) | Display Waypoint | variable (id + u16 length + payload; length is the total size |
| [0xE6](s2c-0xe6.md) | Remove Waypoint | 5 bytes |
| [0xE7](s2c-0xe7.md) | Highlight UI Continue | 12 bytes |
| [0xE9](s2c-0xe9.md) | Highlight UI Toggle | 75 bytes |
| [0xEA](s2c-0xea.md) | Toggle Hotbar | 3 bytes |
| [0xF0](s2c-0xf0.md) | Batch movement and special Krrios packet | variable |
| [0xF2](s2c-0xf2.md) | Time Sync Response | 25 bytes |
| [0xF3](s2c-0xf3.md) | Object Info (SA/new) | 26 bytes (HS clients) | 24 bytes (SA, without the final field) |
| [0xF5](s2c-0xf5.md) | New Map Message | 21 bytes |
| [0xF6](s2c-0xf6.md) | Boat Moving | variable (id + u16 length + payload; length is the total size |
| [0xF7](s2c-0xf7.md) | Packet List (multi-packet) | variable (id + u16 length + payload; length is the total size |

## Client -> server packets

| Packet | Name | Size |
|---|---|---|
| [0x00](c2s-0x00.md) | Create Character | 104 bytes (fixed) |
| [0x01](c2s-0x01.md) | Disconnect / Logout | 5 bytes (fixed) |
| [0x02](c2s-0x02.md) | Move Request | 7 bytes (fixed) |
| [0x03](c2s-0x03.md) | Talk (ASCII) | variable (id + u16 length + payload) |
| [0x04](c2s-0x04.md) | GodModeRequest | 2 bytes (fixed) |
| [0x05](c2s-0x05.md) | Attack Request | 5 bytes (fixed) |
| [0x06](c2s-0x06.md) | Double Click | 5 bytes (fixed) |
| [0x07](c2s-0x07.md) | Pick Up Item | 7 bytes (fixed) |
| [0x08](c2s-0x08.md) | Drop Item | 14 bytes (clients before 6 |
| [0x09](c2s-0x09.md) | Single Click | 5 bytes (fixed) |
| [0x0A](c2s-0x0a.md) | Edit | 11 bytes (fixed) |
| [0x12](c2s-0x12.md) | Use Skill / Cast / Action | variable (id + u16 length + payload) |
| [0x13](c2s-0x13.md) | Equip Item Request | 10 bytes (fixed) |
| [0x14](c2s-0x14.md) | ChangeZ | 6 bytes (fixed) |
| [0x22](c2s-0x22.md) | Resynchronize | 3 bytes (fixed) |
| [0x2C](c2s-0x2c.md) | DeathStatusResponse | 2 bytes (fixed) |
| [0x34](c2s-0x34.md) | Get Player Status | 10 bytes (fixed) |
| [0x3A](c2s-0x3a.md) | Set Skill Lock | variable (id + u16 length + payload) |
| [0x3B](c2s-0x3b.md) | VendorBuyReply | variable (id + u16 length + payload) |
| [0x47](c2s-0x47.md) | NewTerrain | 11 bytes (fixed) |
| [0x48](c2s-0x48.md) | NewAnimData | 73 bytes (fixed) |
| [0x4B](c2s-0x4b.md) | Check Client Version | 9 bytes (fixed) |
| [0x52](c2s-0x52.md) | Board Post Message | variable (id + u16 length + payload) |
| [0x58](c2s-0x58.md) | NewRegion | 106 bytes (fixed) |
| [0x5D](c2s-0x5d.md) | Play Character | 73 bytes (fixed) |
| [0x61](c2s-0x61.md) | DeleteStatic | 9 bytes (fixed) |
| [0x6B](c2s-0x6b.md) | Not documented | 9 bytes (fixed) |
| [0x6C](c2s-0x6c.md) | Target Response | 19 bytes (fixed) |
| [0x6F](c2s-0x6f.md) | SecureTrade | variable (id + u16 length + payload) |
| [0x72](c2s-0x72.md) | Request War Mode | 5 bytes (fixed) |
| [0x73](c2s-0x73.md) | Ping | 2 bytes (fixed) |
| [0x75](c2s-0x75.md) | RenameRequest | 35 bytes (fixed) |
| [0x79](c2s-0x79.md) | ResourceQuery | 9 bytes (fixed) |
| [0x7D](c2s-0x7d.md) | MenuResponse | 13 bytes (fixed) |
| [0x7E](c2s-0x7e.md) | GodviewQuery | 2 bytes (fixed) |
| [0x80](c2s-0x80.md) | Login Request | 62 bytes (fixed) |
| [0x83](c2s-0x83.md) | DeleteCharacter | 39 bytes (fixed) |
| [0x84](c2s-0x84.md) | Change Password | 69 bytes (fixed) |
| [0x8D](c2s-0x8d.md) | ECCreateCharacter | variable (id + u16 length + payload); 146 bytes in clients |
| [0x91](c2s-0x91.md) | Game Server Login | 65 bytes (fixed) |
| [0x95](c2s-0x95.md) | HuePickerResponse | 9 bytes (fixed) |
| [0x96](c2s-0x96.md) | GameCentralMoniter | variable (id + u16 length + payload) |
| [0x98](c2s-0x98.md) | MobileNameRequest | variable (id + u16 length + payload) |
| [0x9A](c2s-0x9a.md) | AsciiPromptResponse | variable (id + u16 length + payload) |
| [0x9B](c2s-0x9b.md) | Help Request | 258 bytes (fixed) |
| [0x9D](c2s-0x9d.md) | GMSingle | 51 bytes (fixed) |
| [0x9F](c2s-0x9f.md) | Sell Reply | variable (id + u16 length + payload) |
| [0xA0](c2s-0xa0.md) | Select Server | 3 bytes (fixed) |
| [0xA4](c2s-0xa4.md) | System Info | 149 bytes (fixed) |
| [0xA7](c2s-0xa7.md) | Request Tip | 4 bytes (fixed) |
| [0xAC](c2s-0xac.md) | Gump Text Entry Dialog Reply | variable (id + u16 length + payload) |
| [0xAD](c2s-0xad.md) | Unicode Speech Request | variable (id + u16 length + payload) |
| [0xB1](c2s-0xb1.md) | Gump Menu Response | variable (id + u16 length + payload) |
| [0xB6](c2s-0xb6.md) | Object Help Request | 9 bytes (fixed) |
| [0xB8](c2s-0xb8.md) | ProfileReq | variable (id + u16 length + payload) |
| [0xBB](c2s-0xbb.md) | AccountID | 9 bytes (fixed) |
| [0xBD](c2s-0xbd.md) | Client Version | variable (id + u16 length + payload) |
| [0xBE](c2s-0xbe.md) | AssistVersion | variable (id + u16 length + payload) |
| [0xC2](c2s-0xc2.md) | UnicodePromptResponse | variable (id + u16 length + payload) |
| [0xC8](c2s-0xc8.md) | SetUpdateRange | 2 bytes (fixed) |
| [0xC9](c2s-0xc9.md) | TripTime | 6 bytes (fixed) |
| [0xCA](c2s-0xca.md) | UTripTime | 6 bytes (fixed) |
| [0xCF](c2s-0xcf.md) | AccountLogin | 78 bytes (fixed) |
| [0xD0](c2s-0xd0.md) | ConfigurationFile | variable (id + u16 length + payload) |
| [0xD1](c2s-0xd1.md) | LogoutReq | 2 bytes (fixed) |
| [0xD6](c2s-0xd6.md) | BatchQueryProperties | variable (id + u16 length + payload) |
| [0xD9](c2s-0xd9.md) | Spy on Client (hardware) | 268 bytes (fixed) |
| [0xE1](c2s-0xe1.md) | ClientType | variable (id + u16 length + payload) |
| [0xEC](c2s-0xec.md) | EquipMacro | variable (id + u16 length + payload) |
| [0xED](c2s-0xed.md) | UnequipMacro | variable (id + u16 length + payload) |
| [0xEF](c2s-0xef.md) | Login Seed | 21 bytes (fixed) |
| [0xF0](c2s-0xf0.md) | Movement Request (batch) | variable (id + u16 length + payload) |
| [0xF1](c2s-0xf1.md) | Time Sync Request | 9 bytes (fixed) |
| [0xF4](c2s-0xf4.md) | CrashReport | variable (id + u16 length + payload) |
| [0xF8](c2s-0xf8.md) | CreateCharacter70160 | 106 bytes (fixed) |
| [0xFA](c2s-0xfa.md) | Unhandled (UO Store) | 1 byte (fixed) |
| [0xFB](c2s-0xfb.md) | PublicHouseContent | 2 bytes (fixed) |

## `0xBF` General Information (subcommands)

Framing and dispatch: [bf-framing.md](bf-framing.md).

| Packet | Name | Size |
|---|---|---|
| [0xBF / 0x01](bf-0x01.md) | Fastwalk: initialize key stack | fixed 29 bytes total |
| [0xBF / 0x02](bf-0x02.md) | Fastwalk: add key | fixed 9 bytes total |
| [0xBF / 0x04](bf-0x04.md) | Close generic gump | fixed 13 bytes total |
| [0xBF / 0x05](bf-0x05.md) | Screen size | fixed 13 bytes total |
| [0xBF / 0x06](bf-0x06.md) | Party commands | variable (depends on the command) |
| [0xBF / 0x07](bf-0x07.md) | Click quest arrow | fixed 6 bytes total |
| [0xBF / 0x08](bf-0x08.md) | Change map | fixed 6 bytes total |
| [0xBF / 0x09](bf-0x09.md) | Disarm (wrestling) | fixed 5 bytes total |
| [0xBF / 0x0A](bf-0x0a.md) | Stun (wrestling) | fixed 5 bytes total |
| [0xBF / 0x0B](bf-0x0b.md) | Client language | fixed 9 bytes total |
| [0xBF / 0x0C](bf-0x0c.md) | Close status bar | fixed 9 bytes total |
| [0xBF / 0x0E](bf-0x0e.md) | Animation (macro) | fixed 9 bytes total |
| [0xBF / 0x0F](bf-0x0f.md) | Client information | fixed 10 bytes total |
| [0xBF / 0x10](bf-0x10.md) | Object properties and equipment info | C->S fixed 9 bytes; S->C variable (minimum 17) |
| [0xBF / 0x13](bf-0x13.md) | Request context menu | fixed 9 bytes total |
| [0xBF / 0x14](bf-0x14.md) | Display context menu | variable (`12 + sum of the entries`) |
| [0xBF / 0x15](bf-0x15.md) | Context menu response | fixed 11 bytes total |
| [0xBF / 0x16](bf-0x16.md) | Close interface window | fixed 13 bytes total |
| [0xBF / 0x17](bf-0x17.md) | Help topic (Codex of Wisdom) | fixed 11 bytes total |
| [0xBF / 0x18](bf-0x18.md) | Enable map patches | variable (`9 + 8 * mapCount`) |
| [0xBF / 0x19](bf-0x19.md) | Extended stats | depends on the variant (11, 12 or 17 bytes) |
| [0xBF / 0x1A](bf-0x1a.md) | Change attribute lock | fixed 7 bytes total |
| [0xBF / 0x1B](bf-0x1b.md) | Spellbook contents (new) | fixed 23 bytes total |
| [0xBF / 0x1C](bf-0x1c.md) | Select/cast spell | 9 or 13 bytes total |
| [0xBF / 0x1D](bf-0x1d.md) | House design revision | fixed 13 bytes total |
| [0xBF / 0x1E](bf-0x1e.md) | Request house design details | fixed 9 bytes total |
| [0xBF / 0x20](bf-0x20.md) | Start/end house customization | fixed 17 bytes total |
| [0xBF / 0x21](bf-0x21.md) | Clear weapon ability | fixed 5 bytes total |
| [0xBF / 0x22](bf-0x22.md) | Damage (old format) | fixed 11 bytes total |
| [0xBF / 0x24](bf-0x24.md) | Anti-cheat (unknown) | variable, not specified |
| [0xBF / 0x25](bf-0x25.md) | Toggle spell/ability icon | fixed 8 bytes total |
| [0xBF / 0x26](bf-0x26.md) | Speed mode | fixed 6 bytes total |
| [0xBF / 0x2A](bf-0x2a.md) | Race change | S->C 7 bytes; C->S 5 or 15 bytes |
| [0xBF / 0x2B](bf-0x2b.md) | Animation by partial serial | fixed 9 bytes total |
| [0xBF / 0x2C](bf-0x2c.md) | Use bandage on target | fixed 13 bytes total |
| [0xBF / 0x2D](bf-0x2d.md) | Targeted spell | fixed 11 bytes total |
| [0xBF / 0x2E](bf-0x2e.md) | Targeted skill | fixed 11 bytes total |
| [0xBF / 0x30](bf-0x30.md) | Resource target (macro) | fixed 11 bytes total |
| [0xBF / 0x31](bf-0x31.md) | Bandage timer | fixed 15 bytes total |
| [0xBF / 0x32](bf-0x32.md) | Toggle flying (gargoyle) | fixed 11 bytes total |
| [0xBF / 0x33](bf-0x33.md) | Move boat with the mouse | fixed 12 bytes total |

## `0xD7` Encoded Client Packet (subcommands)

Framing and dispatch: [d7-framing.md](d7-framing.md).

| Packet | Name | Size |
|---|---|---|
| [0xD7 / 0x02](d7-0x02.md) | House: backup | fixed 10 bytes total |
| [0xD7 / 0x03](d7-0x03.md) | House: restore | fixed 10 bytes total |
| [0xD7 / 0x04](d7-0x04.md) | House: commit | fixed 10 bytes total |
| [0xD7 / 0x05](d7-0x05.md) | House: remove item | fixed 30 bytes total |
| [0xD7 / 0x06](d7-0x06.md) | House: place item | fixed 25 bytes total |
| [0xD7 / 0x0A](d7-0x0a.md) | House: response/action (unknown) | fixed 10 bytes total |
| [0xD7 / 0x0C](d7-0x0c.md) | House: exit designer | fixed 10 bytes total |
| [0xD7 / 0x0D](d7-0x0d.md) | House: place stairs (multi) | fixed 25 bytes total |
| [0xD7 / 0x0E](d7-0x0e.md) | House: synchronize | fixed 10 bytes total |
| [0xD7 / 0x0F](d7-0x0f.md) | House: action (unknown) | not specified |
| [0xD7 / 0x10](d7-0x10.md) | House: clear | fixed 10 bytes total |
| [0xD7 / 0x12](d7-0x12.md) | House: change floor | fixed 15 bytes total |
| [0xD7 / 0x13](d7-0x13.md) | House: place roof | fixed 30 bytes total |
| [0xD7 / 0x14](d7-0x14.md) | House: remove roof | fixed 30 bytes total |
| [0xD7 / 0x19](d7-0x19.md) | Activate special ability | fixed 15 bytes total |
| [0xD7 / 0x1A](d7-0x1a.md) | House: revert | fixed 10 bytes total |
| [0xD7 / 0x1E](d7-0x1e.md) | Equip last weapon | fixed 10 bytes total |
| [0xD7 / 0x28](d7-0x28.md) | Guild button | fixed 10 bytes total |
| [0xD7 / 0x32](d7-0x32.md) | Quest button | fixed 10 bytes total |
