# 0xBC - Seasonal Info
Direção: servidor -> cliente. Tamanho: fixo 3 bytes total.
Finalidade: troca a estação do ano exibida pelo cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBC |
| 1 | 1 | u8 | season | 0 primavera, 1 verão, 2 outono, 3 inverno, 4 desolação (valores > 4 viram 0) |
| 2 | 1 | u8 | playSound | Toca a música da estação (1) ou não (0); o ClassicUO o trata como índice de música |


Referência de implementação:
- ServUO: `Server/Network/Packets.cs` (`SeasonChange`)
- Source-X: `src/network/send.cpp` (`PacketSeason`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`Season`)
