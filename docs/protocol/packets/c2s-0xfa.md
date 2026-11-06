# 0xFA - Unhandled (UO Store)

Direção: cliente -> servidor. Tamanho: 1 byte (fixo). Disponível: em jogo.

Finalidade: Abre a UO Store (botão da loja).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xFA |

Notas:

Sem payload. O ServUO deixa o tratamento para o módulo da UO Store; a tabela de pacotes não fixa tamanho para este id.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (comentário de registro 0xFA)
- Source-X: `src/network/receive.cpp` (`PacketUltimaStoreButton`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_OpenUOStore`)
