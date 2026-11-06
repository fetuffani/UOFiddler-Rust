# 0xBB - AccountID

Direção: cliente -> servidor. Tamanho: 9 bytes (fixo). Disponível: antes do login.

Finalidade: Mensagem do Ultima Messenger (dois serials).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xBB |
| 1 | 4 | u32 | serial1 | Não especificado |
| 5 | 4 | u32 | serial2 | Não especificado |

Notas:

O ServUO ignora o pacote; o Source-X o trata como mensagem de correio entre dois serials.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`AccountID`)
- Source-X: `src/network/receive.cpp` (`PacketMailMessage`)
