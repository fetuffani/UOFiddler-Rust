# 0xDE - Update Mobile Status
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Informa o estado de combate de um mobile.

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDE |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 4 | u32 | serial | Serial do mobile |
| 7 | 1 | u8 | status | Código de status; 1 inclui o campo seguinte |
| 8 | 4 | u32 | attacker | Serial do atacante (somente se status = 1) |

Notas: Layout conforme o parser do ClassicUO; nenhum servidor de referência o envia.

## Referência de implementação
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateMobileStatus`)
