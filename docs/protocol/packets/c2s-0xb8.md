# 0xB8 - ProfileReq

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Pede ou altera o perfil de um personagem.

**Pedido de perfil (length = 8)**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | 8 |
| 3 | 1 | u8 | mode | 0 = exibir |
| 4 | 4 | u32 | serial | serial do mobile |

**Edição de perfil**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB8 |
| 1 | 2 | u16 | length | 12 + 2 x textLength |
| 3 | 1 | u8 | mode | 1 = editar |
| 4 | 4 | u32 | serial | serial do mobile |
| 8 | 2 | u16 | command | o cliente de referência envia 0x0001 |
| 10 | 2 | u16 | textLength | número de caracteres |
| 12 | 2 x textLength | char16[] | text | UTF-16 big-endian |

Notas:

O ServUO rejeita textLength > 511. Resposta ao modo 0: 0xB8 Character Profile (servidor para cliente).

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ProfileReq`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMobilePackets.cs` (`ProfileReq`)
- Source-X: `src/network/receive.cpp` (`PacketProfileReq`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_ProfileRequest`, `Send_ProfileUpdate`)
