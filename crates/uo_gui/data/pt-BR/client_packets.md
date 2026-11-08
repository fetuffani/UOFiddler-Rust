# Pacotes do client 7.0.10.3

Fonte: tabela de comprimentos em `.data` VA 0x6ce150 (12 bytes/entrada; bit 15 do 1º word = tamanho variável) e dispatcher de recebimento em 0x4813a9
(duas tabelas: bytes em 0x4822c0 e jump table em 0x482078). Nomes: deduzidos pelo protocolo conhecido do UO e cruzados com o tamanho (A=forte, B=provável, C=incerto).

## Servidor → cliente (tratados pelo cliente)

| ID | Tam | Handler | Nome | Conf |
|---|---|---|---|---|
| 0x0B | 7 | 0x481e8f | Damage | A |
| 0x11 | var | 0x4817c0 | Mobile Status (HP/stats) | A |
| 0x15 | 9 | 0x4817d5 | Follow | B |
| 0x16 | var | 0x481e65 | Healthbar update (SA/EC) [POL,ServUO] | A |
| 0x17 | var | 0x481e7a | Health Bar Status Update | B |
| 0x1A | var | 0x4813e5 | Object Info (item no chão) | A |
| 0x1B | 37 | 0x4814cc | Login Confirm (personagem entrou) | A |
| 0x1C | var | 0x4814a2 | Speech ASCII | A |
| 0x1D | 5 | 0x48154a | Delete Object | A |
| 0x20 | 19 | 0x481463 | Draw Game Player (posição/direção própria) | A |
| 0x21 | 8 | 0x481478 | Move Reject | A |
| 0x22 | 3 | 0x48148d | Move Acknowledge | A |
| 0x23 | 26 | 0x48155f | Dragging Item (animação) | A |
| 0x24 | 9 | 0x481574 | Draw/Open Container | A |
| 0x25 | 21 | 0x4815b3 | Add Item To Container | A |
| 0x27 | 2 | 0x4815dd | Reject Move Item | A |
| 0x28 | 5 | 0x4815f2 | Drop Item Failed / Clear Square [POL,Sphere] | A |
| 0x29 | 1 | 0x481607 | Drop Item Approved | B |
| 0x2C | 2 | 0x48161c | Resurrection Menu / Death | B |
| 0x2D | 17 | 0x481631 | Mobile Attributes | A |
| 0x2E | 15 | 0x481685 | Equip/Worn Item | A |
| 0x2F | 10 | 0x48169a | Fight Occurring | A |
| 0x30 | 5 | 0x4816af | Attack OK | A |
| 0x31 | var | 0x4816c4 | Attack Ended [POL] | A |
| 0x33 | 2 | 0x4813c5 | Pause/Resume Client | A |
| 0x38 | 7 | 0x4817ea | Pathfinding | A |
| 0x39 | 9 | 0x4818a7 | Remove (Group) [POL] | A |
| 0x3A | var | 0x4818bc | Skills List/Update | A |
| 0x3B | var | 0x4818d1 | End Vendor Buy [ServUO,POL] | A |
| 0x3C | var | 0x4815c8 | Add Multiple Items To Container (conteúdo da bag) | A |
| 0x3E | 37 | 0x48187d | Versions (God client) [POL] | A |
| 0x3F | var | 0x4817ff | Update Statics (God client; UltimaLive command) [POL,UltimaLive] | A |
| 0x40 | 201 | 0x481814 | Update Terrain (God client; UltimaLive) [Kairy,UltimaLive] | A |
| 0x41 | var | 0x481868 | Update Tiledata (God client) [Kairy, nome por memoria] | B |
| 0x42 | var | 0x481829 | Update Art (God client) [Kairy, nome por memoria] | B |
| 0x43 | 553 | 0x48183e | Update Animation (God client) [Kairy, nome por memoria] | B |
| 0x44 | 713 | 0x481853 | Update Hues (God client) [Kairy, nome por memoria] | B |
| 0x45 | 5 | 0x481892 | Version OK [POL] | A |
| 0x4E | 6 | 0x4818e6 | Personal Light Level | A |
| 0x4F | 2 | 0x4818fb | Overall Light Level | A |
| 0x53 | 2 | 0x481910 | Reject Character Logon / Popup message [POL,ServUO] | A |
| 0x54 | 12 | 0x48194f | Play Sound Effect | A |
| 0x55 | 1 | 0x481925 | Login Complete | A |
| 0x56 | 11 | 0x48193a | Map Packet (cartography/treasure) [POL,Sphere] | A |
| 0x57 | 110 | 0x481964 | Update Regions [POL] | A |
| 0x5B | 4 | 0x4819a3 | Time | A |
| 0x65 | 4 | 0x4819b8 | Set Weather | A |
| 0x66 | var | 0x4819cd | Book Pages | A |
| 0x69 | var | 0x481a0c | Change Text/Emote Colors (Options) [POL,Sphere] | A |
| 0x6A | 3 | 0x481a21 | (sem fonte) cliente le 2 bytes e DESCARTA [analise do binario] | B |
| 0x6C | 19 | 0x481a36 | Target Cursor | A |
| 0x6D | 3 | 0x481a4b | Play Music (MIDI) | A |
| 0x6E | 14 | 0x481a60 | Character Animation | A |
| 0x6F | var | 0x481a8a | Secure Trading | A |
| 0x70 | 28 | 0x4816d9 | Graphical Effect | A |
| 0x71 | var | 0x481a9f | Bulletin Board | A |
| 0x72 | 5 | 0x481ab4 | War Mode | A |
| 0x73 | 2 | 0x481ac9 | Ping | A |
| 0x74 | var | 0x481ade | Open Buy Window (lista da loja) | A |
| 0x76 | 16 | 0x481535 | New Subserver | A |
| 0x77 | 17 | 0x4813fa | Update Mobile (move/animação de outro) | A |
| 0x78 | var | 0x48140f | Mobile Incoming (aparece mobile + equip) | A |
| 0x7B | 2 | 0x4816ee | Sequence [ServUO] | A |
| 0x7C | var | 0x481703 | Open Dialog Box (menu) | A |
| 0x81 | var | 0x4814e1 | Change Character | B |
| 0x82 | 2 | 0x481520 | Login Denied | A |
| 0x85 | 2 | 0x481b32 | Character Delete Result | A |
| 0x86 | var | 0x481b47 | Character List Update | A |
| 0x88 | 66 | 0x48172d | Open Paperdoll | A |
| 0x89 | var | 0x481b08 | Corpse Clothing | A |
| 0x8B | var | 0x481742 | Display Sign Gump [ServUO,Sphere] | A |
| 0x8C | 11 | 0x481b1d | Connect To Game Server | A |
| 0x90 | 19 | 0x481589 | Map Message [POL] | A |
| 0x92 | var | 0x481979 | (sem fonte) grava entrada em multi.idx/multi.mul: dword,dword,bytes [analise do binario] | B |
| 0x93 | 99 | 0x4819e2 | Book Header (old) [POL] | A |
| 0x94 | var | 0x48198e | (sem fonte) grava entrada em skills.idx/skills.mul: byte,dword,bytes [analise do binario] | B |
| 0x95 | 9 | 0x481718 | Dye Window | A |
| 0x97 | 2 | 0x481b5c | Move Player | B |
| 0x98 | var | 0x481757 | Mobile Name | A |
| 0x99 | 30 | 0x481b71 | Multi Placement | A |
| 0x9A | var | 0x481b9b | Console Entry Prompt [POL] | A |
| 0x9E | var | 0x481af3 | Sell List | A |
| 0xA1 | 9 | 0x481646 | Update Current HP | A |
| 0xA2 | 9 | 0x48165b | Update Current Mana | A |
| 0xA3 | 9 | 0x481670 | Update Current Stamina | A |
| 0xA5 | var | 0x481bef | Open Web Browser | A |
| 0xA6 | var | 0x481c04 | Tip/Notice | A |
| 0xA8 | var | 0x4814f6 | Game Server List | A |
| 0xA9 | var | 0x48150b | Characters + Starting Locations | A |
| 0xAA | 5 | 0x481c19 | Attack Target (current) | A |
| 0xAB | var | 0x48176c | Gump Text Entry Dialog [POL] | A |
| 0xAE | var | 0x4814b7 | Unicode Speech | A |
| 0xAF | 13 | 0x481781 | Display Death Action | A |
| 0xB0 | var | 0x481796 | Gump Menu Dialog (gump) | A |
| 0xB2 | var | 0x481c43 | Chat Message | B |
| 0xB3 | var | 0x481c58 | Chat Text [POL,Sphere] | A |
| 0xB4 | var | 0x481b86 | Target Object List [UltimaLive] | A |
| 0xB5 | 64 | 0x481c2e | Open Chat Window [POL] | A |
| 0xB7 | var | 0x481c6d | Help/Tip Data | B |
| 0xB8 | var | 0x481c82 | Character Profile | A |
| 0xB9 | 5 | 0x4817ab | Enable Locked Client Features | A |
| 0xBA | 10 | 0x481c97 | Quest Arrow | A |
| 0xBB | 9 | 0x481cac | Ultima Messenger [POL] | A |
| 0xBC | 3 | 0x481ceb | Seasonal Info | A |
| 0xBD | var | 0x481cc1 | Client Version request | A |
| 0xBE | var | 0x481cd6 | Assist Version | A |
| 0xBF | var | 0x4813b0 | General Information (subcomandos) | A |
| 0xC0 | 36 | 0x481d2a | Hued Effect | A |
| 0xC1 | var | 0x481d00 | Localized Message (cliloc) | A |
| 0xC2 | var | 0x481bb0 | Unicode Prompt | A |
| 0xC3 | var | 0x481bc5 | GQ Request? [ServUO] | B |
| 0xC4 | 6 | 0x48144e | Semivisible [POL] | A |
| 0xC6 | 1 | 0x481d3f | Invalid Map Enable | B |
| 0xC7 | 49 | 0x481d93 | 3D Particle Effect | A |
| 0xC8 | 2 | 0x481d54 | Update Range | A |
| 0xC9 | 6 | 0x481d69 | Trip Time [ServUO] | A |
| 0xCA | 6 | 0x481d7e | UTrip Time [ServUO] | A |
| 0xCB | 7 | 0x481bda | Global Queue Count [ServUO,POL] | A |
| 0xCC | var | 0x481d15 | Localized Message Affix | A |
| 0xCD | 1 | 0x481de7 | (sem fonte) handler vazio (ret) - cliente IGNORA [analise do binario] | B |
| 0xCE | var | 0x481dd2 | (sem fonte) dword,dword,16B,36B,dword,str,str -> 5 std::string; possivel ligacao com MyUO/web [hipotese] | C |
| 0xD0 | var | 0x481da8 | Configuration File [POL] | A |
| 0xD1 | 2 | 0x481dbd | Logout Status | A |
| 0xD2 | 25 | 0x481424 | Extended 0x20 (Mobile Update) [POL] | A |
| 0xD3 | var | 0x481439 | Extended 0x78 (Mobile Incoming) [POL] | A |
| 0xD4 | var | 0x4819f7 | Book Header (novo) | A |
| 0xD6 | var | 0x481dfc | Mega Cliloc (properties/tooltips) | A |
| 0xD8 | var | 0x481e11 | Custom House | B |
| 0xDA | var | 0x481e26 | Mahjong (game packets) [ServUO,Sphere] | A |
| 0xDB | var | 0x481e3b | Character Transfer Log [POL,Sphere] | A |
| 0xDC | 9 | 0x481e50 | SE Introduced Revision (hash de props) | A |
| 0xDD | var | 0x481ea4 | Compressed Gump | A |
| 0xDE | var | 0x481ece | Update Mobile Status [POL] | A |
| 0xDF | var | 0x481eb9 | Buff/Debuff | A |
| 0xE2 | 10 | 0x481a75 | New Character Animation | A |
| 0xE5 | var | 0x481ee3 | Display Waypoint [ServUO,Sphere] | A |
| 0xE6 | 5 | 0x481ef8 | Remove Waypoint [ServUO,Sphere] | A |
| 0xE7 | 12 | 0x481f0d | Highlight UI Continue [Sphere] | B |
| 0xE9 | 75 | 0x481f22 | Highlight UI Toggle [Sphere] | B |
| 0xEA | 3 | 0x481f37 | Toggle Hotbar [Sphere] | B |
| 0xF2 | 25 | 0x481f4c | Time Sync Response [Sphere] | B |
| 0xF3 | 26 | 0x481f61 | Object Info (SA/novo) | A |
| 0xF5 | 21 | 0x48159e | New Map Message | A |
| 0xF6 | var | 0x481f83 | Boat Moving | A |

## Cliente → servidor (ids na tabela de tamanhos, sem handler de recebimento)

| ID | Tam | Nome | Conf |
|---|---|---|---|
| 0x00 | 104 | Create Character | A |
| 0x01 | 5 | Disconnect / Logout | A |
| 0x02 | 7 | Move Request | A |
| 0x03 | var | Talk (ASCII) | A |
| 0x04 | 2 | GodModeRequest [ServUO] | A |
| 0x05 | 5 | Attack Request | A |
| 0x06 | 5 | Double Click (usar) | A |
| 0x07 | 7 | Pick Up Item | A |
| 0x08 | 15 | Drop Item | A |
| 0x09 | 5 | Single Click (look) | A |
| 0x0A | 11 | Edit [ServUO] | A |
| 0x12 | var | Use Skill / Cast / Action | A |
| 0x13 | 10 | Equip Item Request | A |
| 0x14 | 6 | ChangeZ [ServUO] | A |
| 0x19 | var | (sem fonte) var (construtora 0x470280 existe, nunca chamada) | C |
| 0x22 | 3 | Resynchronize [ServUO] | A |
| 0x2C | 2 | DeathStatusResponse [ServUO] | A |
| 0x34 | 10 | Get Player Status | A |
| 0x3A | var | Set Skill Lock | B |
| 0x3B | var | VendorBuyReply [ServUO] | A |
| 0x47 | 11 | NewTerrain [ServUO] | A |
| 0x48 | 73 | NewAnimData [ServUO] | A |
| 0x4B | 9 | Check Client Version: id+u32+u32 [POL] (construtora existe, nunca chamada neste cliente) | B |
| 0x52 | var | Board Post Message [POL] (construtora existe, nunca chamada) | B |
| 0x58 | 106 | NewRegion [ServUO] | A |
| 0x5D | 73 | Play Character (selecionar) | A |
| 0x61 | 9 | DeleteStatic [ServUO] | A |
| 0x6B | 9 | (sem fonte) id+u32+u32 (construtora existe, nunca chamada neste cliente) | C |
| 0x6C | 19 | Target Response | A |
| 0x6F | var | SecureTrade [ServUO] | A |
| 0x72 | 5 | Request War Mode | A |
| 0x73 | 2 | Ping | A |
| 0x75 | 35 | RenameRequest [ServUO] | A |
| 0x79 | 9 | ResourceQuery [ServUO] | A |
| 0x7D | 13 | MenuResponse [ServUO] | A |
| 0x7E | 2 | GodviewQuery [ServUO] | A |
| 0x80 | 62 | Login Request | A |
| 0x83 | 39 | DeleteCharacter [ServUO] | A |
| 0x84 | 69 | Change Password: id+char[30]+char[30]+dword+IPv4 local (gethostbyname) [POL/Sphere; layout do binario] | A |
| 0x8D | var | ECCreateCharacter [ServUO] | A |
| 0x91 | 65 | Game Server Login | A |
| 0x95 | 9 | HuePickerResponse [ServUO] | A |
| 0x96 | var | GameCentralMoniter [ServUO] | A |
| 0x98 | var | MobileNameRequest [ServUO] | A |
| 0x9A | var | AsciiPromptResponse [ServUO] | A |
| 0x9B | 258 | Help Request | A |
| 0x9D | 51 | GMSingle [ServUO] | A |
| 0x9F | var | Sell Reply | A |
| 0xA0 | 3 | Select Server | A |
| 0xA4 | 149 | System Info | A |
| 0xA7 | 4 | Request Tip | B |
| 0xAC | var | Gump Text Entry Dialog Reply [POL] | A |
| 0xAD | var | Unicode Speech Request | A |
| 0xB1 | var | Gump Menu Response | A |
| 0xB6 | 9 | Object Help Request [ServUO] | A |
| 0xB8 | var | ProfileReq [ServUO] | A |
| 0xBB | 9 | AccountID [ServUO] | A |
| 0xBD | var | Client Version | A |
| 0xBE | var | AssistVersion [ServUO] | A |
| 0xBF | var | General Information | A |
| 0xC2 | var | UnicodePromptResponse [ServUO] | A |
| 0xC8 | 2 | SetUpdateRange [ServUO] | A |
| 0xC9 | 6 | TripTime [ServUO] | A |
| 0xCA | 6 | UTripTime [ServUO] | A |
| 0xCF | 78 | AccountLogin [ServUO] | A |
| 0xD0 | var | ConfigurationFile [ServUO] | A |
| 0xD1 | 2 | LogoutReq [ServUO] | A |
| 0xD6 | var | BatchQueryProperties [ServUO] | A |
| 0xD7 | var | Encoded Command (AOS) | B |
| 0xD9 | 268 | Spy on Client / pesquisa de hardware (268B; throttle via HKLM\SOFTWARE\Origin Worlds Online\LastHarvestTime) [POL,Sphere] | A |
| 0xE1 | var | ClientType [ServUO] | A |
| 0xEC | var | EquipMacro [ServUO] | A |
| 0xED | var | UnequipMacro [ServUO] | A |
| 0xEF | 8192 | Login Seed (tratado fora da tabela) | B |
| 0xF0 | var | New Walk Request em lote (fila de movimento, timestamps) [Sphere; layout do binario] | B |
| 0xF1 | 9 | Time Sync Request (id+8B; par do 0xF2) [Sphere; layout do binario] | B |
| 0xF4 | var | CrashReport [ServUO] | A |
| 0xF8 | ? | CreateCharacter70160 [ServUO] | A |
| 0xFA | ? | Unhandled [ServUO] | A |
| 0xFB | ? | PublicHouseContent [ServUO] | A |
