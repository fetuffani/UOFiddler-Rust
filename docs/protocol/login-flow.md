# Fluxo de login (conexão, conta, servidor de jogo, personagem e entrada no mundo)

Fluxo multi-pacote que leva o cliente da conexão TCP inicial até o personagem dentro do mundo. Há duas conexões: a primeira com o servidor de login (conta e lista de servidores) e a segunda com o servidor de jogo (lista de personagens, seleção ou criação e entrada no mundo). O layout campo a campo de cada pacote está nos arquivos de [packets/](packets/README.md); este documento descreve a ordem, as dependências entre os pacotes e as diferenças entre implementações. Só o que não tem arquivo próprio (seed de 4 bytes) é detalhado aqui.

Convenções: inteiros big-endian; offsets a partir do byte 0 (id). `S->C` = servidor para cliente, `C->S` = cliente para servidor. Ids e tamanhos gerais em [packets.md](packets.md).

## 1. Visão geral e versões de cliente

| Etapa | Conexão | Pacotes |
|---|---|---|
| Seed | login | seed de 4 bytes ou `0xEF` |
| Conta | login | `0x80` (C->S), `0xA8` ou `0x82` (S->C) |
| Escolha do servidor | login | `0xA0` (C->S), `0x8C` (S->C) |
| Login no servidor de jogo | jogo | seed = chave do `0x8C`, `0x91` (C->S), `0xB9` e `0xA9` (S->C) |
| Personagem | jogo | `0x5D`, `0x00`, `0xF8`, `0x8D`, `0x83` (C->S); `0x85`, `0x86`, `0x53` (S->C) |
| Versão do cliente | jogo | `0xBD` (S->C pedido, C->S resposta) |
| Entrada no mundo | jogo | `0x1B` e seguintes, terminando em `0x55` |

Diferenças por versão de cliente confirmadas nas implementações:

| Versão | Efeito |
|---|---|
| Anterior a 6.0.5.0 | O seed é enviado sozinho (4 bytes), sem `0xEF` e sem versão. O servidor só conhece a versão depois do `0xBD` |
| 6.0.5.0 ou superior | `0xEF` com seed e versão (`major`, `minor`, `revision`, `prototype`) |
| Anterior a 6.0.14.2 | `0xB9` com 3 bytes (flags `u16`) |
| 6.0.14.2 ou superior | `0xB9` com 5 bytes (flags `u32`) |
| 7.0.0.0 ou superior | Valores de `genderRace` com gárgula (ver [c2s-0x00](packets/c2s-0x00.md)) |
| 7.0.13.0 ou superior | `0xA9` com cidades de 89 bytes e campo `lastSlot` |
| 7.0.16.0 ou superior | Criação de personagem pelo `0xF8` (4 habilidades) |
| Enhanced Client | Criação pelo `0x8D`; versão maior `>= 67` é tratada como Enhanced pelo ServUO |

Detecção no ServUO: `NetState.Version` (setter) escolhe o conjunto de `ProtocolChanges` (`NewCharacterList` a partir de 7.0.13.0, `NewCharacterCreation` a partir de 7.0.16.0, `ExtendedSupportedFeatures` a partir de 6.0.14.2, `StygianAbyss` a partir de 7.0.0.0).

## 2. Sequência

### 2.1 Servidor de login

1. O cliente abre a conexão e envia o seed: [`0xEF`](packets/c2s-0xef.md) (6.0.5.0+) ou o seed de 4 bytes (seção 3.1). Seed zero é inválido e a conexão é encerrada.
2. C->S [`0x80`](packets/c2s-0x80.md) com conta e senha. O ClassicUO envia o seed e o `0x80` em sequência, sem esperar resposta.
3. O servidor valida a conta. Em sucesso, S->C [`0xA8`](packets/s2c-0xa8.md) (lista de servidores). Em falha, S->C [`0x82`](packets/s2c-0x82.md) com o motivo e encerramento da conexão (ServUO, ModernUO).
4. C->S [`0xA0`](packets/c2s-0xa0.md) com o índice do servidor escolhido.
5. S->C [`0x8C`](packets/s2c-0x8c.md) com endereço IPv4, porta e chave de autenticação (`authId`).

### 2.2 Servidor de jogo

1. O cliente fecha a conexão de login e abre uma nova com o endereço e a porta do `0x8C`. Na nova conexão ativa a compressão e a criptografia de jogo, e envia primeiro os 4 bytes da chave (o seed da conexão, big-endian, sem id), em seguida [`0x91`](packets/c2s-0x91.md) com a chave repetida no offset 1, conta e senha (ClassicUO, `HandleRelayServerPacket`).
2. O servidor valida a chave e a conta. Em recusa, ServUO e ModernUO encerram a conexão sem enviar `0x82`.
3. Em sucesso, S->C [`0xB9`](packets/s2c-0xb9.md) (recursos habilitados) e [`0xA9`](packets/s2c-0xa9.md) (personagens, cidades iniciais e flags). ServUO e ModernUO ligam a compressão de saída nesse ponto.
4. O cliente escolhe uma das ações da seção 2.3.

### 2.3 Lista de personagens

| Ação | C->S | Resposta do servidor |
|---|---|---|
| Entrar com personagem existente | [`0x5D`](packets/c2s-0x5d.md) | Seção 2.4 |
| Criar personagem | [`0x00`](packets/c2s-0x00.md), [`0xF8`](packets/c2s-0xf8.md) ou [`0x8D`](packets/c2s-0x8d.md) | Seção 2.4 em sucesso; em recusa, [`0x53`](packets/s2c-0x53.md) ou encerramento da conexão |
| Excluir personagem | [`0x83`](packets/c2s-0x83.md) | [`0x86`](packets/s2c-0x86.md) com a lista atualizada; em falha, [`0x85`](packets/s2c-0x85.md) com o motivo (ServUO: seguido de `0x86`) |

Escolha do pacote de criação pelo ClassicUO: `0xF8` quando a versão é `>= CV_70160`, `0x00` caso contrário (`Send_CreateCharacter`). O ClassicUO não envia `0x8D`.

Se o slot de `0x5D` é inválido ou vazio, ServUO e ModernUO encerram a conexão. Se outro personagem da conta já está no mundo, ServUO e ModernUO respondem [`0x53`](packets/s2c-0x53.md) com `CharInWorld` e não encerram. No Source-X o equivalente (`CharIdle`) é devolvido como `0x82` com o motivo `InUse` e fecha a leitura da conexão (`addLoginErr`); o Source-X também envia um aviso (`addIdleWarning`).

### 2.4 Versão do cliente e entrada no mundo

1. Se o servidor ainda não conhece a versão (cliente sem `0xEF`), S->C [`0xBD`](packets/s2c-0xbd.md) (`BD 00 03`) e C->S [`0xBD`](packets/c2s-0xbd.md) com a versão como texto. ServUO bloqueia o processamento dos demais pacotes do cliente (`BlockAllPackets`) e um temporizador de 1 s (ServUO) ou 64 ms (ModernUO) continua o login quando a versão chega. O ModernUO pede a versão sempre, mesmo quando a conhece.
2. S->C [`0x1B`](packets/s2c-0x1b.md) Login Confirm: serial, corpo, posição, direção e tamanho do mapa.
3. Estado do mundo, na ordem de cada servidor (seção 2.5), incluindo mapa ([`0xBF/0x08`](packets/bf-0x08.md)), patches de mapa ([`0xBF/0x18`](packets/bf-0x18.md)), recursos ([`0xB9`](packets/s2c-0xb9.md)), o próprio mobile ([`0x78`](packets/s2c-0x78.md) e [`0x20`](packets/s2c-0x20.md)), luz ([`0x4F`](packets/s2c-0x4f.md) e [`0x4E`](packets/s2c-0x4e.md)), status ([`0x11`](packets/s2c-0x11.md)) e modo de guerra ([`0x72`](packets/s2c-0x72.md)).
4. S->C [`0x55`](packets/s2c-0x55.md) Login Complete. O ClassicUO só troca para a cena de jogo ao receber o `0x55`, e somente se já criou o jogador com o `0x1B`.

Mensagens que o ClassicUO envia ao entrar (referência de cliente, não exigidas pelos servidores de referência):

| Quando | Pacote |
|---|---|
| Ao receber `0x1B` | [`0xBF/0x05`](packets/bf-0x05.md) (tamanho da janela), [`0xBF/0x0B`](packets/bf-0x0b.md) (idioma), `0xBD` (versão), [`0x09`](packets/c2s-0x09.md) (clique simples no próprio personagem), [`0x34`](packets/c2s-0x34.md) (pedido de skills) |
| Ao receber `0x55` | [`0x34`](packets/c2s-0x34.md) (status e skills), [`0x06`](packets/c2s-0x06.md) (duplo clique no próprio personagem), [`0xBF/0x0F`](packets/bf-0x0f.md) (flags do cliente), [`0xC8`](packets/c2s-0xc8.md) (alcance de visão) |

Parte dessas mensagens depende da versão do cliente (`CV_200`, `CV_305D`, `CV_306E`, `CV_70796` em `EnterWorld` e `LoginComplete`). O pacote chamado `Send_ClientType` no código do ClassicUO é o `0xBF/0x0F`; o ClassicUO não envia o `0xE1`.

### 2.5 Ordem de envio ao entrar (por implementação)

ServUO (`PacketHandlers.DoLogin`):

1. `0x1B`
2. `0xBF/0x08` (se há mapa) e `0xBF/0x18`
3. `0xB9`
4. `0x78` do próprio jogador
5. `0x20` (formato `MobileUpdate` ou `MobileUpdateOld` conforme a versão)
6. `SendEverything` (itens e mobiles ao redor)
7. `0x4F` e `0x4E` (`CheckLightLevels(true)`)
8. `0x55`
9. `0x78`, `0x11`, `0x72`, `0xBC`, `0x5B`, `0xBF/0x08`

ModernUO (`IncomingAccountPackets.DoLogin`):

1. `0x1B`
2. `0xBF/0x08` e `0xBF/0x18`
3. `0xBC` e `0xB9`
4. `0x20` (duas vezes)
5. luz (`0x4F`, `0x4E`) e `0x20`
6. `0x78`, `0x11` e `0x72`
7. `SendEverything`
8. `0xB9`, `0x20`, `0x11`, `0x72` e `0x78` novamente
9. `0x55`, `0x5B`, `0xBC`, `0xBF/0x08`
10. `0x6D` (música da região)

Source-X (`CClient::addPlayerStart`):

1. `0x1B`
2. `0xBF/0x18` (sempre enviado, mesmo vazio)
3. `MoveToChar` e `Update` (pacotes do próprio mobile e do entorno; os ids exatos não foram rastreados)
4. `0x72`
5. `0x55`
6. `0x5B`
7. `0xBC`

O Source-X não está confirmado quanto à posição de `0x4F`/`0x4E` nessa sequência (`addLight` é chamado em `addPlayerView`).

## 3. Pacotes

### 3.1 Seed de 4 bytes (C->S, sem id)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u32 | seed | Big-endian; zero é inválido |

Usado por clientes anteriores a 6.0.5.0 na conexão de login. Na conexão de jogo o ClassicUO envia o seed assim, com o `authId` do `0x8C` como valor. O ModernUO trata como seed de 4 bytes os primeiros bytes de qualquer conexão que não começa por `0xEF`. O ServUO diferencia o `0xEF` de um seed de 4 bytes pelo primeiro byte (`0xEF` = 239; o comentário do código diz que um seed normal não começa com esse valor). Ver `MessagePump.HandleSeed`.

No ClassicUO o valor do seed da conexão de login é o IPv4 local do socket (`NetClient.Socket.LocalIP`).

### 3.2 Pacotes com arquivo próprio

| Pacote | Direção | Tamanho | Papel | Arquivo |
|---|---|---|---|---|
| `0xEF` Login Seed | C->S | 21 | seed e versão do cliente | [c2s-0xef](packets/c2s-0xef.md) |
| `0x80` Login Request | C->S | 62 | conta, senha e `nextLoginKey` | [c2s-0x80](packets/c2s-0x80.md) |
| `0xA8` Game Server List | S->C | `6 + 40 * count` | servidores disponíveis | [s2c-0xa8](packets/s2c-0xa8.md) |
| `0x82` Login Denied | S->C | 2 | motivo da recusa | [s2c-0x82](packets/s2c-0x82.md) |
| `0xA0` Select Server | C->S | 3 | índice na lista do `0xA8` | [c2s-0xa0](packets/c2s-0xa0.md) |
| `0x8C` Connect To Game Server | S->C | 11 | IP, porta e `authId` | [s2c-0x8c](packets/s2c-0x8c.md) |
| `0x91` Game Server Login | C->S | 65 | `authId`, conta e senha | [c2s-0x91](packets/c2s-0x91.md) |
| `0xB9` Enable Locked Client Features | S->C | 5 (3 em clientes antigos) | flags de recursos | [s2c-0xb9](packets/s2c-0xb9.md) |
| `0xA9` Characters + Starting Locations | S->C | var | slots, cidades e flags | [s2c-0xa9](packets/s2c-0xa9.md) |
| `0x5D` Play Character | C->S | 73 | slot do personagem | [c2s-0x5d](packets/c2s-0x5d.md) |
| `0x00` Create Character | C->S | 104 | criação clássica | [c2s-0x00](packets/c2s-0x00.md) |
| `0xF8` CreateCharacter70160 | C->S | 106 | criação 7.0.16.0+ | [c2s-0xf8](packets/c2s-0xf8.md) |
| `0x8D` ECCreateCharacter | C->S | var (146) | criação pelo Enhanced Client | [c2s-0x8d](packets/c2s-0x8d.md) |
| `0x83` DeleteCharacter | C->S | 39 | exclusão por slot | [c2s-0x83](packets/c2s-0x83.md) |
| `0x85` Character Delete Result | S->C | 2 | motivo da falha | [s2c-0x85](packets/s2c-0x85.md) |
| `0x86` Character List Update | S->C | var | lista após criar ou excluir | [s2c-0x86](packets/s2c-0x86.md) |
| `0x53` Reject Character Logon | S->C | 2 | recusa ao entrar com personagem | [s2c-0x53](packets/s2c-0x53.md) |
| `0x81` Change Character | S->C | var | lista de personagens da conta para troca | [s2c-0x81](packets/s2c-0x81.md) |
| `0xBD` Client Version | S->C / C->S | 3 / var | pedido e resposta de versão | [s2c-0xbd](packets/s2c-0xbd.md), [c2s-0xbd](packets/c2s-0xbd.md) |
| `0xBE` Assist Version | S->C / C->S | 7 / var | versão de assistente | [s2c-0xbe](packets/s2c-0xbe.md), [c2s-0xbe](packets/c2s-0xbe.md) |
| `0xE1` ClientType | C->S | var | tipo de cliente (KR, Enhanced) | [c2s-0xe1](packets/c2s-0xe1.md) |
| `0xA4` System Info | C->S | 149 | hardware (antigo) | [c2s-0xa4](packets/c2s-0xa4.md) |
| `0xD9` Spy on Client | C->S | 268 | hardware | [c2s-0xd9](packets/c2s-0xd9.md) |
| `0x1B` Login Confirm | S->C | 37 | confirmação de entrada | [s2c-0x1b](packets/s2c-0x1b.md) |
| `0x55` Login Complete | S->C | 1 | fim do carregamento | [s2c-0x55](packets/s2c-0x55.md) |
| `0x78` Mobile Incoming | S->C | var | o próprio personagem com equipamento | [s2c-0x78](packets/s2c-0x78.md) |
| `0x20` Draw Game Player | S->C | 19 | corpo, posição e direção do jogador | [s2c-0x20](packets/s2c-0x20.md) |
| `0x4F` Overall Light Level | S->C | 2 | luz global | [s2c-0x4f](packets/s2c-0x4f.md) |
| `0x4E` Personal Light Level | S->C | 6 | luz pessoal | [s2c-0x4e](packets/s2c-0x4e.md) |
| `0x72` War Mode | S->C / C->S | 5 | modo de guerra | [s2c-0x72](packets/s2c-0x72.md), [c2s-0x72](packets/c2s-0x72.md) |
| `0xBF/0x08` | S->C | 6 | mapa atual | [bf-0x08](packets/bf-0x08.md) |
| `0xBF/0x18` | S->C | `9 + 8 * n` | patches de mapa | [bf-0x18](packets/bf-0x18.md) |
| `0xBC` Seasonal Info | S->C | 3 | estação | [s2c-0xbc](packets/s2c-0xbc.md) |
| `0x5B` Time | S->C | 4 | hora do jogo | [s2c-0x5b](packets/s2c-0x5b.md) |

O `0xB8` (perfil de personagem) é pacote de jogo e não faz parte do login: ver [c2s-0xb8](packets/c2s-0xb8.md) e [s2c-0xb8](packets/s2c-0xb8.md).

## 4. Estruturas de dados

### 4.1 Chave de autenticação (`authId`)

Valor `u32` criado pelo servidor ao receber o `0xA0`, devolvido no `0x8C` e verificado no `0x91`. O cliente também o usa como seed da conexão de jogo.

| Implementação | Geração | Verificação no `0x91` |
|---|---|---|
| ServUO | Aleatório em `[1, uint.MaxValue - 1]`, com o bit 31 ligado em 50% das vezes; janela de ids pendentes com tamanho fixo (`m_AuthIDWindowSize`), o mais antigo é removido quando a janela enche | `authId` deve estar na janela (se `m_ClientVerification`), e deve ser igual ao seed da conexão. O id usado é removido e a versão do cliente guardada com ele é restaurada |
| ModernUO | Aleatório; zero e qualquer valor com primeiro byte `0xEF` são excluídos; vale 2 minutos; é ligado à conta e ao endereço IP de quem o recebeu | `authId` deve ser igual ao seed e pertencer ao mesmo usuário e endereço. Dentro do prazo, substitui a verificação de senha; expirado, cai na verificação de senha. Id desconhecido ou de outro usuário/endereço: conexão encerrada |
| Source-X | Com `UseAuthID` ligado: CRC32 de `nomeDoServidor + nomeDaConta`; desligado: `0x7F000001` fixo | Deve coincidir com o valor guardado na conta (`customerid`); diferente: `BadAuthID` |

### 4.2 Slots de personagem

Cada slot ocupa 60 bytes em `0xA9` e `0x86`: 30 bytes de nome ASCII e 30 bytes zerados; slot livre = 60 zeros. O ClassicUO usa só os 30 primeiros bytes e ignora os demais. O índice de slot enviado em `0x5D`, `0x00`/`0xF8`/`0x8D` e `0x83` é o índice nessa lista. O ClassicUO escolhe o primeiro slot de nome vazio para criar um personagem.

### 4.3 Cidades iniciais

O servidor envia as cidades no `0xA9`; o índice escolhido volta em `cityIndex` nos pacotes de criação. ServUO e ModernUO rejeitam (encerram a conexão) um `cityIndex` fora da lista da conexão (`state.CityInfo`). O ServUO escolhe a lista em `AccountHandler.EventSink_GameLogin` (`SiegeStartingCities`, `StartingCitiesT2A`, `StartingCities` ou `StartingCitiesSA`, conforme o shard e a expansão).

### 4.4 Lista de servidores

Cada entrada do `0xA8` tem 40 bytes e o índice enviado em `0xA0` é o índice da entrada (`u16`). ServUO numera a partir de 0; Source-X numera a partir de 1 e limita a lista a 32 entradas (comentário no código: muitos servidores podem derrubar o cliente). O ClassicUO envia o índice como `u8` (`Send_SelectServer`: `u8 0x00` seguido do `u8` índice).

## 5. Criptografia e compressão

### 5.1 Login (conexão de login)

- O seed é enviado sem criptografia (ClassicUO: `skipEncryption`).
- A criptografia do `0x80` e dos pacotes seguintes da conexão de login depende do seed e de duas chaves derivadas da versão do cliente. Só vale no sentido cliente para servidor; as respostas do servidor de login não são criptografadas (ModernUO, `LoginEncryption.ServerEncrypt` não faz nada).
- ModernUO (`LoginKeys.ComputeKeys` e `LoginEncryption`): `key1` e `key2` são calculados a partir de `major`, `minor` e `revision`; as tabelas de estado são iniciadas a partir do seed (`table1 = ((~seed ^ 0x00001357) << 16) | ((seed ^ 0xFFFFAAAA) & 0xFFFF)`, `table2 = ((seed ^ 0x43210000) >> 16) | ((~seed ^ 0xABCDFFFF) & 0xFFFF0000)`) e cada byte é combinado por XOR com o byte baixo de `table1`, que depois avança. A validação de um login criptografado é: byte 0 = `0x80` e bytes 30 e 60 = `0x00` após descriptografar.
- O ClassicUO escolhe o tipo de criptografia pela versão (`EncryptionHelper.CalculateEncryption`): `OLD_BFISH`, `BLOWFISH__1_25_36`, `BLOWFISH` e `BLOWFISH__2_0_3` para versões antigas e `TWOFISH_MD5` para versões acima de 2.0.3.
- Clientes anteriores a 6.0.5.0 não enviam a versão no seed: o ModernUO tenta todas as chaves conhecidas (`LoginKeys.LegacyKeys`) e o Source-X tenta todas as versões na primeira mensagem (`m_Crypt.Init`, `xProcessClientSetup`).

### 5.2 Jogo (conexão de jogo)

- A chave vem do seed da conexão (o `authId` do `0x8C`). ModernUO (`GameEncryption`): chave Twofish de 16 bytes com o seed repetido 4 vezes (big-endian); uma tabela de 256 bytes (valores 0 a 255) é criptografada com Twofish e renovada a cada 256 bytes recebidos; cliente para servidor = XOR com a tabela; servidor para cliente = XOR com os 16 bytes de `MD5(tabela)`, em rotação.
- ModernUO: a criptografia do `0x91` é detectada pelos primeiros bytes da mensagem depois da descriptografia (id `0x91` e `authId` igual ao seed).
- ClassicUO: na conexão de jogo, `Encryption.Initialize(false, seed)` e `EnableCompression()` são chamados logo após conectar; o cliente só descriptografa a entrada quando a compressão está ligada e o tipo é Twofish/MD5.
- Source-X: ao enviar o `0x8C`, já troca a criptografia da conexão para o modo de jogo (`PacketServerRelay::onSent`, `InitFast(customerId, CONNECT_GAME)`), caso o cliente continue na mesma conexão.

### 5.3 Compressão

- S->C, depois do login de jogo aceito: compressão Huffman de todo o tráfego do servidor (ServUO e ModernUO: `CompressionEnabled = true` no `GameLogin`; Source-X comprime todo pacote quando `GetConnectType() == CONNECT_GAME`).
- Ordem de saída (ModernUO): primeiro comprime, depois criptografa.

### 5.4 Clientes criptografados

- ServUO não suporta cliente criptografado: se o primeiro pacote não for um dos ids `0xF0`, `0xF1`, `0xCF`, `0x80`, `0x91`, `0xA4`, `0xEF`, `0xE4`, `0xFF`, a conexão é encerrada (`MessagePump.CheckEncrypted`). Os ids `0x80` e `0x91` são os do login e do login de jogo em claro.
- ModernUO aceita criptografados e não criptografados por padrão (`network.encryptionMode = Both`, modos `Unencrypted`, `Encrypted`, `Both`, `None`).
- Source-X aceita conforme `USECRYPT` e `USENOCRYPT` do arquivo de configuração.

## 6. Regras do servidor

1. O primeiro pacote da conexão é o seed. Seed zero: encerrar a conexão.
2. Aceitar um único `0x80` por conexão de login e um único `0x91` por conexão de jogo (ServUO: `SentFirstPacket`; ModernUO também recusa um segundo `0xA0` e um segundo seed na mesma conexão).
3. Registrar a chave do `0x8C` associada à conta (e, no ModernUO, ao IP) antes de enviar o pacote; o `0x91` só é aceito se a chave bate com o seed da conexão.
4. O endereço de cada servidor na lista é decisão do servidor. O ServUO devolve o endereço do socket local e troca pelo endereço público configurado quando o cliente não é loopback nem de rede privada (`Scripts/Misc/ServerList.cs`).
5. Em `0xA8` e `0x8C` respeitar a ordem dos octetos do IP por versão de cliente (ver arquivos dos pacotes).
6. Enviar `0xB9` antes de `0xA9` no login de jogo.
7. Escolher o formato de `0xA9` (cidades de 63 ou 89 bytes) e o tamanho de `0xB9` (3 ou 5 bytes) pela versão do cliente; sem versão conhecida (cliente anterior a 6.0.5.0), pedir o `0xBD` antes de continuar o login.
8. Recusar a entrada se outro personagem da conta está no mundo (`0x53` com `CharInWorld`). Se o personagem escolhido já tem uma conexão, ServUO e ModernUO encerram a conexão anterior.
9. Na exclusão: recusar personagem em uso, personagem criado há pouco tempo (`RestrictDeletion` e `DeleteDelay` no ServUO; `m_iMinCharDeleteTime` no Source-X) e índice inválido, com `0x85`. O ServUO envia `0x86` depois de cada `0x85` e depois de cada exclusão bem-sucedida; o Source-X envia só o `0x85`.
10. `0x55` indica o fim do carregamento inicial e vem depois de `0x1B` e do estado do mundo.

## 7. Exemplo

Cliente 7.0.10.3 (`major = 7`, `minor = 0`, `revision = 10`, `prototype = 3`), conta `user`, senha `pass`. Exemplo de ilustração dos layouts. Zeros repetidos abreviados como `00 x N`.

Servidor de login, C->S:

```
EF                                  id
7F 00 00 01                         seed (exemplo)
00 00 00 07  00 00 00 00            major = 7, minor = 0
00 00 00 0A  00 00 00 03            revision = 10, prototype = 3

80                                  id
75 73 65 72  00 x 26                "user" preenchido até 30 bytes
70 61 73 73  00 x 26                "pass" preenchido até 30 bytes
FF                                  nextLoginKey
```

S->C, um servidor na lista (IP na ordem natural dos octetos, como Source-X e ClassicUO interpretam):

```
A8 00 2E 5D 00 01                   id, tamanho 46, flags, count = 1
00 00                               index = 0
54 65 73 74 00 x 28                 "Test" preenchido até 32 bytes
00 00                               percentFull = 0, timezone = 0
7F 00 00 01                         IP
```

Escolha e redirecionamento:

```
A0 00 00                            C->S: servidor de índice 0
8C 7F 00 00 01 0A 21 12 34 56 78    S->C: IP 127.0.0.1, porta 2593 (0x0A21), authId 0x12345678
```

Servidor de jogo (nova conexão), C->S:

```
12 34 56 78                         seed = authId (4 bytes, sem id)
91 12 34 56 78                      id e authId
75 73 65 72  00 x 26                conta
70 61 73 73  00 x 26                senha
```

Recusa de login (servidor de login), S->C, senha incorreta no ServUO (`BadPass = 0x03`):

```
82 03
```

Entrada no mundo, S->C (`0x1B`): jogador `0x00000007`, corpo `0x0190`, posição (1500, 1600, 0), direção 0, mapa 6144 x 4096:

```
1B  00 00 00 07  00 00 00 00  01 90  05 DC  06 40  00 00  00 00
    FF FF FF FF  00 00 00 00  18 00  10 00  00 00 00 00 00 00
```

Fim do carregamento: `55`.

## 8. Armadilhas

- A conexão de jogo começa com 4 bytes de seed (o `authId`), sem id, e só depois vem o `0x91`. No ModernUO, uma conexão que não começa por `0xEF` tem os 4 primeiros bytes lidos como seed; no ServUO e no ModernUO o `authId` do `0x91` precisa ser igual a esse seed.
- O `authId` não pode começar com o byte `0xEF` (ModernUO): o servidor leria o primeiro byte como o pacote `0xEF` e ficaria esperando bytes que nunca chegam.
- O índice enviado no `0xA0` é o que veio no `0xA8` (base 0 no ServUO, base 1 no Source-X). Quem monta a lista decide a base.
- O IP do `0xA8` não é usado para conectar (a conexão real vem do `0x8C`). O IP do `0x8C` está em ordem natural dos octetos.
- Cliente anterior a 6.0.5.0 não informa a versão no seed. O ServUO só pede o `0xBD` depois de `0x5D` ou da criação de personagem, e usa o formato dos pacotes conforme a versão que conhece (`NetState.Version`) quando envia `0xB9` e `0xA9`.
- O ServUO bloqueia os demais pacotes do cliente enquanto espera o `0xBD` (`BlockAllPackets`); respostas atrasadas do cliente atrasam a entrada do jogador.
- `0xA9` não tem o campo `lastSlot` em clientes anteriores a 7.0.13.0 e usa cidades de 63 bytes.
- Em `0x5D` o slot vem do cliente: validar contra o tamanho da lista da conta (ServUO, ModernUO e Source-X rejeitam índice inválido de formas diferentes).
- O ServUO escreve o IP do `0xA8` em ordem invertida; clientes a partir de 4.0.0 esperam ordem natural (ver [s2c-0xa8](packets/s2c-0xa8.md)).
- Alguns pacotes são enviados mais de uma vez na entrada: o ModernUO envia `0x20` quatro vezes e `0xB9`, `0x11`, `0x72` e `0x78` duas vezes, o ServUO envia `0x78` do jogador duas vezes.

## 9. Diferenças entre implementações

| Aspecto | ServUO | ModernUO | Source-X | ClassicUO (cliente) |
|---|---|---|---|---|
| Seed | `0xEF` pelo primeiro byte; senão 4 bytes | `0xEF` ou 4 bytes; seed zero banido | `0xEF` ou 4 bytes; KR usa seed `0xFFFFFFFF` e espera o servidor enviar `0xE3` | Seed = IPv4 local; `0xEF` a partir de `CV_6040` (o arquivo de [c2s-0xef](packets/c2s-0xef.md) cita 6.0.5.0 como limite) |
| Motivos do `0x82` | `Invalid` 0, `InUse` 1, `Blocked` 2, `BadPass` 3, `Idle` 0xFE, `BadComm` 0xFF | Mesmos valores (`ALRReason`) | `Invalid` 0, `InUse` 1, `Blocked` 2, `BadPass` 3, `Other` 4; os demais motivos internos são traduzidos para esses | Mapeia o código para mensagem pela posição (0 a 8, valores >= 9 usam o último item) |
| Recusa no `0x91` | encerra a conexão | encerra a conexão | `addLoginErr` envia `0x82` com o motivo traduzido e fecha | n/a |
| Criptografia | não suporta | login: XOR; jogo: Twofish/MD5 | tenta todas as versões; configurável | Blowfish e Twofish/MD5 por versão |
| Compressão S->C | Huffman após `0x91` | Huffman após `0x91` | Huffman em conexões de jogo | descomprime após o `0x8C` |
| Formato de `0xB9` | 3 ou 5 bytes por versão (`ExtendedSupportedFeatures`) | `SendSupportedFeature` | 4 bytes se a versão `>= MINCLIVER_EXTRAFEATURES` | lê 4 bytes a partir de `CV_60142` |
| Criação `0x8D` | registrado | não registrado | registrado (146 bytes) | não envia |
| Pedido de `0xBD` | só se a versão é desconhecida | sempre ao selecionar ou criar | no login (se a versão reportada é 0) e no login de jogo quando necessário | responde ao pedido |
| Ordem de entrada | seção 2.5 | seção 2.5 | seção 2.5 | espera `0x1B` e `0x55` |

Particularidades:

- O ClassicUO trata `0x82`, `0x85` e `0x53` no mesmo handler e escolhe a mensagem pelo id do pacote e pelo código (`ServerErrorMessages.GetError`).
- No ModernUO, depois do `0xA0` o servidor de login desconecta o cliente (`ProtocolState.LoginServer_AwaitingServerSelect`). Se o cliente continuar enviando na mesma conexão, o estado volta a `AwaitingSeed`, exceto quando compilado com `STRICT_UO_PROTOCOL` (comentário no código: "CUO/Orion do not reconnect").
- O Source-X subtrai `0x80` do índice do `0xA0` quando ele é `>= 0x80` (comentário no código: "Sometimes we get an extra 0x80") e subtrai 1 do índice de base 1 (`Login_Relay`).
- O ClassicUO tem handler para `0xFD` (`LoginDelay`: `u8 delay`, mostra o intervalo `(delay - 1) * 10` a `delay * 10` na mensagem do `0x53` com código 13 ou 14). Nenhuma implementação de servidor de referência consultada envia `0xFD`.

## 10. Não especificado

- Layout e significado de `0xFD` (`LoginDelay`): só o ClassicUO o trata; nenhum servidor de referência o envia.
- Conteúdo do `0xE3` (criptografia KR) além do envio de um bloco fixo de 77 bytes pelo Source-X (`PacketKREncryption`) e do handler `KREncryptionResponse` no ClassicUO.
- Posição de `0x4F` e `0x4E` na sequência de entrada do Source-X.
- Ids exatos dos pacotes do mobile e do entorno enviados por `MoveToChar`/`Update` no Source-X ao entrar.
- Algoritmo completo de derivação das chaves de Blowfish e do login criptografado para versões antigas no ClassicUO (`LoginCryptBehaviour`, `BlowfishBehaviour`) e no Source-X (`CCrypto*`).
- Significado do campo `nextLoginKey` do `0x80` (valor do `uo.cfg`; ignorado pelos servidores; o ClassicUO envia `0xFF`).
- Finalidade de `0xBE`, `0xA4` e `0xD9`: os servidores de referência descartam o conteúdo.
- Significado dos bits `Unk1`, `Unk2`, `Unk3` e `Unk4` das flags do `0xA9`.

## 11. Referência de implementação

| Peça | Arquivo / símbolo |
|---|---|
| ServUO: seed e fila de entrada | `Server/Network/MessagePump.cs` (`HandleSeed`, `CheckEncrypted`, `HandleReceive`) |
| ServUO: handlers | `Server/Network/PacketHandlers.cs` (`LoginServerSeed`, `AccountLogin`, `AccountLogin_ReplyAck`, `AccountLogin_ReplyRej`, `PlayServer`, `GenerateAuthID`, `GameLogin`, `PlayCharacter`, `CreateCharacter`, `CreateCharacter70160`, `ECCreateCharacter`, `DeleteCharacter`, `ClientVersion`, `ClientType`, `LoginTimer`, `DoLogin`) |
| ServUO: pacotes | `Server/Network/Packets.cs` (`AccountLoginAck`, `AccountLoginRej`, `ALRReason`, `PlayServerAck`, `SupportedFeatures`, `CharacterList`, `CharacterListOld`, `CharacterListUpdate`, `DeleteResult`, `PopupMessage`, `ClientVersionReq`, `LoginConfirm`, `LoginComplete`, `MapChange`, `MapPatches`, `SetWarMode`, `CurrentTime`) |
| ServUO: estado do cliente | `Server/Network/NetState.cs` (`ProtocolChanges`, `Version`, `IsEnhancedClient`, `CompressionEnabled`), `Server/Network/Compression.cs` |
| ServUO: contas e lista de servidores | `Scripts/Accounting/AccountHandler.cs` (`EventSink_AccountLogin`, `EventSink_GameLogin`, `EventSink_DeleteRequest`), `Scripts/Misc/ServerList.cs` (`EventSink_ServerList`), `Scripts/Mobiles/PlayerMobile.cs` (`CheckLightLevels`) |
| ModernUO: handlers | `Projects/UOContent/Network/Packets/IncomingAccountPackets.cs` (`LoginServerSeed`, `AccountLogin`, `CompleteAccountLogin`, `PlayServer`, `RegisterAuthId`, `ConsumeAuthId`, `GameLogin`, `PlayCharacter`, `CreateCharacter`, `DeleteCharacter`, `ClientVersion`, `ClientType`, `LoginTimer`, `DoLogin`) |
| ModernUO: pacotes | `Projects/Server/Network/Packets/OutgoingAccountPackets.cs` (`SendPlayServerAck`, `SendLoginConfirmation`, `SendLoginComplete`, `SendClientVersionRequest`, `SendCharacterDeleteResult`, `SendCharacterListUpdate`) |
| ModernUO: máquina de estados e criptografia | `Projects/Server/Network/NetState/NetState.cs` (`ProtocolState`, tratamento de seed, envio com compressão e criptografia), `Projects/Server/Network/Encryption/` (`EncryptionManager`, `LoginEncryption`, `LoginKeys`, `GameEncryption`, `TwofishEngine`), `Projects/Server/Network/NetworkCompression.cs` |
| Source-X | `src/network/CNetworkInput.cpp` (seed e primeira mensagem), `src/network/CNetworkOutput.cpp` (compressão e criptografia de saída), `src/network/receive.cpp` (`PacketServersReq`, `PacketServerSelect`, `PacketCharListReq`, `PacketCharPlay`, `PacketCreate`, `PacketCreateHS`, `PacketCreateNew`, `PacketCharDelete`, `PacketClientVersion`), `src/network/send.cpp` (`PacketServerList`, `PacketServerRelay`, `PacketLoginError`, `PacketDeleteError`, `PacketCharacterListUpdate`, `PacketEnableFeatures`, `PacketPlayerStart`, `PacketLoginComplete`, `PacketWarMode`, `PacketGameTime`, `PacketEnableMapDiffs`, `PacketKREncryption`), `src/game/clients/CClientLog.cpp` (`Login_ServerList`, `Login_Relay`, `addRelay`, `addLoginErr`, `xProcessClientSetup`, `xCanEncLogin`), `src/game/clients/CClientMsg.cpp` (`Setup_ListReq`, `Setup_Play`, `Setup_Start`, `Setup_Delete`, `LogIn`, `addPlayerStart`, `addDeleteErr`), `src/common/crypto/` (`CCrypto*`, `CHuffman`) |
| ClassicUO | `src/ClassicUO.Client/Game/Scenes/LoginScene.cs` (`OnNetClientConnected`, `ServerListReceived`, `SelectServer`, `HandleRelayServerPacket`, `ParseCharacterList`, `ParseCities`, `ReceiveCharacterList`, `UpdateCharacterList`, `HandleErrorCode`, `HandleLoginDelayPacket`), `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_Seed`, `Send_Seed_Old`, `Send_FirstLogin`, `Send_SelectServer`, `Send_SecondLogin`, `Send_CreateCharacter`, `Send_DeleteCharacter`, `Send_SelectCharacter`, `Send_ClientVersion`, `Send_ClientType`, `Send_GameWindowSize`, `Send_Language`, `Send_ClientViewRange`), `src/ClassicUO.Client/Network/PacketHandlers.cs` (`EnterWorld`, `LoginComplete`, `ServerListReceived`, `ReceiveServerRelay`, `ReceiveCharacterList`, `UpdateCharacterList`, `ReceiveLoginRejection`, `LoginDelay`, `EnableLockedFeatures`, `ClientVersion`), `src/ClassicUO.Client/Network/NetClient.cs` (`Send`, `ProcessEncryption`), `src/ClassicUO.Client/Network/Encryption/Encryption.cs` (`EncryptionHelper`), `src/ClassicUO.Client/Game/Data/ServerErrorMessages.cs` |
