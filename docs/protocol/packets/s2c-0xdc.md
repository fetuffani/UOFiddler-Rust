# 0xDC - SE Introduced Revision
Direção: servidor -> cliente. Tamanho: fixo 9 bytes total.
Finalidade: informa a revisão (hash) da lista de propriedades de um objeto, para que o cliente peça o 0xD6 só se a revisão mudou.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDC |
| 1 | 4 | u32 | serial | Objeto |
| 5 | 4 | u32 | revision | `0x40000000 + hash` (ServUO e ModernUO) |

Notas: não há campo de comprimento; o tamanho é fixo em 9. O cliente compara a revisão com a que tem em cache e, se diferir, pede o 0xD6.

Referência de implementação:
- ServUO `Server/ObjectPropertyList.cs` (`OPLInfo`)
- ModernUO `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateOPLInfo`)
- Source-X `send.cpp` (`PacketPropertyListVersion`)
- ClassicUO `PacketHandlers.cs` (`OPLInfo`)

## Exemplo

Revisão do objeto 0x00001234 com hash 0x0000002A (enviada como `0x40000000 + hash`):

```
DC 00 00 12 34 40 00 00 2A
```

`DC` id; `00 00 12 34` serial; `40 00 00 2A` revisão. Não há campo de comprimento (9 bytes fixos).
