# 0x6F - Secure Trading

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Gerencia a janela de troca segura (abrir, fechar, aceitar, atualizar ouro).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6F |
| 1 | 2 | u16 | length | 47 para display; 17 para os demais (ModernUO, Source-X). |
| 3 | 1 | u8 | action | Ação (tabela). |
| 4 | 4 | u32 | serial | Display: mobile da outra parte. Demais: container de troca. |
| 8 | 4 | u32 | value1 | Ver variantes. |
| 12 | 4 | u32 | value2 | Ver variantes. |
| 16 | 1 | bool | hasName | Ver variantes. |
| 17 | 30 | ASCII[30] | name | Somente display. |

Notas:

| action | Significado | value1 / value2 / hasName |
|---|---|---|
| 0 | Abrir janela | value1 = container próprio, value2 = container do outro, hasName = 1, name = nome da outra parte |
| 1 | Fechar | value1 = 0, value2 = 0, hasName = 0 |
| 2 | Atualizar aceite | value1 = aceite próprio, value2 = aceite do outro, hasName = 0 |
| 3 | Atualizar ouro e platina do outro | value1 = ouro, value2 = platina |
| 4 | Atualizar ouro e platina próprios | value1 = ouro, value2 = platina |

- O ServUO envia o fechamento só com 8 bytes (id, length, action, serial), sem value1, value2 e hasName; o cliente lê esses campos apenas para as ações que os usam.
- Itens da troca são enviados com o pacote 0x25 (container = mobile dono do lado).
- O Source-X usa o mesmo layout.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `DisplaySecureTrade`, `CloseSecureTrade`, `UpdateSecureTrade`, `TradeFlag`
- ModernUO: `Projects/Server/Network/Packets/OutgoingSecureTradePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketTradeAction`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `SecureTrading`
