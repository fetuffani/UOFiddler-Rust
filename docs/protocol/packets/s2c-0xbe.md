# 0xBE - Assist Version
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Pede a versão do assistente (Assist) do cliente.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBE |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | version | Versão do assistente do servidor |

Notas: Tamanho total 7 bytes no ServUO. O ClassicUO tem o handler vazio.

## Referência de implementação
- ServUO: `Server/Network/Packets.cs` (`AssistVersionReq`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`AssistVersion`)
