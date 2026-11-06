# 0xB7 - Help/Tip Data
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Envia o texto de ajuda de um objeto.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB7 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do objeto |
| 7 | 2*N+2 | utf16be | text | Texto, terminado por 0x0000 |

Notas: O ClassicUO registra handler vazio.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`ObjectHelpResponse`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketTooltip`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Help`)
