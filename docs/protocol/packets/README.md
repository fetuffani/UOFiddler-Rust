# Pacotes servidor -> cliente (0x00 a 0x7F)

| ID | Arquivo | Nome | Tamanho |
|---|---|---|---|
| 0x0B | [s2c-0x0b.md](s2c-0x0b.md) | Damage | 7 bytes |
| 0x11 | [s2c-0x11.md](s2c-0x11.md) | Mobile Status (HP/stats) | variável (id + u16 length + payload) |
| 0x15 | [s2c-0x15.md](s2c-0x15.md) | Follow | 9 bytes |
| 0x16 | [s2c-0x16.md](s2c-0x16.md) | Healthbar update (SA/EC) | variável (id + u16 length + payload) |
| 0x17 | [s2c-0x17.md](s2c-0x17.md) | Health Bar Status Update | variável (id + u16 length + payload) |
| 0x1A | [s2c-0x1a.md](s2c-0x1a.md) | Object Info (item no chão) | variável (id + u16 length + payload) |
| 0x1B | [s2c-0x1b.md](s2c-0x1b.md) | Login Confirm (personagem entrou) | 37 bytes |
| 0x1C | [s2c-0x1c.md](s2c-0x1c.md) | Speech ASCII | variável (id + u16 length + payload) |
| 0x1D | [s2c-0x1d.md](s2c-0x1d.md) | Delete Object | 5 bytes |
| 0x20 | [s2c-0x20.md](s2c-0x20.md) | Draw Game Player (posição/direção própria) | 19 bytes |
| 0x21 | [s2c-0x21.md](s2c-0x21.md) | Move Reject | 8 bytes |
| 0x22 | [s2c-0x22.md](s2c-0x22.md) | Move Acknowledge | 3 bytes |
| 0x23 | [s2c-0x23.md](s2c-0x23.md) | Dragging Item (animação) | 26 bytes |
| 0x24 | [s2c-0x24.md](s2c-0x24.md) | Draw/Open Container | 7 ou 9 bytes (9 em clientes High Seas, 7.0.9.0+) |
| 0x25 | [s2c-0x25.md](s2c-0x25.md) | Add Item To Container | 21 bytes (20 bytes antes do cliente 6.0.1.7) |
| 0x27 | [s2c-0x27.md](s2c-0x27.md) | Reject Move Item | 2 bytes |
| 0x28 | [s2c-0x28.md](s2c-0x28.md) | Drop Item Failed / Clear Square | 5 bytes |
| 0x29 | [s2c-0x29.md](s2c-0x29.md) | Drop Item Approved | 1 byte |
| 0x2C | [s2c-0x2c.md](s2c-0x2c.md) | Resurrection Menu / Death | 2 bytes |
| 0x2D | [s2c-0x2d.md](s2c-0x2d.md) | Mobile Attributes | 17 bytes |
| 0x2E | [s2c-0x2e.md](s2c-0x2e.md) | Equip/Worn Item | 15 bytes |
| 0x2F | [s2c-0x2f.md](s2c-0x2f.md) | Fight Occurring | 10 bytes |
| 0x30 | [s2c-0x30.md](s2c-0x30.md) | Attack OK | 5 bytes |
| 0x31 | [s2c-0x31.md](s2c-0x31.md) | Attack Ended / Pet Window | variável (id + u16 length + payload) |
| 0x33 | [s2c-0x33.md](s2c-0x33.md) | Pause/Resume Client | 2 bytes |
| 0x38 | [s2c-0x38.md](s2c-0x38.md) | Pathfinding | 7 bytes |
| 0x39 | [s2c-0x39.md](s2c-0x39.md) | Remove (Group) | 9 bytes |
| 0x3A | [s2c-0x3a.md](s2c-0x3a.md) | Skills List/Update | variável (id + u16 length + payload) |
| 0x3B | [s2c-0x3b.md](s2c-0x3b.md) | End Vendor Buy | variável (id + u16 length + payload) |
| 0x3C | [s2c-0x3c.md](s2c-0x3c.md) | Add Multiple Items To Container (conteúdo da bag) | variável (id + u16 length + payload) |
| 0x3E | [s2c-0x3e.md](s2c-0x3e.md) | Versions (God client) | 37 bytes |
| 0x3F | [s2c-0x3f.md](s2c-0x3f.md) | Update Statics (God client; UltimaLive command) | variável (id + u16 length + payload) |
| 0x40 | [s2c-0x40.md](s2c-0x40.md) | Update Terrain (God client; UltimaLive) | 201 bytes |
| 0x41 | [s2c-0x41.md](s2c-0x41.md) | Update Tiledata (God client) | variável (id + u16 length + payload) |
| 0x42 | [s2c-0x42.md](s2c-0x42.md) | Update Art (God client) | variável (id + u16 length + payload) |
| 0x43 | [s2c-0x43.md](s2c-0x43.md) | Update Animation (God client) | 553 bytes |
| 0x44 | [s2c-0x44.md](s2c-0x44.md) | Update Hues (God client) | 713 bytes |
| 0x45 | [s2c-0x45.md](s2c-0x45.md) | Version OK | 5 bytes |
| 0x4E | [s2c-0x4e.md](s2c-0x4e.md) | Personal Light Level | 6 bytes |
| 0x4F | [s2c-0x4f.md](s2c-0x4f.md) | Overall Light Level | 2 bytes |
| 0x53 | [s2c-0x53.md](s2c-0x53.md) | Reject Character Logon / Popup message | 2 bytes |
| 0x54 | [s2c-0x54.md](s2c-0x54.md) | Play Sound Effect | 12 bytes |
| 0x55 | [s2c-0x55.md](s2c-0x55.md) | Login Complete | 1 byte |
| 0x56 | [s2c-0x56.md](s2c-0x56.md) | Map Packet (cartography/treasure) | 11 bytes |
| 0x57 | [s2c-0x57.md](s2c-0x57.md) | Update Regions | 110 bytes |
| 0x5B | [s2c-0x5b.md](s2c-0x5b.md) | Time | 4 bytes |
| 0x65 | [s2c-0x65.md](s2c-0x65.md) | Set Weather | 4 bytes |
| 0x66 | [s2c-0x66.md](s2c-0x66.md) | Book Pages | variável (id + u16 length + payload) |
| 0x69 | [s2c-0x69.md](s2c-0x69.md) | Change Text/Emote Colors (Options) | variável (id + u16 length + payload) |
| 0x6A | [s2c-0x6a.md](s2c-0x6a.md) | Não documentado | 3 bytes |
| 0x6C | [s2c-0x6c.md](s2c-0x6c.md) | Target Cursor | 19 bytes |
| 0x6D | [s2c-0x6d.md](s2c-0x6d.md) | Play Music (MIDI) | 3 bytes |
| 0x6E | [s2c-0x6e.md](s2c-0x6e.md) | Character Animation | 14 bytes |
| 0x6F | [s2c-0x6f.md](s2c-0x6f.md) | Secure Trading | variável (id + u16 length + payload) |
| 0x70 | [s2c-0x70.md](s2c-0x70.md) | Graphical Effect | 28 bytes |
| 0x71 | [s2c-0x71.md](s2c-0x71.md) | Bulletin Board | variável (id + u16 length + payload) |
| 0x72 | [s2c-0x72.md](s2c-0x72.md) | War Mode | 5 bytes |
| 0x73 | [s2c-0x73.md](s2c-0x73.md) | Ping | 2 bytes |
| 0x74 | [s2c-0x74.md](s2c-0x74.md) | Open Buy Window (lista da loja) | variável (id + u16 length + payload) |
| 0x76 | [s2c-0x76.md](s2c-0x76.md) | New Subserver | 16 bytes |
| 0x77 | [s2c-0x77.md](s2c-0x77.md) | Update Mobile (move/animação de outro) | 17 bytes |
| 0x78 | [s2c-0x78.md](s2c-0x78.md) | Mobile Incoming (aparece mobile + equip) | variável (id + u16 length + payload) |
| 0x7B | [s2c-0x7b.md](s2c-0x7b.md) | Sequence | 2 bytes |
| 0x7C | [s2c-0x7c.md](s2c-0x7c.md) | Open Dialog Box (menu) | variável (id + u16 length + payload) |
