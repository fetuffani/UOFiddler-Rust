# 0xF0 - Movement Request (lote)

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Envia vários passos de movimento de uma vez (clientes KR/SA com movimento novo).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xF0 |
| 1 | 2 | u16 | length | 4 + 34 x steps |
| 3 | 1 | u8 | steps | número de passos |

**Passo (34 bytes, repetido steps vezes, a partir do offset 4)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 8 | u64 | time1 | marca de tempo (início) |
| +8 | 8 | u64 | time2 | marca de tempo (fim) |
| +16 | 1 | u8 | sequence | sequência do passo |
| +17 | 1 | u8 | direction | direção (0-7) |
| +18 | 4 | u32 | mode | 1 = andar; 2 = correr |
| +22 | 4 | i32 | x | posição X |
| +26 | 4 | i32 | y | posição Y |
| +30 | 4 | i32 | z | posição Z |

Notas:

O servidor aplica cada passo em ordem, com as mesmas regras de sequência do 0x02; mode = 2 equivale ao bit 0x80 de direção. O Source-X só trata o pacote quando o movimento SA está habilitado e length coincide com o tamanho recebido. O ModernUO mantém o tratamento desativado, pois o id também é usado por extensões de assistentes.

Referência de implementação:

- Source-X: `src/network/receive.cpp` (`PacketMovementReqNew`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMovementPackets.cs` (`NewMovementReq`, desativado)
