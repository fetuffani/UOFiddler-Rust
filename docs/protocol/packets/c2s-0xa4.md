# 0xA4 - System Info

Direção: cliente -> servidor. Tamanho: 149 bytes (fixo). Disponível: antes do login e em jogo.

Finalidade: Informações de hardware (formato antigo).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xA4 |
| 1 | 1 | u8 | v1 | Não especificado |
| 2 | 2 | u16 | v2 | Não especificado |
| 4 | 1 | u8 | v3 | Não especificado |
| 5 | 32 | char[32] | s1 | ASCII |
| 37 | 32 | char[32] | s2 | ASCII |
| 69 | 32 | char[32] | s3 | ASCII |
| 101 | 32 | char[32] | s4 | ASCII |
| 133 | 2 | u16 | v4 | Não especificado |
| 135 | 2 | u16 | v5 | Não especificado |
| 137 | 4 | i32 | v6 | Não especificado |
| 141 | 4 | i32 | v7 | Não especificado |
| 145 | 4 | i32 | v8 | Não especificado |

Notas:

O significado dos campos não é especificado; os servidores descartam o pacote.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`SystemInfo`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingPlayerPackets.cs` (`SystemInfo`)
- Source-X: `src/network/receive.cpp` (`PacketSystemInfo`)
