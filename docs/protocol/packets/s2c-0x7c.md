# 0x7C - Open Dialog Box (menu)

Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload).

Finalidade: Abre um menu de seleção (lista de itens ou perguntas).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x7C |
| 1 | 2 | u16 | length | Tamanho total. |
| 3 | 4 | u32 | serial | Dono/origem do menu. |
| 7 | 2 | u16 | menuId | Identificador do menu. |
| 9 | 1 | u8 | questionLength | Tamanho do texto. |
| 10 | n | ASCII | question | Título, sem terminador. |
| + | 1 | u8 | count | Número de opções. |
| + | n | opção[] | options | `count` opções. |

**Opção**

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| +0 | 2 | u16 | graphic | ItemID; 0 em menus de texto. |
| +2 | 2 | u16 | hue | Cor. |
| +4 | 1 | u8 | textLength |  |
| +5 | n | ASCII | text | Sem terminador. |

Notas:

- Menu de itens: cada opção tem `graphic` != 0. Menu de perguntas (cinza): `graphic` = 0 e `hue` = 0 (ServUO escreve um u32 0); o cliente detecta o tipo lendo o `graphic` da primeira opção.
- O cliente numera as respostas a partir de 1.

## Referência de implementação

- ServUO: `Server/Network/Packets.cs`: `DisplayItemListMenu`, `DisplayQuestionMenu`
- ModernUO: `Projects/Server/Network/Packets/OutgoingMenuPackets.cs`
- Source-X: `src/network/send.cpp`, `src/network/send.h`: `PacketDisplayMenu`
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs`: `OpenMenu`
