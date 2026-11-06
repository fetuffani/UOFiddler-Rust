# Pacotes do protocolo (client 7.0.x)

Tabelas de pacotes do cliente Ultima Online 7.0.x. Coluna **Tamanho**: número de bytes do pacote inteiro (incluindo o byte de id) ou `var` para tamanho variável. `—` indica tamanho não fixado pela tabela do cliente.

## Servidor para cliente

| ID | Tamanho | Nome | Formato |
|---|---|---|---|
| 0x0B | 7 | Damage | [formato](packets/s2c-0x0b.md) |
| 0x11 | var | Mobile Status (HP/stats) | [formato](packets/s2c-0x11.md) |
| 0x15 | 9 | Follow | [formato](packets/s2c-0x15.md) |
| 0x16 | var | Healthbar update (SA/EC) | [formato](packets/s2c-0x16.md) |
| 0x17 | var | Health Bar Status Update | [formato](packets/s2c-0x17.md) |
| 0x1A | var | Object Info (item no chão) | [formato](packets/s2c-0x1a.md) |
| 0x1B | 37 | Login Confirm (personagem entrou) | [formato](packets/s2c-0x1b.md) |
| 0x1C | var | Speech ASCII | [formato](packets/s2c-0x1c.md) |
| 0x1D | 5 | Delete Object | [formato](packets/s2c-0x1d.md) |
| 0x20 | 19 | Draw Game Player (posição/direção própria) | [formato](packets/s2c-0x20.md) |
| 0x21 | 8 | Move Reject | [formato](packets/s2c-0x21.md) |
| 0x22 | 3 | Move Acknowledge | [formato](packets/s2c-0x22.md) |
| 0x23 | 26 | Dragging Item (animação) | [formato](packets/s2c-0x23.md) |
| 0x24 | 9 | Draw/Open Container | [formato](packets/s2c-0x24.md) |
| 0x25 | 21 | Add Item To Container | [formato](packets/s2c-0x25.md) |
| 0x27 | 2 | Reject Move Item | [formato](packets/s2c-0x27.md) |
| 0x28 | 5 | Drop Item Failed / Clear Square | [formato](packets/s2c-0x28.md) |
| 0x29 | 1 | Drop Item Approved | [formato](packets/s2c-0x29.md) |
| 0x2C | 2 | Resurrection Menu / Death | [formato](packets/s2c-0x2c.md) |
| 0x2D | 17 | Mobile Attributes | [formato](packets/s2c-0x2d.md) |
| 0x2E | 15 | Equip/Worn Item | [formato](packets/s2c-0x2e.md) |
| 0x2F | 10 | Fight Occurring | [formato](packets/s2c-0x2f.md) |
| 0x30 | 5 | Attack OK | [formato](packets/s2c-0x30.md) |
| 0x31 | var | Attack Ended | [formato](packets/s2c-0x31.md) |
| 0x33 | 2 | Pause/Resume Client | [formato](packets/s2c-0x33.md) |
| 0x38 | 7 | Pathfinding | [formato](packets/s2c-0x38.md) |
| 0x39 | 9 | Remove (Group) | [formato](packets/s2c-0x39.md) |
| 0x3A | var | Skills List/Update | [formato](packets/s2c-0x3a.md) |
| 0x3B | var | End Vendor Buy | [formato](packets/s2c-0x3b.md) |
| 0x3C | var | Add Multiple Items To Container (conteúdo da bag) | [formato](packets/s2c-0x3c.md) |
| 0x3E | 37 | Versions (God client) | [formato](packets/s2c-0x3e.md) |
| 0x3F | var | Update Statics (God client; UltimaLive command) | [formato](packets/s2c-0x3f.md) |
| 0x40 | 201 | Update Terrain (God client; UltimaLive) | [formato](packets/s2c-0x40.md) |
| 0x41 | var | Update Tiledata (God client) | [formato](packets/s2c-0x41.md) |
| 0x42 | var | Update Art (God client) | [formato](packets/s2c-0x42.md) |
| 0x43 | 553 | Update Animation (God client) | [formato](packets/s2c-0x43.md) |
| 0x44 | 713 | Update Hues (God client) | [formato](packets/s2c-0x44.md) |
| 0x45 | 5 | Version OK | [formato](packets/s2c-0x45.md) |
| 0x4E | 6 | Personal Light Level | [formato](packets/s2c-0x4e.md) |
| 0x4F | 2 | Overall Light Level | [formato](packets/s2c-0x4f.md) |
| 0x53 | 2 | Reject Character Logon / Popup message | [formato](packets/s2c-0x53.md) |
| 0x54 | 12 | Play Sound Effect | [formato](packets/s2c-0x54.md) |
| 0x55 | 1 | Login Complete | [formato](packets/s2c-0x55.md) |
| 0x56 | 11 | Map Packet (cartography/treasure) | [formato](packets/s2c-0x56.md) |
| 0x57 | 110 | Update Regions | [formato](packets/s2c-0x57.md) |
| 0x5B | 4 | Time | [formato](packets/s2c-0x5b.md) |
| 0x65 | 4 | Set Weather | [formato](packets/s2c-0x65.md) |
| 0x66 | var | Book Pages | [formato](packets/s2c-0x66.md) |
| 0x69 | var | Change Text/Emote Colors (Options) | [formato](packets/s2c-0x69.md) |
| 0x6A | 3 | Não documentado (3 bytes) | [formato](packets/s2c-0x6a.md) |
| 0x6C | 19 | Target Cursor | [formato](packets/s2c-0x6c.md) |
| 0x6D | 3 | Play Music (MIDI) | [formato](packets/s2c-0x6d.md) |
| 0x6E | 14 | Character Animation | [formato](packets/s2c-0x6e.md) |
| 0x6F | var | Secure Trading | [formato](packets/s2c-0x6f.md) |
| 0x70 | 28 | Graphical Effect | [formato](packets/s2c-0x70.md) |
| 0x71 | var | Bulletin Board | [formato](packets/s2c-0x71.md) |
| 0x72 | 5 | War Mode | [formato](packets/s2c-0x72.md) |
| 0x73 | 2 | Ping | [formato](packets/s2c-0x73.md) |
| 0x74 | var | Open Buy Window (lista da loja) | [formato](packets/s2c-0x74.md) |
| 0x76 | 16 | New Subserver | [formato](packets/s2c-0x76.md) |
| 0x77 | 17 | Update Mobile (move/animação de outro) | [formato](packets/s2c-0x77.md) |
| 0x78 | var | Mobile Incoming (aparece mobile + equip) | [formato](packets/s2c-0x78.md) |
| 0x7B | 2 | Sequence | [formato](packets/s2c-0x7b.md) |
| 0x7C | var | Open Dialog Box (menu) | [formato](packets/s2c-0x7c.md) |
| 0x81 | var | Change Character | [formato](packets/s2c-0x81.md) |
| 0x82 | 2 | Login Denied | [formato](packets/s2c-0x82.md) |
| 0x85 | 2 | Character Delete Result | [formato](packets/s2c-0x85.md) |
| 0x86 | var | Character List Update | [formato](packets/s2c-0x86.md) |
| 0x88 | 66 | Open Paperdoll | [formato](packets/s2c-0x88.md) |
| 0x89 | var | Corpse Clothing | [formato](packets/s2c-0x89.md) |
| 0x8B | var | Display Sign Gump | [formato](packets/s2c-0x8b.md) |
| 0x8C | 11 | Connect To Game Server | [formato](packets/s2c-0x8c.md) |
| 0x90 | 19 | Map Message | [formato](packets/s2c-0x90.md) |
| 0x92 | var | Não documentado | [formato](packets/s2c-0x92.md) |
| 0x93 | 99 | Book Header (old) | [formato](packets/s2c-0x93.md) |
| 0x94 | var | Não documentado | [formato](packets/s2c-0x94.md) |
| 0x95 | 9 | Dye Window | [formato](packets/s2c-0x95.md) |
| 0x97 | 2 | Move Player | [formato](packets/s2c-0x97.md) |
| 0x98 | var | Mobile Name | [formato](packets/s2c-0x98.md) |
| 0x99 | 30 | Multi Placement (26 bytes em clientes anteriores ao 7.0.9.0) | [formato](packets/s2c-0x99.md) |
| 0x9A | var | Console Entry Prompt | [formato](packets/s2c-0x9a.md) |
| 0x9E | var | Sell List | [formato](packets/s2c-0x9e.md) |
| 0xA1 | 9 | Update Current HP | [formato](packets/s2c-0xa1.md) |
| 0xA2 | 9 | Update Current Mana | [formato](packets/s2c-0xa2.md) |
| 0xA3 | 9 | Update Current Stamina | [formato](packets/s2c-0xa3.md) |
| 0xA5 | var | Open Web Browser | [formato](packets/s2c-0xa5.md) |
| 0xA6 | var | Tip/Notice | [formato](packets/s2c-0xa6.md) |
| 0xA8 | var | Game Server List | [formato](packets/s2c-0xa8.md) |
| 0xA9 | var | Characters + Starting Locations | [formato](packets/s2c-0xa9.md) |
| 0xAA | 5 | Attack Target (current) | [formato](packets/s2c-0xaa.md) |
| 0xAB | var | Gump Text Entry Dialog | [formato](packets/s2c-0xab.md) |
| 0xAE | var | Unicode Speech | [formato](packets/s2c-0xae.md) |
| 0xAF | 13 | Display Death Action | [formato](packets/s2c-0xaf.md) |
| 0xB0 | var | Gump Menu Dialog (gump) | [formato](packets/s2c-0xb0.md) |
| 0xB2 | var | Chat Message | [formato](packets/s2c-0xb2.md) |
| 0xB3 | var | Chat Text | [formato](packets/s2c-0xb3.md) |
| 0xB4 | var | Target Object List | [formato](packets/s2c-0xb4.md) |
| 0xB5 | 64 | Open Chat Window | [formato](packets/s2c-0xb5.md) |
| 0xB7 | var | Help/Tip Data | [formato](packets/s2c-0xb7.md) |
| 0xB8 | var | Character Profile | [formato](packets/s2c-0xb8.md) |
| 0xB9 | 5 | Enable Locked Client Features | [formato](packets/s2c-0xb9.md) |
| 0xBA | 10 | Quest Arrow | [formato](packets/s2c-0xba.md) |
| 0xBB | 9 | Ultima Messenger | [formato](packets/s2c-0xbb.md) |
| 0xBC | 3 | Seasonal Info | [formato](packets/s2c-0xbc.md) |
| 0xBD | var | Client Version request | [formato](packets/s2c-0xbd.md) |
| 0xBE | var | Assist Version | [formato](packets/s2c-0xbe.md) |
| 0xBF | var | General Information (subcomandos) | [formato](packets/bf-framing.md) |
| 0xC0 | 36 | Hued Effect | [formato](packets/s2c-0xc0.md) |
| 0xC1 | var | Localized Message (cliloc) | [formato](packets/s2c-0xc1.md) |
| 0xC2 | var | Unicode Prompt | [formato](packets/s2c-0xc2.md) |
| 0xC3 | var | GQ Request | [formato](packets/s2c-0xc3.md) |
| 0xC4 | 6 | Semivisible | [formato](packets/s2c-0xc4.md) |
| 0xC6 | 1 | Invalid Map Enable | [formato](packets/s2c-0xc6.md) |
| 0xC7 | 49 | 3D Particle Effect | [formato](packets/s2c-0xc7.md) |
| 0xC8 | 2 | Update Range | [formato](packets/s2c-0xc8.md) |
| 0xC9 | 6 | Trip Time | [formato](packets/s2c-0xc9.md) |
| 0xCA | 6 | UTrip Time | [formato](packets/s2c-0xca.md) |
| 0xCB | 7 | Global Queue Count | [formato](packets/s2c-0xcb.md) |
| 0xCC | var | Localized Message Affix | [formato](packets/s2c-0xcc.md) |
| 0xCD | 1 | Não documentado (1 byte) | [formato](packets/s2c-0xcd.md) |
| 0xCE | var | Não documentado | [formato](packets/s2c-0xce.md) |
| 0xD0 | var | Configuration File | [formato](packets/s2c-0xd0.md) |
| 0xD1 | 2 | Logout Status | [formato](packets/s2c-0xd1.md) |
| 0xD2 | 25 | Extended 0x20 (Mobile Update) | [formato](packets/s2c-0xd2.md) |
| 0xD3 | var | Extended 0x78 (Mobile Incoming) | [formato](packets/s2c-0xd3.md) |
| 0xD4 | var | Book Header (novo) | [formato](packets/s2c-0xd4.md) |
| 0xD6 | var | Mega Cliloc (properties/tooltips) | [formato](packets/s2c-0xd6.md) |
| 0xD8 | var | Custom House | [formato](packets/s2c-0xd8.md) |
| 0xDA | var | Mahjong (game packets) | [formato](packets/s2c-0xda.md) |
| 0xDB | var | Character Transfer Log | [formato](packets/s2c-0xdb.md) |
| 0xDC | 9 | SE Introduced Revision (hash de props) | [formato](packets/s2c-0xdc.md) |
| 0xDD | var | Compressed Gump | [formato](packets/s2c-0xdd.md) |
| 0xDE | var | Update Mobile Status | [formato](packets/s2c-0xde.md) |
| 0xDF | var | Buff/Debuff | [formato](packets/s2c-0xdf.md) |
| 0xE2 | 10 | New Character Animation | [formato](packets/s2c-0xe2.md) |
| 0xE5 | var | Display Waypoint | [formato](packets/s2c-0xe5.md) |
| 0xE6 | 5 | Remove Waypoint | [formato](packets/s2c-0xe6.md) |
| 0xE7 | 12 | Highlight UI Continue | [formato](packets/s2c-0xe7.md) |
| 0xE9 | 75 | Highlight UI Toggle | [formato](packets/s2c-0xe9.md) |
| 0xEA | 3 | Toggle Hotbar | [formato](packets/s2c-0xea.md) |
| 0xF2 | 25 | Time Sync Response | [formato](packets/s2c-0xf2.md) |
| 0xF3 | 26 | Object Info (SA/novo) | [formato](packets/s2c-0xf3.md) |
| 0xF5 | 21 | New Map Message | [formato](packets/s2c-0xf5.md) |
| 0xF6 | var | Boat Moving | [formato](packets/s2c-0xf6.md) |
| 0xF7 | var | Packet Container (agrupa entradas 0xF3 de um mesmo movimento) | [formato](packets/s2c-0xf7.md) |

## Cliente para servidor

| ID | Tamanho | Nome | Formato |
|---|---|---|---|
| 0x00 | 104 | Create Character | [formato](packets/c2s-0x00.md) |
| 0x01 | 5 | Disconnect / Logout | [formato](packets/c2s-0x01.md) |
| 0x02 | 7 | Move Request | [formato](packets/c2s-0x02.md) |
| 0x03 | var | Talk (ASCII) | [formato](packets/c2s-0x03.md) |
| 0x04 | 2 | GodModeRequest | [formato](packets/c2s-0x04.md) |
| 0x05 | 5 | Attack Request | [formato](packets/c2s-0x05.md) |
| 0x06 | 5 | Double Click (usar) | [formato](packets/c2s-0x06.md) |
| 0x07 | 7 | Pick Up Item | [formato](packets/c2s-0x07.md) |
| 0x08 | 15 | Drop Item | [formato](packets/c2s-0x08.md) |
| 0x09 | 5 | Single Click (look) | [formato](packets/c2s-0x09.md) |
| 0x0A | 11 | Edit | [formato](packets/c2s-0x0a.md) |
| 0x12 | var | Use Skill / Cast / Action | [formato](packets/c2s-0x12.md) |
| 0x13 | 10 | Equip Item Request | [formato](packets/c2s-0x13.md) |
| 0x14 | 6 | ChangeZ | [formato](packets/c2s-0x14.md) |
| 0x22 | 3 | Resynchronize | [formato](packets/c2s-0x22.md) |
| 0x2C | 2 | DeathStatusResponse | [formato](packets/c2s-0x2c.md) |
| 0x34 | 10 | Get Player Status | [formato](packets/c2s-0x34.md) |
| 0x3A | var | Set Skill Lock | [formato](packets/c2s-0x3a.md) |
| 0x3B | var | VendorBuyReply | [formato](packets/c2s-0x3b.md) |
| 0x47 | 11 | NewTerrain | [formato](packets/c2s-0x47.md) |
| 0x48 | 73 | NewAnimData | [formato](packets/c2s-0x48.md) |
| 0x4B | 9 | Check Client Version | [formato](packets/c2s-0x4b.md) |
| 0x52 | var | Board Post Message | [formato](packets/c2s-0x52.md) |
| 0x58 | 106 | NewRegion | [formato](packets/c2s-0x58.md) |
| 0x5D | 73 | Play Character (selecionar) | [formato](packets/c2s-0x5d.md) |
| 0x61 | 9 | DeleteStatic | [formato](packets/c2s-0x61.md) |
| 0x6B | 9 | Não documentado | [formato](packets/c2s-0x6b.md) |
| 0x6C | 19 | Target Response | [formato](packets/c2s-0x6c.md) |
| 0x6F | var | SecureTrade | [formato](packets/c2s-0x6f.md) |
| 0x72 | 5 | Request War Mode | [formato](packets/c2s-0x72.md) |
| 0x73 | 2 | Ping | [formato](packets/c2s-0x73.md) |
| 0x75 | 35 | RenameRequest | [formato](packets/c2s-0x75.md) |
| 0x79 | 9 | ResourceQuery | [formato](packets/c2s-0x79.md) |
| 0x7D | 13 | MenuResponse | [formato](packets/c2s-0x7d.md) |
| 0x7E | 2 | GodviewQuery | [formato](packets/c2s-0x7e.md) |
| 0x80 | 62 | Login Request | [formato](packets/c2s-0x80.md) |
| 0x83 | 39 | DeleteCharacter | [formato](packets/c2s-0x83.md) |
| 0x84 | 69 | Change Password | [formato](packets/c2s-0x84.md) |
| 0x8D | var | ECCreateCharacter | [formato](packets/c2s-0x8d.md) |
| 0x91 | 65 | Game Server Login | [formato](packets/c2s-0x91.md) |
| 0x95 | 9 | HuePickerResponse | [formato](packets/c2s-0x95.md) |
| 0x96 | var | GameCentralMoniter | [formato](packets/c2s-0x96.md) |
| 0x98 | var | MobileNameRequest | [formato](packets/c2s-0x98.md) |
| 0x9A | var | AsciiPromptResponse | [formato](packets/c2s-0x9a.md) |
| 0x9B | 258 | Help Request | [formato](packets/c2s-0x9b.md) |
| 0x9D | 51 | GMSingle | [formato](packets/c2s-0x9d.md) |
| 0x9F | var | Sell Reply | [formato](packets/c2s-0x9f.md) |
| 0xA0 | 3 | Select Server | [formato](packets/c2s-0xa0.md) |
| 0xA4 | 149 | System Info | [formato](packets/c2s-0xa4.md) |
| 0xA7 | 4 | Request Tip | [formato](packets/c2s-0xa7.md) |
| 0xAC | var | Gump Text Entry Dialog Reply | [formato](packets/c2s-0xac.md) |
| 0xAD | var | Unicode Speech Request | [formato](packets/c2s-0xad.md) |
| 0xB1 | var | Gump Menu Response | [formato](packets/c2s-0xb1.md) |
| 0xB6 | 9 | Object Help Request | [formato](packets/c2s-0xb6.md) |
| 0xB8 | var | ProfileReq | [formato](packets/c2s-0xb8.md) |
| 0xBB | 9 | AccountID | [formato](packets/c2s-0xbb.md) |
| 0xBD | var | Client Version | [formato](packets/c2s-0xbd.md) |
| 0xBE | var | AssistVersion | [formato](packets/c2s-0xbe.md) |
| 0xBF | var | General Information | [formato](packets/bf-framing.md) |
| 0xC2 | var | UnicodePromptResponse | [formato](packets/c2s-0xc2.md) |
| 0xC8 | 2 | SetUpdateRange | [formato](packets/c2s-0xc8.md) |
| 0xC9 | 6 | TripTime | [formato](packets/c2s-0xc9.md) |
| 0xCA | 6 | UTripTime | [formato](packets/c2s-0xca.md) |
| 0xCF | 78 | AccountLogin | [formato](packets/c2s-0xcf.md) |
| 0xD0 | var | ConfigurationFile | [formato](packets/c2s-0xd0.md) |
| 0xD1 | 2 | LogoutReq | [formato](packets/c2s-0xd1.md) |
| 0xD6 | var | BatchQueryProperties | [formato](packets/c2s-0xd6.md) |
| 0xD7 | var | Encoded Command (AOS) | [formato](packets/d7-framing.md) |
| 0xD9 | 268 | Spy on Client (pesquisa de hardware) | [formato](packets/c2s-0xd9.md) |
| 0xE1 | var | ClientType | [formato](packets/c2s-0xe1.md) |
| 0xEC | var | EquipMacro | [formato](packets/c2s-0xec.md) |
| 0xED | var | UnequipMacro | [formato](packets/c2s-0xed.md) |
| 0xEF | — | Login Seed | [formato](packets/c2s-0xef.md) |
| 0xF0 | var | Movement Request (lote) | [formato](packets/c2s-0xf0.md) |
| 0xF1 | 9 | Time Sync Request | [formato](packets/c2s-0xf1.md) |
| 0xF4 | var | CrashReport | [formato](packets/c2s-0xf4.md) |
| 0xF8 | — | CreateCharacter70160 | [formato](packets/c2s-0xf8.md) |
| 0xFA | — | Unhandled | [formato](packets/c2s-0xfa.md) |
| 0xFB | 2 | PublicHouseContent (id + flag) | [formato](packets/c2s-0xfb.md) |

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
