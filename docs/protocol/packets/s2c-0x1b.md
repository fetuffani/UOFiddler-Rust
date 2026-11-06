# 0x1B - Login Confirm (personagem entrou)

Direção: servidor -> cliente. Tamanho: 37 bytes.

Finalidade: Confirma a entrada no mundo e informa serial, corpo, posição e tamanho do mapa do personagem.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x1B |
| 1 | 4 | u32 | serial | Serial do personagem. |
| 5 | 4 | u32 | zero | 0. |
| 9 | 2 | u16 | body | Corpo. |
| 11 | 2 | u16 | x | Posição X. |
| 13 | 2 | u16 | y | Posição Y. |
| 15 | 2 | u16 | z | Posição Z (o cliente lê como i8 a partir do valor). |
| 17 | 1 | u8 | direction | Direção; o cliente usa os 3 bits baixos. |
| 18 | 1 | u8 | zero | 0. |
| 19 | 4 | i32 | minusOne | 0xFFFFFFFF. |
| 23 | 4 | u32 | zero | 0 (ServUO escreve dois u16 0, ModernUO um u32 0; equivalente). |
| 27 | 2 | u16 | mapWidth | Largura do mapa (padrão 6144). |
| 29 | 2 | u16 | mapHeight | Altura do mapa (padrão 4096). |
| 31 | 6 | u8[6] | padding | Zeros. |

Notas:

Total: 37 bytes, igual ao valor da tabela de packets.md.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `LoginConfirm`
- ModernUO: `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendLoginConfirmation`)
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketPlayerStart`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `EnterWorld`
