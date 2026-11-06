# 0xCF - AccountLogin

Direção: cliente -> servidor. Tamanho: 78 bytes (fixo). Disponível: antes do login.

Finalidade: Login de conta em clientes recentes.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xCF |
| 1 | 77 | u8[77] | data | Não especificado |

Notas:

O ServUO registra o id como variável e lê account[30] e password[30] a partir do offset 3. O ModernUO e o Source-X não tratam este id.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`AccountLogin`)
