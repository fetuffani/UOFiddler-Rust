# 0xD2 - Extended 0x20 (Mobile Update)
Direção: servidor -> cliente. Tamanho: 25 bytes.
Finalidade: Variante estendida da atualização de mobile (0x20).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD2 |
| 1 | 16 | u8[16] | body | Mesmo prefixo do 0x77 conforme o ClassicUO: serial u32, graphic u16, x u16, y u16, z i8, direction u8, hue u16, flags u8, notoriety u8 |
| 17 | 8 | u8[8] | extra | Não especificado |

Notas: O ClassicUO trata o 0xD2 com o mesmo parser do 0x77; o Source-X apenas define o id. Os 8 bytes finais e a relação com o layout do 0x20 não estão especificados no código.

## Referência de implementação
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`UpdateCharacter`)
- Source-X: `src/network/send.cpp` (`sphereproto.h`: `XCMD_CharMove2`)
