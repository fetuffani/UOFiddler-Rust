# 0xD6 - Mega Cliloc (lista de propriedades do objeto)
Direção: ambas. Tamanho: variável.
Finalidade: S->C entrega a lista de propriedades (tooltip) de um objeto; C->S pede as propriedades de vários objetos de uma vez.

S->C:

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | Tamanho total |
| 3 | 2 | u16 | unknown | 0x0001 (o cliente ignora o pacote se for maior que 1) |
| 5 | 4 | u32 | serial | Objeto |
| 9 | 2 | u16 | reserved | 0x0000 |
| 11 | 4 | u32 | revision | Hash/revisão da lista (a mesma enviada no 0xDC, sem o bit 30) |
| 15 | ... | entry[] | entries | Entradas |
| fim | 4 | u32 | terminator | 0x00000000 |

Entrada:

| Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|
| 4 | u32 | cliloc | Número do cliloc (0 encerra a lista) |
| 2 | u16 | byteLength | Comprimento em bytes dos argumentos |
| byteLength | UTF-16LE | arguments | Argumentos, sem terminador nulo; vários argumentos são separados por tabulação |

Notas:
- O primeiro cliloc da lista é o nome do objeto (cabeçalho).
- ModernUO limita cada argumento a 504 caracteres (cliente 2D antigo corrompe memória acima de ~510).
- O hash é calculado pelo servidor sobre o corpo: no ServUO é um XOR de `cliloc & 0x3FFFFFF` e `(cliloc >> 26) & 0x3F` mais o hash do texto dos argumentos; no ModernUO é o xxHash3 do bloco de entradas mascarado com `0x3FFFFFF`. O cliente só compara o valor para igualdade.
- Linhas com cliloc sem tradução são descartadas pelo cliente.

C->S (consulta em lote):

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD6 |
| 1 | 2 | u16 | length | `3 + 4*n` |
| 3 | 4*n | u32[n] | serials | Objetos consultados (o ClassicUO envia no máximo 15 por pacote) |

Notas: o servidor exige `(length - 3) % 4 == 0`. O mesmo pedido para um único objeto pode ser feito com 0xBF/0x10 (formato antigo).

Referência de implementação:
- ServUO `Server/ObjectPropertyList.cs` (`ObjectPropertyList`), `Server/Network/PacketHandlers.cs` (`BatchQueryProperties`)
- ModernUO `Projects/Server/PropertyList/ObjectPropertyList.cs`
- Source-X `src/network/send.cpp` (`PacketPropertyList`)
- ClassicUO `PacketHandlers.cs` (`MegaCliloc`), `OutgoingPackets.cs` (`Send_MegaClilocRequest`)

## Exemplo

Propriedades do item 0x40001234 com revisão 0x0012ABCD e duas entradas: cliloc 1042971 com argumento `Sword` e cliloc 1075085 sem argumento (41 bytes):

```
D6 00 29 00 01 40 00 12 34 00 00 00 12 AB CD
00 0F EA 1B 00 0A 53 00 77 00 6F 00 72 00 64 00
00 10 67 8D 00 00
00 00 00 00
```

| Trecho | Significado |
|---|---|
| `D6 00 29` | id e length = 41 |
| `00 01` | unknown = 1 |
| `40 00 12 34` | serial |
| `00 00` | reservado |
| `00 12 AB CD` | revisão |
| `00 0F EA 1B` | cliloc 1042971 |
| `00 0A` | 10 bytes de argumentos |
| `53 00 77 00 6F 00 72 00 64 00` | "Sword" em UTF-16LE |
| `00 10 67 8D` `00 00` | cliloc 1075085 sem argumentos |
| `00 00 00 00` | terminador |

---
