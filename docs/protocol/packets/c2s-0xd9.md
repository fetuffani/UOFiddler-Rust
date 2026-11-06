# 0xD9 - Spy on Client (hardware)

Direção: cliente -> servidor. Tamanho: 268 bytes (fixo). Disponível: antes do login e em jogo.

Finalidade: Envia dados de hardware e sistema do cliente.

| Offset | Tamanho | Tipo | Campo | Descrição |
|---|---|---|---|---|
| 0 | 1 | u8 | id | 0xD9 |
| 1 | 1 | u8 | clientType | tipo de cliente |
| 2 | 4 | u32 | instanceId | id da instância |
| 6 | 4 | u32 | osMajor | versão maior do sistema operacional |
| 10 | 4 | u32 | osMinor | versão menor |
| 14 | 4 | u32 | osRevision | revisão |
| 18 | 1 | u8 | cpuManufacturer | fabricante da CPU |
| 19 | 4 | u32 | cpuFamily | família |
| 23 | 4 | u32 | cpuModel | modelo |
| 27 | 4 | u32 | cpuClock | clock |
| 31 | 1 | u8 | cpuQuantity | número de CPUs |
| 32 | 4 | u32 | memory | memória física |
| 36 | 4 | u32 | screenWidth | largura da tela |
| 40 | 4 | u32 | screenHeight | altura da tela |
| 44 | 4 | u32 | screenDepth | profundidade de cor |
| 48 | 2 | u16 | directXMajor | DirectX maior |
| 50 | 2 | u16 | directXMinor | DirectX menor |
| 52 | 128 | char16[64] | videoCard | descrição da placa de vídeo, UTF-16 |
| 180 | 4 | u32 | videoVendorId | id do fabricante |
| 184 | 4 | u32 | videoDeviceId | id do dispositivo |
| 188 | 4 | u32 | videoMemory | memória de vídeo |
| 192 | 1 | u8 | distribution | distribuição |
| 193 | 1 | u8 | clientsRunning | clientes em execução |
| 194 | 1 | u8 | clientsInstalled | clientes instalados |
| 195 | 1 | u8 | clientsPartial | clientes parcialmente instalados |
| 196 | 8 | char16[4] | language | idioma, UTF-16 |
| 204 | 64 | char16[32] | unknown | Não especificado |

Notas:

A ordem de bytes dos campos numéricos e das cadeias UTF-16 não é especificada pelos servidores de referência (que apenas ignoram o conteúdo). O cliente 7.0.10.3 só envia este pacote respeitando um intervalo mínimo entre envios.

Referência de implementação:

- Source-X: `src/network/receive.cpp` (`PacketHardwareInfo`)
- Tabela de tamanhos do ClassicUO: `src/ClassicUO.Client/Network/PacketsTable.cs` (268 bytes)
