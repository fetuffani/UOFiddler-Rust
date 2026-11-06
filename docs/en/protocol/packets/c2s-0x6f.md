# 0x6F - SecureTrade

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Actions in the secure trade window: cancel, accept, update gold.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6F |
| 1 | 2 | u16 | length | 8, 12 or 16 |
| 3 | 1 | u8 | action | 1 = cancel; 2 = mark/unmark accept; 3 = update gold |
| 4 | 4 | u32 | container | serial of the trade container |

Notes:

Additional fields depending on action:

| action | Offset | Size | Type | Field |
|---|---|---|---|---|
| 1 | none | | | length = 8 |
| 2 | 8 | 4 | u32 | accepted (0 or 1); length = 12 |
| 3 | 8 | 4 | u32 | gold; length = 16 |
| 3 | 12 | 4 | u32 | platinum |

Source-X also defines action 0 (open) and 4 (ledger) in the enumeration, which the client does not send.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`SecureTrade`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMobilePackets.cs` (`SecureTrade`)
- Source-X: `src/network/receive.cpp` (`PacketSecureTradeReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_TradeResponse`, `Send_TradeUpdateGold`)
