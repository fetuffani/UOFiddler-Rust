# 0xAD - Unicode Speech Request

Direction: client -> server. Size: variable (id + u16 length + payload). Available: in game.

Purpose: Sends Unicode speech, with or without encoded speech keywords.

| Offset | Size | Type | Field | Description |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xAD |
| 1 | 2 | u16 | length | total size |
| 3 | 1 | u8 | type | speech type; bits 0xC0 set = with keywords |
| 4 | 2 | u16 | hue | color |
| 6 | 2 | u16 | font | font |
| 8 | 4 | char[4] | language | ASCII language code, for example "ENU" + NUL |
| 12 | n | variable | speech | see below |

Notes:

Without keywords (type & 0xC0 == 0): `speech` is UTF-16 big-endian text terminated by NUL (2 zero bytes).

With keywords (type & 0xC0 != 0): `speech` = keyword block + UTF-8 text terminated by NUL.

Keyword block: sequence of 12-bit fields, big-endian, packed continuously (the first field occupies the highest 12 bits of the first u16):
1. count (12 bits), with count <= 50;
2. count keyword ids (12 bits each);
3. padding bits (zeros) to complete the byte.
Block size = ceil(12 x (count + 1) / 8) bytes. Examples: count 0 = 2 bytes; count 1 = 3 bytes; count 2 = 5 bytes. The ids come from the speech keyword table in speech.mul.

Speech types (type, after removing the 0xC0 bits): 0 normal, 1 system, 2 emote, 6 label, 7 focus, 8 whisper, 9 yell, 10 spell, 13 guild, 14 alliance, 15 command.
ServUO discards empty text or text longer than 128 characters.

Implementation reference:

- ServUO: `Server/Network/PacketHandlers.cs` (`UnicodeSpeech`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingMessagePackets.cs` (`UnicodeSpeech`)
- Source-X: `src/network/receive.cpp` (`PacketSpeakReqUNICODE`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_UnicodeSpeechRequest`)
