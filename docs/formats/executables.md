# Executáveis, bibliotecas e arquivos auxiliares

Arquivos do diretório do cliente que não contêm dados de jogo. Nenhum deles é interpretado pelo Fiddler; a aba Arquivos os classifica como "Executável" ou "Binário" e exibe apenas hex.

## Clientes

| Arquivo | Versão | Papel |
|---|---|---|
| `client.exe` | 7.0.23.1 | Cliente Classic 7.0.23.1. `client.exe.bak` e o `client.exe` dentro de `client.zip` são cópias idênticas. |
| `client 7.0.10.3.exe` | 7.0.10.3 | Cliente 7.0.10.3 (referência para tabelas de pacotes e comandos de gump em `docs/protocol`) |
| `client7.0.6.4.exe` | 7.0.6.4 | Cliente 7.0.6.4 |
| `client7.0.3.exe` | 7.0.3.0 | Cliente 7.0.3 |
| `client7.0.2.2.exe` | 7.0.2.2 | Cliente 7.0.2.2 |
| `Decrypted_client.exe` | 7.0.16.0 | Cliente 7.0.16 (cópia descriptografada) |
| `client6.0.1.10.exe`, `client601.exe` | 6.0.1.10 | Cliente 6.0.1.10 (arquivos idênticos) |
| `LUO-Client.exe` | 6.0.1.10 | Cliente 6.0.1.10 (variante LUO) |
| `client_5.0.9.1.exe` | 5.0.9.1 | Cliente 5.0.9.1 |
| `Client 502.exe`, `Client502.exe` | n/d | Cliente 5.0.2 pelo nome (arquivos idênticos) |

## Launcher, patcher e utilitários

| Arquivo | Papel |
|---|---|
| `uo.exe` | Launcher: verifica atualização e inicia o cliente; manifesto em `uo.exe.manifest` (exige administrador) |
| `UOPatch.exe` | Patcher automático (`UO AUTOPATCH`) |
| `ClientPicker.exe` | Seletor de cliente a executar |
| `Patchw32.dll` | Biblioteca RTPatch (Pocket Soft) usada para aplicar patches `.rtp` |
| `patch/` | Pasta de download de patches (`.rtp`) |
| `patchlog` | Log do patcher |
| `SetUOFolder.exe` | Utilitário .NET que define a pasta de instalação do Ultima Online |
| `uninstUOClassic.exe` | Desinstalador (NSIS) |
| `uotd.exe` | Executável auxiliar do cliente |
| `Owo.exe` | Executável auxiliar com cliente HTTP |
| `Transerv.exe` | Servidor de tradução de fala (usa `sjis2uni.mul`; configuração em `Uoupg.cfg`) |

## Bibliotecas

| Arquivo | Papel |
|---|---|
| `Binkw32.dll` | Codec de vídeo Bink (reprodução de `music/Intro.bik`) |
| `Granny.dll` | Granny 3D (Rad Game Tools): animação e renderização de modelos 3D |
| `zlib1.dll` | Compressão zlib |
| `dbghelp.dll` | Microsoft Debug Help Library |
| `Igrping.dll` | Biblioteca de ping (`CIgr_pinger`) carregada pelo cliente |
| `Igrping.dll.1`, `Igrping.dll.themida`, `Igrping.uog.dll`, `Igrping_UL.dll` | Variantes da `Igrping.dll` |
| `GDF/UOClassicGDF.dll` | Definição de jogo (Game Definition File) para o Windows Game Explorer; acompanha `UO-MCE.lnk`, `UO-MCE.mcl` e `UO-MCE.png` |

## Ferramenta de dependências

| Arquivo | Papel |
|---|---|
| `depends.exe` | Dependency Walker 2.2 (64 bits) |
| `depends.dll` | Biblioteca do Dependency Walker |
| `depends.chm` | Ajuda do Dependency Walker (Microsoft Compiled HTML Help) |

## Outros arquivos auxiliares

| Arquivo | Papel |
|---|---|
| `uo.exe.manifest`, `client.manifest` | Manifestos de aplicação do Windows |
| `client.zip` | Arquivo ZIP contendo `client.exe` |
| `regfile.reg` | Chaves de registro da instalação |
| `Logs/` | Logs de depuração do cliente (`DebugLog.*`) |
| `music/Intro.bik` | Vídeo de abertura (Bink) |
| `music/Digital/*.mp3` | Trilha sonora em MP3 |
| `*.cfg`, `*.lst` | Configurações e listas; ver `misc-binaries.md` |
