# 0xC1 - Localized Message (cliloc)
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Mensagem de um objeto com texto vindo do cliloc, com argumentos.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xC1 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do falante; 0xFFFFFFFF = sistema |
| 7 | 2 | u16 | graphic | Graphic do falante; 0xFFFF se nenhum |
| 9 | 1 | u8 | type | Tipo da mensagem (mesma tabela do 0xAE) |
| 10 | 2 | u16 | hue | Cor; ServUO usa 0x03B2 quando 0 |
| 12 | 2 | u16 | font | Fonte |
| 14 | 4 | u32 | cliloc | Número do cliloc |
| 18 | 30 | ascii[30] | name | Nome do falante |
| 48 | 2*N+2 | utf16le | arguments | Argumentos terminados por 0x0000 (UTF-16 little-endian); separados por tabulação ('\t') |

Notas: Atenção: os argumentos do 0xC1 são UTF-16 little-endian; no 0xCC são big-endian. Argumentos que começam com `#` referenciam outro cliloc. O ClassicUO lê todo o restante do pacote como argumentos.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`MessageLocalized`)
- ModernUO: `Projects/Server/Network/Packets/OutgoingMessagePackets.cs`
- Source-X: `src/network/send.cpp` (`PacketMessageLocalised`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`DisplayClilocString`)
