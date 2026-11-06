# 0x6C - Target Cursor

Direção: servidor -> cliente. Tamanho: 19 bytes.

Finalidade: Solicita ao cliente que mostre um cursor de alvo (ou o cancela).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6C |
| 1 | 1 | u8 | targetType | 0 = objeto, 1 = posição no chão. |
| 2 | 4 | u32 | cursorId | Identificador devolvido na resposta. |
| 6 | 1 | u8 | cursorType | 0 = neutro, 1 = prejudicial, 2 = benéfico, 3 = cancelar. |
| 7 | 12 | u8[12] | padding | Zeros (x, y, z e graphic só são usados na resposta do cliente). |

Notas:

Para cancelar: targetType 0, cursorId 0 e cursorType 3.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `TargetReq`, `CancelTarget`
- ModernUO: `Projects/Server/Network/Packets/OutgoingTargetPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketAddTarget`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `TargetCursor`
