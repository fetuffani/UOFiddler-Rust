# 0xF4 - CrashReport

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: antes do login e em jogo.

Finalidade: Relatório de falha do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF4 |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 1 | u8 | major | versão maior |
| 4 | 1 | u8 | minor | versão menor |
| 5 | 1 | u8 | revision | revisão |
| 6 | 1 | u8 | patch | patch |
| 7 | 2 | u16 | x | X do personagem |
| 9 | 2 | u16 | y | Y |
| 11 | 1 | i8 | z | Z |
| 12 | 1 | u8 | map | mapa |
| 13 | 32 | char[32] | account | conta |
| 45 | 32 | char[32] | character | personagem |
| 77 | 15 | char[15] | ip | IP |
| 92 | 4 | u32 | unknown | Não especificado |
| 96 | 4 | u32 | exception | código de exceção |
| 100 | 100 | char[100] | process | executável |
| 200 | 100 | char[100] | report | descrição |
| 300 | 1 | u8 | zero | 0x00 |
| 301 | 4 | u32 | offset | offset do erro |
| 305 | 1 | u8 | count | número de endereços (nem sempre presente) |
| 306 | 4 x count | u32[] | addresses | endereços da pilha |

Notas:

O Source-X lê até o offset do erro (campo em 301); o ServUO lê também a lista de endereços.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`CrashReport`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`CrashReport`)
- Source-X: `src/network/receive.cpp` (`PacketCrashReport`)
