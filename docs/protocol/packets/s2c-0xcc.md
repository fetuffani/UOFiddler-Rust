# 0xCC - Localized Message Affix
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Mensagem cliloc com texto adicional (prefixo ou sufixo).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCC |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do falante |
| 7 | 2 | u16 | graphic | Graphic |
| 9 | 1 | u8 | type | Tipo da mensagem |
| 10 | 2 | u16 | hue | Cor |
| 12 | 2 | u16 | font | Fonte |
| 14 | 4 | u32 | cliloc | Número do cliloc |
| 18 | 1 | u8 | affixType | Flags (tabela) |
| 19 | 30 | ascii[30] | name | Nome |
| 49 | N+1 | ascii | affix | Texto adicional terminado em NUL |
| ... | 2*M+2 | utf16be | arguments | Argumentos terminados por 0x0000 (big-endian) |

Notas: `affixType`: 0x00 acrescenta ao final, 0x01 coloca no início, 0x02 marca a mensagem como de sistema. Diferente do 0xC1, os argumentos são big-endian (o ServUO usa little-endian para clientes Enhanced).

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`MessageLocalizedAffix`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageLocalisedEx`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayClilocString`)
