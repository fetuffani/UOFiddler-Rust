# 0x4B - Check Client Version

Direção: cliente -> servidor. Tamanho: 9 bytes (fixo). Disponível: Não especificado.

Finalidade: Verificação de versão do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0x4B |
| 1 | 8 | u8[8] | data | Não especificado |

Notas:

Nenhuma implementação de servidor de referência trata este id; o cliente 7.0.10.3 contém o construtor do pacote sem uso.

Referência de implementação:

- Tabela de tamanhos do ClassicUO: `src/ClassicUO.Client/Network/PacketsTable.cs` (9 bytes)
