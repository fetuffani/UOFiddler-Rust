# 0xAD - Unicode Speech Request

Direção: cliente -> servidor. Tamanho: variável (id + u16 length + payload). Disponível: em jogo.

Finalidade: Envia fala em Unicode, com ou sem palavras-chave de fala codificadas.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAD |
| 1 | 2 | u16 | length | tamanho total |
| 3 | 1 | u8 | type | tipo de fala; bits 0xC0 ligados = com palavras-chave |
| 4 | 2 | u16 | hue | cor |
| 6 | 2 | u16 | font | fonte |
| 8 | 4 | char[4] | language | código de idioma ASCII, por exemplo "ENU" + NUL |
| 12 | n | variável | speech | ver abaixo |

Notas:

Sem palavras-chave (type & 0xC0 == 0): `speech` é texto UTF-16 big-endian terminado por NUL (2 bytes de zero).

Com palavras-chave (type & 0xC0 != 0): `speech` = bloco de palavras-chave + texto UTF-8 terminado por NUL.

Bloco de palavras-chave: sequência de campos de 12 bits, em big-endian, empacotados continuamente (o primeiro campo ocupa os 12 bits mais altos do primeiro u16):
1. count (12 bits), com count <= 50;
2. count ids de palavra-chave (12 bits cada);
3. bits de preenchimento (zeros) até completar o byte.
Tamanho do bloco = ceil(12 x (count + 1) / 8) bytes. Exemplos: count 0 = 2 bytes; count 1 = 3 bytes; count 2 = 5 bytes. Os ids vêm da tabela de palavras-chave de speech.mul.

Tipos de fala (type, após remover os bits 0xC0): 0 normal, 1 sistema, 2 emote, 6 rótulo, 7 foco, 8 sussurro, 9 grito, 10 magia, 13 guilda, 14 aliança, 15 comando.
O ServUO descarta texto vazio ou com mais de 128 caracteres.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnicodeSpeech`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMessagePackets.cs` (`UnicodeSpeech`)
- Source-X: `src/network/receive.cpp` (`PacketSpeakReqUNICODE`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnicodeSpeechRequest`)
