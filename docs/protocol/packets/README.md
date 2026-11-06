# Formato dos pacotes

Um arquivo por pacote, com tamanho, campos byte a byte, variações por versão de cliente e referência de implementação. Convenções: offsets a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; `u8/u16/u32` sem sinal e `i8/i16/i32` com sinal.

Pacotes cujo layout não pôde ser confirmado nas implementações de referência estão marcados como "Não documentado" ou "Não especificado" no próprio arquivo.

## Pacotes servidor -> cliente

| Pacote | Nome | Tamanho |
|---|---|---|
| [0x0B](s2c-0x0b.md) | Damage | 7 bytes |
| [0x11](s2c-0x11.md) | Mobile Status (HP/stats) | variável (id + u16 length + payload) |
| [0x15](s2c-0x15.md) | Follow | 9 bytes |
| [0x16](s2c-0x16.md) | Healthbar update (SA/EC) | variável (id + u16 length + payload) |
| [0x17](s2c-0x17.md) | Health Bar Status Update | variável (id + u16 length + payload) |
| [0x1A](s2c-0x1a.md) | Object Info (item no chão) | variável (id + u16 length + payload) |
| [0x1B](s2c-0x1b.md) | Login Confirm (personagem entrou) | 37 bytes |
| [0x1C](s2c-0x1c.md) | Speech ASCII | variável (id + u16 length + payload) |
| [0x1D](s2c-0x1d.md) | Delete Object | 5 bytes |
| [0x20](s2c-0x20.md) | Draw Game Player (posição/direção própria) | 19 bytes |
| [0x21](s2c-0x21.md) | Move Reject | 8 bytes |
| [0x22](s2c-0x22.md) | Move Acknowledge | 3 bytes |
| [0x23](s2c-0x23.md) | Dragging Item (animação) | 26 bytes |
| [0x24](s2c-0x24.md) | Draw/Open Container | 7 ou 9 bytes (9 em clientes High Seas, 7 |
| [0x25](s2c-0x25.md) | Add Item To Container | 21 bytes (20 bytes antes do cliente 6 |
| [0x27](s2c-0x27.md) | Reject Move Item | 2 bytes |
| [0x28](s2c-0x28.md) | Drop Item Failed / Clear Square | 5 bytes |
| [0x29](s2c-0x29.md) | Drop Item Approved | 1 byte |
| [0x2C](s2c-0x2c.md) | Resurrection Menu / Death | 2 bytes |
| [0x2D](s2c-0x2d.md) | Mobile Attributes | 17 bytes |
| [0x2E](s2c-0x2e.md) | Equip/Worn Item | 15 bytes |
| [0x2F](s2c-0x2f.md) | Fight Occurring | 10 bytes |
| [0x30](s2c-0x30.md) | Attack OK | 5 bytes |
| [0x31](s2c-0x31.md) | Attack Ended / Pet Window | variável (id + u16 length + payload) |
| [0x33](s2c-0x33.md) | Pause/Resume Client | 2 bytes |
| [0x38](s2c-0x38.md) | Pathfinding | 7 bytes |
| [0x39](s2c-0x39.md) | Remove (Group) | 9 bytes |
| [0x3A](s2c-0x3a.md) | Skills List/Update | variável (id + u16 length + payload) |
| [0x3B](s2c-0x3b.md) | End Vendor Buy | variável (id + u16 length + payload) |
| [0x3C](s2c-0x3c.md) | Add Multiple Items To Container (conteúdo da bag) | variável (id + u16 length + payload) |
| [0x3E](s2c-0x3e.md) | Versions (God client) | 37 bytes |
| [0x3F](s2c-0x3f.md) | Update Statics (God client; UltimaLive command) | variável (id + u16 length + payload) |
| [0x40](s2c-0x40.md) | Update Terrain (God client; UltimaLive) | 201 bytes |
| [0x41](s2c-0x41.md) | Update Tiledata (God client) | variável (id + u16 length + payload) |
| [0x42](s2c-0x42.md) | Update Art (God client) | variável (id + u16 length + payload) |
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
| [0x66](s2c-0x66.md) | Book Pages | variável (id + u16 length + payload) |
| [0x69](s2c-0x69.md) | Change Text/Emote Colors (Options) | variável (id + u16 length + payload) |
| [0x6A](s2c-0x6a.md) | Não documentado | 3 bytes |
| [0x6C](s2c-0x6c.md) | Target Cursor | 19 bytes |
| [0x6D](s2c-0x6d.md) | Play Music (MIDI) | 3 bytes |
| [0x6E](s2c-0x6e.md) | Character Animation | 14 bytes |
| [0x6F](s2c-0x6f.md) | Secure Trading | variável (id + u16 length + payload) |
| [0x70](s2c-0x70.md) | Graphical Effect | 28 bytes |
| [0x71](s2c-0x71.md) | Bulletin Board | variável (id + u16 length + payload) |
| [0x72](s2c-0x72.md) | War Mode | 5 bytes |
| [0x73](s2c-0x73.md) | Ping | 2 bytes |
| [0x74](s2c-0x74.md) | Open Buy Window (lista da loja) | variável (id + u16 length + payload) |
| [0x76](s2c-0x76.md) | New Subserver | 16 bytes |
| [0x77](s2c-0x77.md) | Update Mobile (move/animação de outro) | 17 bytes |
| [0x78](s2c-0x78.md) | Mobile Incoming (aparece mobile + equip) | variável (id + u16 length + payload) |
| [0x7B](s2c-0x7b.md) | Sequence | 2 bytes |
| [0x7C](s2c-0x7c.md) | Open Dialog Box (menu) | variável (id + u16 length + payload) |
| [0x81](s2c-0x81.md) | Change Character | variável (id + u16 length + payload; length é o tamanho tota |
| [0x82](s2c-0x82.md) | Login Denied | 2 bytes |
| [0x85](s2c-0x85.md) | Character Delete Result | 2 bytes |
| [0x86](s2c-0x86.md) | Character List Update | variável (id + u16 length + payload; length é o tamanho tota |
| [0x88](s2c-0x88.md) | Open Paperdoll | 66 bytes |
| [0x89](s2c-0x89.md) | Corpse Clothing | variável (id + u16 length + payload; length é o tamanho tota |
| [0x8B](s2c-0x8b.md) | Display Sign Gump | variável (id + u16 length + payload; length é o tamanho tota |
| [0x8C](s2c-0x8c.md) | Connect To Game Server | 11 bytes |
| [0x90](s2c-0x90.md) | Map Message | 19 bytes |
| [0x92](s2c-0x92.md) | Não documentado | variável (id + u16 length + payload; length é o tamanho tota |
| [0x93](s2c-0x93.md) | Book Header (old) | 99 bytes |
| [0x94](s2c-0x94.md) | Não documentado | variável (id + u16 length + payload; length é o tamanho tota |
| [0x95](s2c-0x95.md) | Dye Window | 9 bytes |
| [0x97](s2c-0x97.md) | Move Player | 2 bytes |
| [0x98](s2c-0x98.md) | Mobile Name | variável (id + u16 length + payload; length é o tamanho tota |
| [0x99](s2c-0x99.md) | Multi Placement | 30 bytes (clientes HS) | 26 bytes (formato anterior) |
| [0x9A](s2c-0x9a.md) | Console Entry Prompt | variável (id + u16 length + payload; length é o tamanho tota |
| [0x9E](s2c-0x9e.md) | Sell List | variável (id + u16 length + payload; length é o tamanho tota |
| [0xA1](s2c-0xa1.md) | Update Current HP | 9 bytes |
| [0xA2](s2c-0xa2.md) | Update Current Mana | 9 bytes |
| [0xA3](s2c-0xa3.md) | Update Current Stamina | 9 bytes |
| [0xA5](s2c-0xa5.md) | Open Web Browser | variável (id + u16 length + payload; length é o tamanho tota |
| [0xA6](s2c-0xa6.md) | Tip/Notice | variável (id + u16 length + payload; length é o tamanho tota |
| [0xA8](s2c-0xa8.md) | Game Server List | variável (id + u16 length + payload; length é o tamanho tota |
| [0xA9](s2c-0xa9.md) | Characters + Starting Locations | variável (id + u16 length + payload; length é o tamanho tota |
| [0xAA](s2c-0xaa.md) | Attack Target (current) | 5 bytes |
| [0xAB](s2c-0xab.md) | Gump Text Entry Dialog | variável (id + u16 length + payload; length é o tamanho tota |
| [0xAE](s2c-0xae.md) | Unicode Speech | variável (id + u16 length + payload; length é o tamanho tota |
| [0xAF](s2c-0xaf.md) | Display Death Action | 13 bytes |
| [0xB0](s2c-0xb0.md) | Gump Menu Dialog (gump) | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB2](s2c-0xb2.md) | Chat Message | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB3](s2c-0xb3.md) | Chat Text | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB4](s2c-0xb4.md) | Target Object List | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB5](s2c-0xb5.md) | Open Chat Window | 64 bytes |
| [0xB7](s2c-0xb7.md) | Help/Tip Data | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB8](s2c-0xb8.md) | Character Profile | variável (id + u16 length + payload; length é o tamanho tota |
| [0xB9](s2c-0xb9.md) | Enable Locked Client Features | 5 bytes (clientes 6 |
| [0xBA](s2c-0xba.md) | Quest Arrow | 10 bytes (HS) | 6 bytes (anterior) |
| [0xBB](s2c-0xbb.md) | Mensagem/ID de conta | fixo 9 bytes total |
| [0xBC](s2c-0xbc.md) | Seasonal Info | fixo 3 bytes total |
| [0xBD](s2c-0xbd.md) | Client Version request | variável (id + u16 length + payload; length é o tamanho tota |
| [0xBE](s2c-0xbe.md) | Assist Version | variável (id + u16 length + payload; length é o tamanho tota |
| [0xC0](s2c-0xc0.md) | Hued Effect | 36 bytes |
| [0xC1](s2c-0xc1.md) | Localized Message (cliloc) | variável (id + u16 length + payload; length é o tamanho tota |
| [0xC2](s2c-0xc2.md) | Unicode Prompt | variável (id + u16 length + payload; length é o tamanho tota |
| [0xC3](s2c-0xc3.md) | GQ Request | variável (id + u16 length + payload; length é o tamanho tota |
| [0xC4](s2c-0xc4.md) | Semivisible | 6 bytes |
| [0xC6](s2c-0xc6.md) | Invalid Map Enable | 1 byte |
| [0xC7](s2c-0xc7.md) | 3D Particle Effect | 49 bytes |
| [0xC8](s2c-0xc8.md) | Update Range | 2 bytes |
| [0xC9](s2c-0xc9.md) | Trip Time | 6 bytes |
| [0xCA](s2c-0xca.md) | UTrip Time | 6 bytes |
| [0xCB](s2c-0xcb.md) | Global Queue Count | 7 bytes |
| [0xCC](s2c-0xcc.md) | Localized Message Affix | variável (id + u16 length + payload; length é o tamanho tota |
| [0xCD](s2c-0xcd.md) | Não documentado (1 byte) | 1 byte |
| [0xCE](s2c-0xce.md) | Não documentado | variável (id + u16 length + payload; length é o tamanho tota |
| [0xD0](s2c-0xd0.md) | Configuration File | variável (id + u16 length + payload; length é o tamanho tota |
| [0xD1](s2c-0xd1.md) | Logout Status | 2 bytes |
| [0xD2](s2c-0xd2.md) | Extended 0x20 (Mobile Update) | 25 bytes |
| [0xD3](s2c-0xd3.md) | Extended 0x78 (Mobile Incoming) | variável (id + u16 length + payload; length é o tamanho tota |
| [0xD4](s2c-0xd4.md) | Book Header (novo) | variável (id + u16 length + payload; length é o tamanho tota |
| [0xD6](s2c-0xd6.md) | Mega Cliloc (lista de propriedades do objeto) | variável |
| [0xD8](s2c-0xd8.md) | Custom House | variável (id + u16 length + payload; length é o tamanho tota |
| [0xDA](s2c-0xda.md) | Mahjong (game packets) | variável (id + u16 length + payload; length é o tamanho tota |
| [0xDB](s2c-0xdb.md) | Character Transfer Log | variável (id + u16 length + payload; length é o tamanho tota |
| [0xDC](s2c-0xdc.md) | SE Introduced Revision | fixo 9 bytes total |
| [0xDD](s2c-0xdd.md) | Compressed Gump | variável |
| [0xDE](s2c-0xde.md) | Update Mobile Status | variável (id + u16 length + payload; length é o tamanho tota |
| [0xDF](s2c-0xdf.md) | Buff/Debuff | variável (id + u16 length + payload; length é o tamanho tota |
| [0xE2](s2c-0xe2.md) | New Character Animation | 10 bytes |
| [0xE5](s2c-0xe5.md) | Display Waypoint | variável (id + u16 length + payload; length é o tamanho tota |
| [0xE6](s2c-0xe6.md) | Remove Waypoint | 5 bytes |
| [0xE7](s2c-0xe7.md) | Highlight UI Continue | 12 bytes |
| [0xE9](s2c-0xe9.md) | Highlight UI Toggle | 75 bytes |
| [0xEA](s2c-0xea.md) | Toggle Hotbar | 3 bytes |
| [0xF0](s2c-0xf0.md) | Movimento em lote e pacote especial Krrios | variável |
| [0xF2](s2c-0xf2.md) | Time Sync Response | 25 bytes |
| [0xF3](s2c-0xf3.md) | Object Info (SA/novo) | 26 bytes (clientes HS) | 24 bytes (SA, sem o campo final) |
| [0xF5](s2c-0xf5.md) | New Map Message | 21 bytes |
| [0xF6](s2c-0xf6.md) | Boat Moving | variável (id + u16 length + payload; length é o tamanho tota |
| [0xF7](s2c-0xf7.md) | Packet List (multi-pacote) | variável (id + u16 length + payload; length é o tamanho tota |

## Pacotes cliente -> servidor

| Pacote | Nome | Tamanho |
|---|---|---|
| [0x00](c2s-0x00.md) | Create Character | 104 bytes (fixo) |
| [0x01](c2s-0x01.md) | Disconnect / Logout | 5 bytes (fixo) |
| [0x02](c2s-0x02.md) | Move Request | 7 bytes (fixo) |
| [0x03](c2s-0x03.md) | Talk (ASCII) | variável (id + u16 length + payload) |
| [0x04](c2s-0x04.md) | GodModeRequest | 2 bytes (fixo) |
| [0x05](c2s-0x05.md) | Attack Request | 5 bytes (fixo) |
| [0x06](c2s-0x06.md) | Double Click | 5 bytes (fixo) |
| [0x07](c2s-0x07.md) | Pick Up Item | 7 bytes (fixo) |
| [0x08](c2s-0x08.md) | Drop Item | 14 bytes (clientes antes de 6 |
| [0x09](c2s-0x09.md) | Single Click | 5 bytes (fixo) |
| [0x0A](c2s-0x0a.md) | Edit | 11 bytes (fixo) |
| [0x12](c2s-0x12.md) | Use Skill / Cast / Action | variável (id + u16 length + payload) |
| [0x13](c2s-0x13.md) | Equip Item Request | 10 bytes (fixo) |
| [0x14](c2s-0x14.md) | ChangeZ | 6 bytes (fixo) |
| [0x22](c2s-0x22.md) | Resynchronize | 3 bytes (fixo) |
| [0x2C](c2s-0x2c.md) | DeathStatusResponse | 2 bytes (fixo) |
| [0x34](c2s-0x34.md) | Get Player Status | 10 bytes (fixo) |
| [0x3A](c2s-0x3a.md) | Set Skill Lock | variável (id + u16 length + payload) |
| [0x3B](c2s-0x3b.md) | VendorBuyReply | variável (id + u16 length + payload) |
| [0x47](c2s-0x47.md) | NewTerrain | 11 bytes (fixo) |
| [0x48](c2s-0x48.md) | NewAnimData | 73 bytes (fixo) |
| [0x4B](c2s-0x4b.md) | Check Client Version | 9 bytes (fixo) |
| [0x52](c2s-0x52.md) | Board Post Message | variável (id + u16 length + payload) |
| [0x58](c2s-0x58.md) | NewRegion | 106 bytes (fixo) |
| [0x5D](c2s-0x5d.md) | Play Character | 73 bytes (fixo) |
| [0x61](c2s-0x61.md) | DeleteStatic | 9 bytes (fixo) |
| [0x6B](c2s-0x6b.md) | Não documentado | 9 bytes (fixo) |
| [0x6C](c2s-0x6c.md) | Target Response | 19 bytes (fixo) |
| [0x6F](c2s-0x6f.md) | SecureTrade | variável (id + u16 length + payload) |
| [0x72](c2s-0x72.md) | Request War Mode | 5 bytes (fixo) |
| [0x73](c2s-0x73.md) | Ping | 2 bytes (fixo) |
| [0x75](c2s-0x75.md) | RenameRequest | 35 bytes (fixo) |
| [0x79](c2s-0x79.md) | ResourceQuery | 9 bytes (fixo) |
| [0x7D](c2s-0x7d.md) | MenuResponse | 13 bytes (fixo) |
| [0x7E](c2s-0x7e.md) | GodviewQuery | 2 bytes (fixo) |
| [0x80](c2s-0x80.md) | Login Request | 62 bytes (fixo) |
| [0x83](c2s-0x83.md) | DeleteCharacter | 39 bytes (fixo) |
| [0x84](c2s-0x84.md) | Change Password | 69 bytes (fixo) |
| [0x8D](c2s-0x8d.md) | ECCreateCharacter | variável (id + u16 length + payload); 146 bytes em clientes  |
| [0x91](c2s-0x91.md) | Game Server Login | 65 bytes (fixo) |
| [0x95](c2s-0x95.md) | HuePickerResponse | 9 bytes (fixo) |
| [0x96](c2s-0x96.md) | GameCentralMoniter | variável (id + u16 length + payload) |
| [0x98](c2s-0x98.md) | MobileNameRequest | variável (id + u16 length + payload) |
| [0x9A](c2s-0x9a.md) | AsciiPromptResponse | variável (id + u16 length + payload) |
| [0x9B](c2s-0x9b.md) | Help Request | 258 bytes (fixo) |
| [0x9D](c2s-0x9d.md) | GMSingle | 51 bytes (fixo) |
| [0x9F](c2s-0x9f.md) | Sell Reply | variável (id + u16 length + payload) |
| [0xA0](c2s-0xa0.md) | Select Server | 3 bytes (fixo) |
| [0xA4](c2s-0xa4.md) | System Info | 149 bytes (fixo) |
| [0xA7](c2s-0xa7.md) | Request Tip | 4 bytes (fixo) |
| [0xAC](c2s-0xac.md) | Gump Text Entry Dialog Reply | variável (id + u16 length + payload) |
| [0xAD](c2s-0xad.md) | Unicode Speech Request | variável (id + u16 length + payload) |
| [0xB1](c2s-0xb1.md) | Gump Menu Response | variável (id + u16 length + payload) |
| [0xB6](c2s-0xb6.md) | Object Help Request | 9 bytes (fixo) |
| [0xB8](c2s-0xb8.md) | ProfileReq | variável (id + u16 length + payload) |
| [0xBB](c2s-0xbb.md) | AccountID | 9 bytes (fixo) |
| [0xBD](c2s-0xbd.md) | Client Version | variável (id + u16 length + payload) |
| [0xBE](c2s-0xbe.md) | AssistVersion | variável (id + u16 length + payload) |
| [0xC2](c2s-0xc2.md) | UnicodePromptResponse | variável (id + u16 length + payload) |
| [0xC8](c2s-0xc8.md) | SetUpdateRange | 2 bytes (fixo) |
| [0xC9](c2s-0xc9.md) | TripTime | 6 bytes (fixo) |
| [0xCA](c2s-0xca.md) | UTripTime | 6 bytes (fixo) |
| [0xCF](c2s-0xcf.md) | AccountLogin | 78 bytes (fixo) |
| [0xD0](c2s-0xd0.md) | ConfigurationFile | variável (id + u16 length + payload) |
| [0xD1](c2s-0xd1.md) | LogoutReq | 2 bytes (fixo) |
| [0xD6](c2s-0xd6.md) | BatchQueryProperties | variável (id + u16 length + payload) |
| [0xD9](c2s-0xd9.md) | Spy on Client (hardware) | 268 bytes (fixo) |
| [0xE1](c2s-0xe1.md) | ClientType | variável (id + u16 length + payload) |
| [0xEC](c2s-0xec.md) | EquipMacro | variável (id + u16 length + payload) |
| [0xED](c2s-0xed.md) | UnequipMacro | variável (id + u16 length + payload) |
| [0xEF](c2s-0xef.md) | Login Seed | 21 bytes (fixo) |
| [0xF0](c2s-0xf0.md) | Movement Request (lote) | variável (id + u16 length + payload) |
| [0xF1](c2s-0xf1.md) | Time Sync Request | 9 bytes (fixo) |
| [0xF4](c2s-0xf4.md) | CrashReport | variável (id + u16 length + payload) |
| [0xF8](c2s-0xf8.md) | CreateCharacter70160 | 106 bytes (fixo) |
| [0xFA](c2s-0xfa.md) | Unhandled (UO Store) | 1 byte (fixo) |
| [0xFB](c2s-0xfb.md) | PublicHouseContent | 2 bytes (fixo) |

## `0xBF` General Information (subcomandos)

Enquadramento e despacho: [bf-framing.md](bf-framing.md).

| Pacote | Nome | Tamanho |
|---|---|---|
| [0xBF / 0x01](bf-0x01.md) | Fastwalk: inicializar pilha de chaves | fixo 29 bytes total |
| [0xBF / 0x02](bf-0x02.md) | Fastwalk: adicionar chave | fixo 9 bytes total |
| [0xBF / 0x04](bf-0x04.md) | Fechar gump genérico | fixo 13 bytes total |
| [0xBF / 0x05](bf-0x05.md) | Tamanho da tela | fixo 13 bytes total |
| [0xBF / 0x06](bf-0x06.md) | Comandos de party | variável (depende do comando) |
| [0xBF / 0x07](bf-0x07.md) | Clique na seta de quest | fixo 6 bytes total |
| [0xBF / 0x08](bf-0x08.md) | Mudar mapa | fixo 6 bytes total |
| [0xBF / 0x09](bf-0x09.md) | Desarmar (wrestling) | fixo 5 bytes total |
| [0xBF / 0x0A](bf-0x0a.md) | Atordoar (wrestling) | fixo 5 bytes total |
| [0xBF / 0x0B](bf-0x0b.md) | Idioma do cliente | fixo 9 bytes total |
| [0xBF / 0x0C](bf-0x0c.md) | Fechar barra de status | fixo 9 bytes total |
| [0xBF / 0x0E](bf-0x0e.md) | Animação (macro) | fixo 9 bytes total |
| [0xBF / 0x0F](bf-0x0f.md) | Informação do cliente | fixo 10 bytes total |
| [0xBF / 0x10](bf-0x10.md) | Propriedades do objeto e info de equipamento | C->S fixo 9 bytes; S->C variável (mínimo 17) |
| [0xBF / 0x13](bf-0x13.md) | Requisitar menu de contexto | fixo 9 bytes total |
| [0xBF / 0x14](bf-0x14.md) | Exibir menu de contexto | variável (`12 + soma das entradas`) |
| [0xBF / 0x15](bf-0x15.md) | Resposta do menu de contexto | fixo 11 bytes total |
| [0xBF / 0x16](bf-0x16.md) | Fechar janela de interface | fixo 13 bytes total |
| [0xBF / 0x17](bf-0x17.md) | Tópico de ajuda (Codex of Wisdom) | fixo 11 bytes total |
| [0xBF / 0x18](bf-0x18.md) | Habilitar map patches | variável (`9 + 8 * mapCount`) |
| [0xBF / 0x19](bf-0x19.md) | Estatísticas estendidas | depende da variante (11, 12 ou 17 bytes) |
| [0xBF / 0x1A](bf-0x1a.md) | Alterar trava de atributo | fixo 7 bytes total |
| [0xBF / 0x1B](bf-0x1b.md) | Conteúdo do spellbook (novo) | fixo 23 bytes total |
| [0xBF / 0x1C](bf-0x1c.md) | Selecionar/conjurar feitiço | 9 ou 13 bytes total |
| [0xBF / 0x1D](bf-0x1d.md) | Revisão do design da casa | fixo 13 bytes total |
| [0xBF / 0x1E](bf-0x1e.md) | Pedir detalhes do design da casa | fixo 9 bytes total |
| [0xBF / 0x20](bf-0x20.md) | Iniciar/encerrar customização de casa | fixo 17 bytes total |
| [0xBF / 0x21](bf-0x21.md) | Limpar habilidade de arma | fixo 5 bytes total |
| [0xBF / 0x22](bf-0x22.md) | Dano (formato antigo) | fixo 11 bytes total |
| [0xBF / 0x24](bf-0x24.md) | Anti-cheat (desconhecido) | variável, não especificado |
| [0xBF / 0x25](bf-0x25.md) | Alternar ícone de feitiço/habilidade | fixo 8 bytes total |
| [0xBF / 0x26](bf-0x26.md) | Modo de velocidade | fixo 6 bytes total |
| [0xBF / 0x2A](bf-0x2a.md) | Troca de raça | S->C 7 bytes; C->S 5 ou 15 bytes |
| [0xBF / 0x2B](bf-0x2b.md) | Animação por serial parcial | fixo 9 bytes total |
| [0xBF / 0x2C](bf-0x2c.md) | Usar bandagem em alvo | fixo 13 bytes total |
| [0xBF / 0x2D](bf-0x2d.md) | Feitiço com alvo | fixo 11 bytes total |
| [0xBF / 0x2E](bf-0x2e.md) | Skill com alvo | fixo 11 bytes total |
| [0xBF / 0x30](bf-0x30.md) | Alvo por recurso (macro) | fixo 11 bytes total |
| [0xBF / 0x31](bf-0x31.md) | Temporizador de bandagem | fixo 15 bytes total |
| [0xBF / 0x32](bf-0x32.md) | Alternar voo (gargoyle) | fixo 11 bytes total |
| [0xBF / 0x33](bf-0x33.md) | Mover barco com o mouse | fixo 12 bytes total |

## `0xD7` Encoded Client Packet (subcomandos)

Enquadramento e despacho: [d7-framing.md](d7-framing.md).

| Pacote | Nome | Tamanho |
|---|---|---|
| [0xD7 / 0x02](d7-0x02.md) | Casa: backup | fixo 10 bytes total |
| [0xD7 / 0x03](d7-0x03.md) | Casa: restaurar | fixo 10 bytes total |
| [0xD7 / 0x04](d7-0x04.md) | Casa: commit | fixo 10 bytes total |
| [0xD7 / 0x05](d7-0x05.md) | Casa: remover item | fixo 30 bytes total |
| [0xD7 / 0x06](d7-0x06.md) | Casa: colocar item | fixo 25 bytes total |
| [0xD7 / 0x0A](d7-0x0a.md) | Casa: resposta/ação (desconhecido) | fixo 10 bytes total |
| [0xD7 / 0x0C](d7-0x0c.md) | Casa: sair do designer | fixo 10 bytes total |
| [0xD7 / 0x0D](d7-0x0d.md) | Casa: colocar escada (multi) | fixo 25 bytes total |
| [0xD7 / 0x0E](d7-0x0e.md) | Casa: sincronizar | fixo 10 bytes total |
| [0xD7 / 0x0F](d7-0x0f.md) | Casa: ação (desconhecido) | não especificado |
| [0xD7 / 0x10](d7-0x10.md) | Casa: limpar | fixo 10 bytes total |
| [0xD7 / 0x12](d7-0x12.md) | Casa: trocar de andar | fixo 15 bytes total |
| [0xD7 / 0x13](d7-0x13.md) | Casa: colocar telhado | fixo 30 bytes total |
| [0xD7 / 0x14](d7-0x14.md) | Casa: remover telhado | fixo 30 bytes total |
| [0xD7 / 0x19](d7-0x19.md) | Ativar habilidade especial | fixo 15 bytes total |
| [0xD7 / 0x1A](d7-0x1a.md) | Casa: reverter | fixo 10 bytes total |
| [0xD7 / 0x1E](d7-0x1e.md) | Equipar última arma | fixo 10 bytes total |
| [0xD7 / 0x28](d7-0x28.md) | Botão de guild | fixo 10 bytes total |
| [0xD7 / 0x32](d7-0x32.md) | Botão de quest | fixo 10 bytes total |
