# 0xBB - Mensagem/ID de conta
Direção: ambas. Tamanho: fixo 9 bytes total.
Finalidade: id reservado; as implementações divergem sobre o uso.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBB |
| 1 | 4 | u32 | serial1 | Primeiro serial |
| 5 | 4 | u32 | serial2 | Segundo serial |

Notas: o Source-X lê dois serials (evento de mensagem de e-mail, "mail message"); o ModernUO aceita o pacote como "Account ID" antes do login; o ClassicUO registra um handler vazio (Ultima Messenger). O conteúdo além dos 9 bytes de tamanho não é especificado.


Referência de implementação:
- Source-X: `src/network/receive.cpp` (`PacketMailMessage`)
- ModernUO: `Projects/Server/Network/Packets/IncomingPackets.cs`
- ClassicUO: `PacketHandlers.cs` (`UltimaMessengerR`)
