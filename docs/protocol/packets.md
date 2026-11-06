# Pacotes do protocolo (client 7.0.x)

Tabelas de pacotes do cliente Ultima Online 7.0.x. Coluna **Tamanho**: número de bytes do pacote inteiro (incluindo o byte de id) ou `var` para tamanho variável. `—` indica tamanho não fixado pela tabela do cliente.

## Servidor para cliente

| ID | Tamanho | Nome |
|---|---|---|
| 0x0B | 7 | Damage |
| 0x11 | var | Mobile Status (HP/stats) |
| 0x15 | 9 | Follow |
| 0x16 | var | Healthbar update (SA/EC) |
| 0x17 | var | Health Bar Status Update |
| 0x1A | var | Object Info (item no chão) |
| 0x1B | 37 | Login Confirm (personagem entrou) |
| 0x1C | var | Speech ASCII |
| 0x1D | 5 | Delete Object |
| 0x20 | 19 | Draw Game Player (posição/direção própria) |
| 0x21 | 8 | Move Reject |
| 0x22 | 3 | Move Acknowledge |
| 0x23 | 26 | Dragging Item (animação) |
| 0x24 | 9 | Draw/Open Container |
| 0x25 | 21 | Add Item To Container |
| 0x27 | 2 | Reject Move Item |
| 0x28 | 5 | Drop Item Failed / Clear Square |
| 0x29 | 1 | Drop Item Approved |
| 0x2C | 2 | Resurrection Menu / Death |
| 0x2D | 17 | Mobile Attributes |
| 0x2E | 15 | Equip/Worn Item |
| 0x2F | 10 | Fight Occurring |
| 0x30 | 5 | Attack OK |
| 0x31 | var | Attack Ended |
| 0x33 | 2 | Pause/Resume Client |
| 0x38 | 7 | Pathfinding |
| 0x39 | 9 | Remove (Group) |
| 0x3A | var | Skills List/Update |
| 0x3B | var | End Vendor Buy |
| 0x3C | var | Add Multiple Items To Container (conteúdo da bag) |
| 0x3E | 37 | Versions (God client) |
| 0x3F | var | Update Statics (God client; UltimaLive command) |
| 0x40 | 201 | Update Terrain (God client; UltimaLive) |
| 0x41 | var | Update Tiledata (God client) |
| 0x42 | var | Update Art (God client) |
| 0x43 | 553 | Update Animation (God client) |
| 0x44 | 713 | Update Hues (God client) |
| 0x45 | 5 | Version OK |
| 0x4E | 6 | Personal Light Level |
| 0x4F | 2 | Overall Light Level |
| 0x53 | 2 | Reject Character Logon / Popup message |
| 0x54 | 12 | Play Sound Effect |
| 0x55 | 1 | Login Complete |
| 0x56 | 11 | Map Packet (cartography/treasure) |
| 0x57 | 110 | Update Regions |
| 0x5B | 4 | Time |
| 0x65 | 4 | Set Weather |
| 0x66 | var | Book Pages |
| 0x69 | var | Change Text/Emote Colors (Options) |
| 0x6A | 3 | Não documentado (3 bytes) |
| 0x6C | 19 | Target Cursor |
| 0x6D | 3 | Play Music (MIDI) |
| 0x6E | 14 | Character Animation |
| 0x6F | var | Secure Trading |
| 0x70 | 28 | Graphical Effect |
| 0x71 | var | Bulletin Board |
| 0x72 | 5 | War Mode |
| 0x73 | 2 | Ping |
| 0x74 | var | Open Buy Window (lista da loja) |
| 0x76 | 16 | New Subserver |
| 0x77 | 17 | Update Mobile (move/animação de outro) |
| 0x78 | var | Mobile Incoming (aparece mobile + equip) |
| 0x7B | 2 | Sequence |
| 0x7C | var | Open Dialog Box (menu) |
| 0x81 | var | Change Character |
| 0x82 | 2 | Login Denied |
| 0x85 | 2 | Character Delete Result |
| 0x86 | var | Character List Update |
| 0x88 | 66 | Open Paperdoll |
| 0x89 | var | Corpse Clothing |
| 0x8B | var | Display Sign Gump |
| 0x8C | 11 | Connect To Game Server |
| 0x90 | 19 | Map Message |
| 0x92 | var | Não documentado |
| 0x93 | 99 | Book Header (old) |
| 0x94 | var | Não documentado |
| 0x95 | 9 | Dye Window |
| 0x97 | 2 | Move Player |
| 0x98 | var | Mobile Name |
| 0x99 | 30 | Multi Placement |
| 0x9A | var | Console Entry Prompt |
| 0x9E | var | Sell List |
| 0xA1 | 9 | Update Current HP |
| 0xA2 | 9 | Update Current Mana |
| 0xA3 | 9 | Update Current Stamina |
| 0xA5 | var | Open Web Browser |
| 0xA6 | var | Tip/Notice |
| 0xA8 | var | Game Server List |
| 0xA9 | var | Characters + Starting Locations |
| 0xAA | 5 | Attack Target (current) |
| 0xAB | var | Gump Text Entry Dialog |
| 0xAE | var | Unicode Speech |
| 0xAF | 13 | Display Death Action |
| 0xB0 | var | Gump Menu Dialog (gump) |
| 0xB2 | var | Chat Message |
| 0xB3 | var | Chat Text |
| 0xB4 | var | Target Object List |
| 0xB5 | 64 | Open Chat Window |
| 0xB7 | var | Help/Tip Data |
| 0xB8 | var | Character Profile |
| 0xB9 | 5 | Enable Locked Client Features |
| 0xBA | 10 | Quest Arrow |
| 0xBB | 9 | Ultima Messenger |
| 0xBC | 3 | Seasonal Info |
| 0xBD | var | Client Version request |
| 0xBE | var | Assist Version |
| 0xBF | var | General Information (subcomandos) |
| 0xC0 | 36 | Hued Effect |
| 0xC1 | var | Localized Message (cliloc) |
| 0xC2 | var | Unicode Prompt |
| 0xC3 | var | GQ Request |
| 0xC4 | 6 | Semivisible |
| 0xC6 | 1 | Invalid Map Enable |
| 0xC7 | 49 | 3D Particle Effect |
| 0xC8 | 2 | Update Range |
| 0xC9 | 6 | Trip Time |
| 0xCA | 6 | UTrip Time |
| 0xCB | 7 | Global Queue Count |
| 0xCC | var | Localized Message Affix |
| 0xCD | 1 | Não documentado (1 byte) |
| 0xCE | var | Não documentado |
| 0xD0 | var | Configuration File |
| 0xD1 | 2 | Logout Status |
| 0xD2 | 25 | Extended 0x20 (Mobile Update) |
| 0xD3 | var | Extended 0x78 (Mobile Incoming) |
| 0xD4 | var | Book Header (novo) |
| 0xD6 | var | Mega Cliloc (properties/tooltips) |
| 0xD8 | var | Custom House |
| 0xDA | var | Mahjong (game packets) |
| 0xDB | var | Character Transfer Log |
| 0xDC | 9 | SE Introduced Revision (hash de props) |
| 0xDD | var | Compressed Gump |
| 0xDE | var | Update Mobile Status |
| 0xDF | var | Buff/Debuff |
| 0xE2 | 10 | New Character Animation |
| 0xE5 | var | Display Waypoint |
| 0xE6 | 5 | Remove Waypoint |
| 0xE7 | 12 | Highlight UI Continue |
| 0xE9 | 75 | Highlight UI Toggle |
| 0xEA | 3 | Toggle Hotbar |
| 0xF2 | 25 | Time Sync Response |
| 0xF3 | 26 | Object Info (SA/novo) |
| 0xF5 | 21 | New Map Message |
| 0xF6 | var | Boat Moving |

## Cliente para servidor

| ID | Tamanho | Nome |
|---|---|---|
| 0x00 | 104 | Create Character |
| 0x01 | 5 | Disconnect / Logout |
| 0x02 | 7 | Move Request |
| 0x03 | var | Talk (ASCII) |
| 0x04 | 2 | GodModeRequest |
| 0x05 | 5 | Attack Request |
| 0x06 | 5 | Double Click (usar) |
| 0x07 | 7 | Pick Up Item |
| 0x08 | 15 | Drop Item |
| 0x09 | 5 | Single Click (look) |
| 0x0A | 11 | Edit |
| 0x12 | var | Use Skill / Cast / Action |
| 0x13 | 10 | Equip Item Request |
| 0x14 | 6 | ChangeZ |
| 0x22 | 3 | Resynchronize |
| 0x2C | 2 | DeathStatusResponse |
| 0x34 | 10 | Get Player Status |
| 0x3A | var | Set Skill Lock |
| 0x3B | var | VendorBuyReply |
| 0x47 | 11 | NewTerrain |
| 0x48 | 73 | NewAnimData |
| 0x4B | 9 | Check Client Version |
| 0x52 | var | Board Post Message |
| 0x58 | 106 | NewRegion |
| 0x5D | 73 | Play Character (selecionar) |
| 0x61 | 9 | DeleteStatic |
| 0x6B | 9 | Não documentado |
| 0x6C | 19 | Target Response |
| 0x6F | var | SecureTrade |
| 0x72 | 5 | Request War Mode |
| 0x73 | 2 | Ping |
| 0x75 | 35 | RenameRequest |
| 0x79 | 9 | ResourceQuery |
| 0x7D | 13 | MenuResponse |
| 0x7E | 2 | GodviewQuery |
| 0x80 | 62 | Login Request |
| 0x83 | 39 | DeleteCharacter |
| 0x84 | 69 | Change Password |
| 0x8D | var | ECCreateCharacter |
| 0x91 | 65 | Game Server Login |
| 0x95 | 9 | HuePickerResponse |
| 0x96 | var | GameCentralMoniter |
| 0x98 | var | MobileNameRequest |
| 0x9A | var | AsciiPromptResponse |
| 0x9B | 258 | Help Request |
| 0x9D | 51 | GMSingle |
| 0x9F | var | Sell Reply |
| 0xA0 | 3 | Select Server |
| 0xA4 | 149 | System Info |
| 0xA7 | 4 | Request Tip |
| 0xAC | var | Gump Text Entry Dialog Reply |
| 0xAD | var | Unicode Speech Request |
| 0xB1 | var | Gump Menu Response |
| 0xB6 | 9 | Object Help Request |
| 0xB8 | var | ProfileReq |
| 0xBB | 9 | AccountID |
| 0xBD | var | Client Version |
| 0xBE | var | AssistVersion |
| 0xBF | var | General Information |
| 0xC2 | var | UnicodePromptResponse |
| 0xC8 | 2 | SetUpdateRange |
| 0xC9 | 6 | TripTime |
| 0xCA | 6 | UTripTime |
| 0xCF | 78 | AccountLogin |
| 0xD0 | var | ConfigurationFile |
| 0xD1 | 2 | LogoutReq |
| 0xD6 | var | BatchQueryProperties |
| 0xD7 | var | Encoded Command (AOS) |
| 0xD9 | 268 | Spy on Client (pesquisa de hardware) |
| 0xE1 | var | ClientType |
| 0xEC | var | EquipMacro |
| 0xED | var | UnequipMacro |
| 0xEF | — | Login Seed |
| 0xF0 | var | Movement Request (lote) |
| 0xF1 | 9 | Time Sync Request |
| 0xF4 | var | CrashReport |
| 0xF8 | — | CreateCharacter70160 |
| 0xFA | — | Unhandled |
| 0xFB | — | PublicHouseContent |

## Observações

- Pacotes de tamanho `var` começam com o id (1 byte) seguido de um campo de tamanho de 2 bytes (u16) com o comprimento total do pacote. Exemplo: `0xB4` tem o layout `[id][u16 len]...`.
- A tabela de tamanhos do cliente cobre os ids `0x00` a `0xF6`.
- `0xBF` (General Information) agrupa subcomandos.
- `0x73` (Ping) e `0x72` (War Mode) existem nos dois sentidos com o mesmo id.
- `0xEF` (Login Seed) é tratado fora da tabela de tamanhos.
- Os ids `0x6A`, `0x92`, `0x94`, `0xCD`, `0xCE` não têm layout documentado. O cliente 7.0.10.3 descarta `0x6A` e `0xCD`.

### 0xB4 Target Object List

Servidor para cliente, tamanho `var`:

| Campo | Tipo |
|---|---|
| id | u8 (`0xB4`) |
| length | u16 |
| allowGround | u8 |
| serial | u32 |
| x, y, z | u16, u16, u16 |
| count | u16 |
| entradas (`count`) | `itemId`, `hue`, `xOff`, `yOff`, `zOff`: 5 x u16 |

Os itens são exibidos como "fantasmas" que acompanham o cursor de alvo; o clique é respondido com `0x6C` (Target Response).

### 0x84 Change Password

Cliente para servidor, 69 bytes: id, `char[30]` senha atual, `char[30]` nova senha, u32, endereço IPv4 local.

### 0xF1 / 0xF2 Time Sync

`0xF1` (cliente para servidor, 9 bytes): id + 8 bytes. A resposta `0xF2` (servidor para cliente, 25 bytes) contém id + 3 x 8 bytes.
