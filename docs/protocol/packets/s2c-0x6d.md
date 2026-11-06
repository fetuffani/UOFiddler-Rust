# 0x6D - Play Music (MIDI)

Direção: servidor -> cliente. Tamanho: 3 bytes.

Finalidade: Toca uma música.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x6D |
| 1 | 2 | u16 | musicId | Índice da música. |

Notas:

O ModernUO para a música com os bytes `1F FF`; o ClassicUO interpreta esses dois bytes como comando de parada.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `PlayMusic`
- ModernUO: `Projects/Server/Network/Packets/OutgoingPlayerPackets.cs` (`SendPlayMusic`, `SendStopMusic`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketPlayMusic`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `PlayMusic`
