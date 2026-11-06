# 0xF8 - CreateCharacter70160

Direção: cliente -> servidor. Tamanho: 106 bytes (fixo). Disponível: antes do login (seleção de personagem).

Finalidade: Cria personagem (clientes 7.0.16.0 ou posteriores, 4 habilidades).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF8 |
| 1 | 4 | u32 | padrão | 0xEDEDEDED |
| 5 | 4 | u32 | padrão | 0xFFFFFFFF |
| 9 | 1 | u8 | reservado | 0x00 |
| 10 | 30 | char[30] | name | ASCII |
| 40 | 2 | u8[2] | reservado | zeros |
| 42 | 4 | u32 | flags | flags de protocolo do cliente |
| 46 | 8 | u8[8] | reservado | o cliente de referência envia 0x00000001 e 0x00000000 |
| 54 | 1 | u8 | profession | profissão |
| 55 | 15 | u8[15] | reservado | zeros |
| 70 | 1 | u8 | genderRace | 2/3 humano M/F; 4/5 elfo M/F; 6/7 gárgula M/F |
| 71 | 1 | u8 | str | força |
| 72 | 1 | u8 | dex | destreza |
| 73 | 1 | u8 | int | inteligência |
| 74 | 8 | 4 x (u8, u8) | skills | quatro pares (índice, valor) |
| 82 | 2 | u16 | skinHue | cor da pele |
| 84 | 2 | u16 | hairId | cabelo |
| 86 | 2 | u16 | hairHue | cor do cabelo |
| 88 | 2 | u16 | facialHairId | barba |
| 90 | 2 | u16 | facialHairHue | cor da barba |
| 92 | 1 | u8 | reservado | zero |
| 93 | 1 | u8 | cityIndex | cidade inicial |
| 94 | 4 | u32 | slot | slot do personagem |
| 98 | 4 | u32 | clientIp | IPv4 do cliente |
| 102 | 2 | u16 | shirtHue | cor da camisa |
| 104 | 2 | u16 | pantsHue | cor da calça |

Notas:

Mesmo layout do 0x00 com um quarto par de habilidade; todos os campos a partir do offset 82 ficam deslocados em 2 bytes. A tabela de pacotes não fixa tamanho para este id; os servidores registram 106 bytes.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`CreateCharacter70160`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`CreateCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCreateHS`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CreateCharacter`)
