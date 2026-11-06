# 0xB8 - Character Profile
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Entrega o perfil (profile) de um personagem.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do personagem (0 se oculto) |
| 7 | N+1 | ascii | header | Nome/cabeçalho, terminado em NUL |
| ... | 2*M+2 | utf16be | footer | Título/rodapé, terminado por 0x0000 |
| ... | 2*K+2 | utf16be | body | Texto do perfil, terminado por 0x0000 |

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`DisplayProfile`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs`
- Source-X: `src/network/send.cpp` (`PacketProfile`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CharacterProfile`)
