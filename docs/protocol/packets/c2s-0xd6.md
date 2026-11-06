# 0xD6 - BatchQueryProperties

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Pede as propriedades (tooltips) de vários objetos.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | 3 + 4 x n |
| 3 | 4 | u32 | serial | serial do objeto (repete-se n vezes) |

Notas:

O cliente de referência envia no máximo 15 serials por pacote. Resposta: 0xD6 Mega Cliloc para cada objeto visível.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`BatchQueryProperties`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingEntityPackets.cs` (`BatchQueryProperties`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MegaClilocRequest`)
