# 0x9D - GMSingle

Direção: cliente -> servidor. Tamanho: 51 bytes (fixo). Disponível: em jogo (somente cliente God).

Finalidade: Comando de GM (cliente God).

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x9D |
| 1 | 50 | u8[50] | data | Não especificado |

Notas:

O ServUO apenas despeja o conteúdo.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`GMSingle`)
