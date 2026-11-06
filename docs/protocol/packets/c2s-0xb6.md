# 0xB6 - Object Help Request

Direção: cliente -> servidor. Tamanho: 9 bytes (fixo). Disponível: em jogo.

Finalidade: Pede ajuda contextual sobre um objeto.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB6 |
| 1 | 4 | u32 | serial | serial do objeto |
| 5 | 1 | u8 | unknown | Não especificado |
| 6 | 3 | char[3] | language | código de idioma ASCII |

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`ObjectHelpRequest`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingEntityPackets.cs` (`ObjectHelpRequest`)
