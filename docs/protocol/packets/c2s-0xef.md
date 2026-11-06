# 0xEF - Login Seed

Direção: cliente -> servidor. Tamanho: 21 bytes (fixo). Disponível: antes do login (primeiro pacote da conexão).

Finalidade: Envia o seed de criptografia e a versão do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xEF |
| 1 | 4 | u32 | seed | seed (zero é inválido) |
| 5 | 4 | u32 | major | versão maior do cliente |
| 9 | 4 | u32 | minor | versão menor |
| 13 | 4 | u32 | revision | revisão |
| 17 | 4 | u32 | prototype | patch |

Notas:

Clientes anteriores à 6.0.5.0 enviam apenas o seed de 4 bytes, sem id e sem versão. A tabela de pacotes marca este id como fora da tabela de tamanhos; os servidores o registram com 21 bytes.

Referência de implementação:

- ServUO: `Server/Network/PacketHandlers.cs` (`LoginServerSeed`)
- ModernUO: `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`LoginServerSeed`)
- ClassicUO: `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_Seed`, `Send_Seed_Old`)
