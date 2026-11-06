# 0x8D - ECCreateCharacter

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload); 146 bytes em clientes Enhanced. Disponível: antes do login (seleção de personagem).

Finalidade: Cria personagem pelo Enhanced Client (KR/SA).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x8D |
| 1 | 2 | u16 | length | tamanho total (146) |
| 3 | 4 | u32 | pattern | padrão do cliente |
| 7 | 4 | u32 | slot | slot do personagem |
| 11 | 30 | char[30] | name | ASCII |
| 41 | 30 | char[30] | unknown | Não especificado |
| 71 | 1 | u8 | profession | profissão |
| 72 | 1 | u8 | cityIndex | cidade inicial |
| 73 | 1 | u8 | gender | 0 = masculino; diferente de 0 = feminino |
| 74 | 1 | u8 | race | raça (ver notas) |
| 75 | 1 | u8 | str | força |
| 76 | 1 | u8 | dex | destreza |
| 77 | 1 | u8 | int | inteligência |
| 78 | 2 | u16 | skinHue | cor da pele |
| 80 | 8 | u8[8] | reservado | zeros |
| 88 | 1 | u8 | skill1 | índice da habilidade 1 |
| 89 | 1 | u8 | skill1Value | valor |
| 90 | 1 | u8 | skill2 | habilidade 2 |
| 91 | 1 | u8 | skill2Value | valor |
| 92 | 1 | u8 | skill3 | habilidade 3 |
| 93 | 1 | u8 | skill3Value | valor |
| 94 | 1 | u8 | skill4 | habilidade 4 |
| 95 | 1 | u8 | skill4Value | valor |
| 96 | 26 | u8[26] | reservado | zeros |
| 122 | 2 | u16 | hairHue | cor do cabelo |
| 124 | 2 | u16 | hairId | estilo de cabelo |
| 126 | 6 | u8[6] | reservado | Não especificado |
| 132 | 2 | u16 | shirtHue | cor da camisa |
| 134 | 2 | u16 | shirtId | id da camisa |
| 136 | 1 | u8 | reservado | Não especificado |
| 137 | 2 | u16 | faceHue | cor do rosto |
| 139 | 2 | u16 | faceId | id do rosto |
| 141 | 1 | u8 | reservado | Não especificado |
| 142 | 2 | u16 | beardHue | cor da barba |
| 144 | 2 | u16 | beardId | estilo de barba |

Notas:

`race`: o ServUO usa (race - 1) como índice de raça (clientes SA enviam um valor acima do KR); o Source-X subtrai 1 somente para clientes KR. Os valores de índice resultantes seguem a numeração de raças do servidor (0 humano, 1 elfo, 2 gárgula).
Quando uma profissão é escolhida, o pacote não carrega atributos e habilidades coerentes; o Source-X deriva stats e habilidades da profissão.
O ServUO registra o id com tamanho variável; o Source-X espera 146 bytes. O ModernUO não registra este id.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ECCreateCharacter`)
- Source-X: `src/network/receive.cpp` (`PacketCreateNew`)
