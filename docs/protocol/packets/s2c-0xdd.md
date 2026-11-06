# 0xDD - Compressed Gump
Direção: servidor -> cliente. Tamanho: variável.
Finalidade: abre um gump com layout e linhas de texto comprimidos com zlib.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xDD |
| 1 | 2 | u16 | length | Tamanho total |
| 3 | 4 | u32 | serial | Serial do dono do gump |
| 7 | 4 | u32 | gumpId | TypeID do gump |
| 11 | 4 | u32 | x | Posição X |
| 15 | 4 | u32 | y | Posição Y |
| 19 | 4 | u32 | layoutCompressedLen | Bytes comprimidos + 4 (o bloco de dados tem `valor - 4` bytes) |
| 23 | 4 | u32 | layoutRawLen | Tamanho do layout descomprimido |
| 27 | n | zlib | layoutData | Layout comprimido |
| ... | 4 | u32 | lineCount | Número de linhas de texto |
| ... | 4 | u32 | textCompressedLen | Bytes comprimidos + 4 (só presente se `lineCount > 0` no cliente) |
| ... | 4 | u32 | textRawLen | Tamanho do bloco de textos descomprimido |
| ... | m | zlib | textData | Linhas comprimidas |

Layout descomprimido: texto ASCII dos comandos de gump em sequência (`{ ... }{ ... }`), terminado por um byte 0x00 (ServUO e Source-X incluem o terminador no comprimento). A gramática dos comandos está em [gump-layout.md](../gump-layout.md).

Textos descomprimidos: sequência de `lineCount` entradas, cada uma `u16 chars` (número de caracteres) + `chars` caracteres UTF-16BE, sem terminador.

Notas:
- Com layout vazio, o ServUO/ModernUO escrevem apenas `u32 0` no lugar do par (comprimento, dados). Quando não há linhas, escrevem `lineCount = 0` seguido de `u32 0`.
- Se `lineCount = 0`, o ClassicUO não lê o par de comprimentos de texto.
- A compressão é zlib padrão (cabeçalho `78 9C`).

Referência de implementação:
- ServUO `Server/Network/Packets.cs` (`DisplayGumpPacked`)
- ModernUO `Projects/UOContent/Gumps/Base/DynamicGump.cs`, `StaticGump.cs`, `OutgoingGumpPackets.cs` (`WritePacked`)
- Source-X `send.cpp` (`PacketGumpDialog::writeCompressedControls`)
- ClassicUO `PacketHandlers.cs` (`OpenCompressedGump`)

## Exemplo

Gump do serial 0x40001234, TypeID 1, posição (100, 50), layout `{ page 0 }{ text 10 10 0 0 }` + NUL (29 bytes, 32 comprimidos) e uma linha de texto `Hi` (6 bytes, 14 comprimidos), 85 bytes no total:

```
DD 00 55
40 00 12 34 00 00 00 01 00 00 00 64 00 00 00 32
00 00 00 24 00 00 00 1D
78 9C AB 56 28 48 4C 4F 55 30 50 A8 AD 56 28 49 AD 28 51 30 34 00 21 10 AC 65 00 00 84 15 07 C5
00 00 00 01
00 00 00 12 00 00 00 06
78 9C 63 60 62 F0 60 C8 04 00 01 51 00 B4
```

| Trecho | Significado |
|---|---|
| `DD 00 55` | id e length = 85 |
| `40 00 12 34` | serial |
| `00 00 00 01` | gumpId |
| `00 00 00 64` / `00 00 00 32` | x = 100, y = 50 |
| `00 00 00 24` | compressed + 4 = 36 (32 bytes de zlib) |
| `00 00 00 1D` | layout bruto = 29 bytes |
| `78 9C ...` | zlib do layout |
| `00 00 00 01` | lineCount = 1 |
| `00 00 00 12` | compressed + 4 = 18 (14 bytes de zlib) |
| `00 00 00 06` | bloco bruto = 6 bytes (`00 02 00 48 00 69`: 2 caracteres, "H", "i") |
| `78 9C ...` | zlib do bloco de textos |
