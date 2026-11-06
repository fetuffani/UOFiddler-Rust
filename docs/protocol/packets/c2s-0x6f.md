# 0x6F - SecureTrade

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Ações na janela de troca segura: cancelar, aceitar, atualizar ouro.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6F |
| 1 | 2 | u16 | length | 8, 12 ou 16 |
| 3 | 1 | u8 | action | 1 = cancelar; 2 = marcar/desmarcar aceite; 3 = atualizar ouro |
| 4 | 4 | u32 | container | serial do container da troca |

Notas:

Campos adicionais conforme action:

| action | Offset | Tamanho | Tipo | Campo |
|---|---|---|---|---|
| 1 | nenhum | | | length = 8 |
| 2 | 8 | 4 | u32 | accepted (0 ou 1); length = 12 |
| 3 | 8 | 4 | u32 | gold; length = 16 |
| 3 | 12 | 4 | u32 | platinum |

O Source-X também define action 0 (abrir) e 4 (ledger) na enumeração, que o cliente não envia.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`SecureTrade`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMobilePackets.cs` (`SecureTrade`)
- Source-X: `src/network/receive.cpp` (`PacketSecureTradeReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TradeResponse`, `Send_TradeUpdateGold`)
