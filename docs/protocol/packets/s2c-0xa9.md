# 0xA9 - Characters + Starting Locations
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Entrega a lista de personagens, as cidades iniciais e as flags de recursos do cliente.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA9 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 1 | u8 | charCount | Quantidade de slots de personagem (1, 5, 6 ou 7) |
| 4 | 60 x charCount | entry[] | characters | Slot: ASCII[30] nome + 30 bytes zerados; slot livre = 60 zeros |
| 4+60*charCount | 1 | u8 | cityCount | Quantidade de cidades iniciais |
| ... | cityCount x (63 ou 89) | city[] | cities | Formato depende da versão do cliente (abaixo) |
| ... | 4 | u32 | flags | Flags da lista de personagens (tabela abaixo) |
| ... | 2 | i16 | lastSlot | Somente clientes 7.0.13.0+ / Enhanced: slot do último personagem (ServUO envia -1) |

Cidade, cliente a partir de 7.0.13.0 (89 bytes):

| Offset | Tamanho | Tipo | Campo |
|---|---|---|---|
| 0 | 1 | u8 | index |
| 1 | 32 | ascii[32] | city |
| 33 | 32 | ascii[32] | building |
| 65 | 4 | i32 | x |
| 69 | 4 | i32 | y |
| 73 | 4 | i32 | z |
| 77 | 4 | i32 | mapId |
| 81 | 4 | u32 | cliloc da descrição |
| 85 | 4 | u32 | 0 |

Cidade, clientes anteriores (63 bytes): `u8 index`, `ascii[31] city`, `ascii[31] building`. O cliente antigo lê a posição e a descrição de arquivos locais (`citytext.enu`).

Flags (`CharacterListFlags`):

| Valor | Nome |
|---|---|
| 0x0001 | Unk1 |
| 0x0002 | OverwriteConfigButton |
| 0x0004 | OneCharacterSlot |
| 0x0008 | ContextMenus |
| 0x0010 | SlotLimit |
| 0x0020 | AOS |
| 0x0040 | SixthCharacterSlot |
| 0x0080 | SE |
| 0x0100 | ML |
| 0x0200 | Unk2 |
| 0x0400 | UO3DClientType |
| 0x0600 | KR (combinação 0x200 + 0x400) |
| 0x0800 | Unk3 |
| 0x1000 | SeventhCharacterSlot |
| 0x2000 | Unk4 |
| 0x4000 | NewMovementSystem |
| 0x8000 | NewFeluccaAreas |

Notas: O ClassicUO escolhe o formato de 89 bytes quando a versão do cliente é >= 7.0.13.0. O ServUO e o ModernUO usam `charCount = max(maior slot + 1, limite, 5)`. Com recursos de terceiros desativados, o ServUO sobrescreve bytes dos slots com uma máscara de features e um hash MD5 (extensão de assistentes, fora do protocolo base). O Source-X acrescenta `flags` e `lastSlot` somente quando a versão reportada é maior que 1.26.0. Tamanho total para o formato novo: `11 + 60*charCount + 89*cityCount` (a mais 2 se `lastSlot` for enviado, conforme a implementação).

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`CharacterList`, `CharacterListOld`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendCharacterList`)
- ServUO: `Server/ExpansionInfo.cs` (`CharacterListFlags`)
- Source-X: `src/network/send.cpp` (`PacketCharacterList`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ReceiveCharacterList`); ClassicUO: `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`ParseCharacterList`, `ParseCities`)
