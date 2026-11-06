# 0xB2 - Chat Message
Direção: servidor -> cliente. Tamanho: variável (id + u16 length + payload; length é o tamanho total do pacote).
Finalidade: Mensagem do sistema de chat (canal de chat antigo).

Convenções: offsets contados a partir do primeiro byte do pacote (o id); inteiros de mais de um byte em big-endian; u8/u16/u32 sem sinal, i8/i16/i32 com sinal.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xB2 |
| 1 | 2 | u16 | length | Tamanho total do pacote, incluindo id e este campo |
| 3 | 2 | u16 | command | Código do comando/mensagem (tabela) |
| 5 | 4 | ascii[4] | language | Idioma, ex. "ENU"; zeros se ausente |
| 9 | 2*N+2 | utf16be | param1 | Primeiro parâmetro, terminado por 0x0000 |
| ... | 2*M+2 | utf16be | param2 | Segundo parâmetro, terminado por 0x0000 |

Notas: Servidores enviam `command = número - 20`. Comandos tratados pelo ClassicUO:

| command | Significado |
|---|---|
| 0x03E8 | Criar conferência (param1 = nome do canal; u16 de senha 0x31 indica protegida) |
| 0x03E9 | Destruir conferência |
| 0x03EB | Pedir nome de usuário |
| 0x03EC | Fechar chat |
| 0x03ED | Nome de usuário aceito |
| 0x03EE | Adicionar usuário (tipo u16 + nome) |
| 0x03EF | Remover usuário |
| 0x03F0 | Limpar todos os jogadores |
| 0x03F1 | Entrou na conferência |
| 0x03F4 | Saiu da conferência |
| 0x0025 a 0x0027 | Mensagem de chat (u16 tipo, nome, texto) |
| 0x0001 a 0x0024, 0x0028 a 0x002C | Mensagens padrão com substituição de `%1` (e `%2`) por param1 |

## Referência de implementação
- ServUO: `Scripts/Services/Chat/Packets.cs` (`ChatMessagePacket`)
- ModernUO: `Projects/UOContent/Engines/Chat/ChatPackets.cs` (`SendChatMessage`)
- Source-X: `src/network/send.cpp` (`PacketChatMessage`)
- ClassicUO: `src/ClassicUO.Client/Network/PacketHandlers.cs` (`ChatMessage`)
