# 0x6F - Secure Trading

Direction: server -> client. Size: variable (id + u16 length + payload).

Purpose: Manages the secure trade window (open, close, accept, update gold).

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6F |
| 1 | 2 | u16 | length | 47 for display; 17 for the others (ModernUO, Source-X). |
| 3 | 1 | u8 | action | Action (table). |
| 4 | 4 | u32 | serial | Display: the other party's mobile. Others: trade container. |
| 8 | 4 | u32 | value1 | See variants. |
| 12 | 4 | u32 | value2 | See variants. |
| 16 | 1 | bool | hasName | See variants. |
| 17 | 30 | ASCII[30] | name | Display only. |

Notes:

| action | Meaning | value1 / value2 / hasName |
|---|---|---|
| 0 | Open window | value1 = own container, value2 = other's container, hasName = 1, name = name of the other party |
| 1 | Close | value1 = 0, value2 = 0, hasName = 0 |
| 2 | Update acceptance | value1 = own acceptance, value2 = other's acceptance, hasName = 0 |
| 3 | Update other's gold and platinum | value1 = gold, value2 = platinum |
| 4 | Update own gold and platinum | value1 = gold, value2 = platinum |

- ServUO sends the close with only 8 bytes (id, length, action, serial), without value1, value2 and hasName; the client reads those fields only for the actions that use them.
- Items in the trade are sent with packet 0x25 (container = the mobile owning the side).
- Source-X uses the same layout.

## Implementation reference

- ServUO: `Server/Network/Packets.cs`: `DisplaySecureTrade`, `CloseSecureTrade`, `UpdateSecureTrade`, `TradeFlag`
- ModernUO: `Projects/Server/Network/Packets/OutgoingSecureTradePackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketTradeAction`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `SecureTrading`
