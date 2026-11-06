# 0xD0 - Configuration File
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Arquivo de configuração enviado ao cliente.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD0 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |

Notas: Não especificado: o ClassicUO tem handler vazio e nenhuma implementação de servidor o envia.

## Referência de implementação
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ConfigurationFileR`)
