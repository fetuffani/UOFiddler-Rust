# Barcos e Smooth Sailing

Fluxo multi-pacote de movimento de barcos. O barco é um multi (`BaseMulti`) com itens e mobiles "a bordo". Um passo de barco precisa mover o multi, o timoneiro, as pranchas, o porão, todos os itens e todos os mobiles ao mesmo tempo. O Smooth Sailing (clientes High Seas) faz isso com um único pacote `0xF6`; clientes anteriores recebem uma atualização individual por entidade.

Convenções: inteiros big-endian; offsets a partir do byte 0 (id). `S->C` = servidor para cliente, `C->S` = cliente para servidor. Ids e tamanhos gerais em [packets.md](packets.md).

## 1. Visão geral e quem recebe o quê

| Cliente | Movimento do barco | Pilotagem pelo mouse |
|---|---|---|
| Anterior a 7.0.9.0 | Atualização individual de cada entidade (itens e mobiles) a cada passo | Não |
| 7.0.9.0 ou superior (High Seas, flag `HighSeas`) | `0xF6` único por passo | `0xBF/0x33` (C->S) |
| Enhanced / KR | tratado como High Seas (Sphere) | idem |

Detecção:

- ServUO/ModernUO: `NetState.HighSeas`, ativado com a versão do cliente `>= 7.0.9.0` (`ProtocolChanges.Version7090`).
- Sphere: `MINCLIVER_HS = 7000900` (7.0.9.0) ou cliente Enhanced, e a opção `OF_NoSmoothSailing` desligada.
- ModernUO usa o caminho de passo único (`Teleport`) para todos quando o servidor roda com expansão abaixo de HS (`NewBoatMovement => Core.HS`).
- ServUO mantém `NewBoatMovement = true` fixo; quando o deslocamento de um passo ultrapassa 1 tile em algum eixo (por exemplo, wrap de mapa), usa `Teleport`.

O cliente ClassicUO processa `0xF6` sempre; a animação suave só ocorre se a opção `UseSmoothBoatMovement` estiver ligada (padrão desligada). Desligada, o cliente apenas reposiciona o multi e cada entidade da lista nas coordenadas finais.

## 2. Sequências

### 2.1 Passo de movimento (servidor, High Seas)

1. O temporizador do barco dispara (`MoveTimer`) e `Move` calcula a direção de deslocamento `d` e o deslocamento `(xOffset, yOffset)`; cada tile do passo passa por `CanFit`.
2. Se o passo é de no máximo 1 tile por eixo, o servidor monta uma única vez o `0xF6` com as posições finais (posição atual + deslocamento) do barco e de todas as entidades a bordo (menos o próprio barco).
3. S->C `0xF6` para cada cliente High Seas que `CanSee(barco)` e está dentro do alcance de atualização do barco (`InRange(Location, GetUpdateRange(m))`).
4. Só depois o servidor altera as posições reais das entidades e do barco. Durante essa alteração, uma flag por entidade (`NoMoveHS`) suprime os pacotes de movimento individuais para clientes High Seas.
5. Clientes que não são High Seas recebem as atualizações individuais normais (item: world packet `0x1A`/`0xF3`; mobile: `0x77`/`0x78`/`0xD3`; o próprio jogador: `0x20` e reinício da sequência de movimento).
6. ServUO ainda chama `SendContainerPacket`: para clientes High Seas fora do alcance de atualização do barco mas dentro do raio de radar global, envia `0x1D` do barco e de seus itens e um `0xF7` com todas as entidades (cada uma como um `0xF3`).

### 2.2 Passo legado (cliente < 7.0.9.0 ou servidor sem HS)

1. Mesmo cálculo de `Move`.
2. `Teleport(xOffset, yOffset, 0)`: cada entidade a bordo (exceto componentes) e o barco têm `Location` alterada.
3. A mudança de `Location` gera os pacotes habituais por entidade para cada cliente em alcance; para o jogador a bordo, `0x20` (ou o formato antigo) e `Sequence = 0`.
4. Em ModernUO sem HS os intervalos são mais longos e andam 3 tiles por passo na velocidade rápida (ver 5).

### 2.3 Piloto (HS) e pilotagem pelo mouse

1. C->S `0x06` (duplo clique no leme: `TillerMan`, `ShipWheel`).
2. O servidor valida: jogador a bordo, não voando, não montado (a não ser que a montaria seja a do barco), barco não afundado (`Scuttled`). Se já há piloto e ele é o dono do barco, outro jogador não assume o controle; se o piloto é outro jogador, quem usa o leme assume o controle; se quem usa já é o piloto, deixa de pilotar.
3. S->C item virtual equipado no jogador: gráfico `0x3E96` na camada de montaria (`Layer.Mount`), ligado ao barco (`BoatMountItem`); o jogador passa a ter `Direction` igual ao `Facing` do barco; mensagem cliloc `1116727`.
4. O cliente detecta o gráfico `0x3E96` na camada de montaria e passa a se considerar "pilotando": cliques de mouse deixam de gerar `0x02` (andar) e geram `0xBF/0x33`.
5. C->S `0xBF/0x33` repetido enquanto o botão do mouse se mantém; solta o botão = `0xBF/0x33` com velocidade 0.
6. Fim: novo duplo clique no leme, desconexão ou morte (`ForceRemovePilot`); o servidor remove o item virtual e envia a mensagem cliloc `1149592`.

### 2.4 Colocação do barco (deed ou barco atracado)

1. C->S `0x06` (duplo clique no deed).
2. S->C gump de escolha de direção (ServUO, `BoatPlacementGump`): botões 1 = oeste, 2 = norte, 3 = sul, 4 = leste.
3. C->S `0xB1` com o botão escolhido.
4. S->C `0x99` Multi Placement (alvo com o multi do barco naquela direção).
5. C->S `0x6C` Target Response com o ponto escolhido.
6. O servidor valida a região (dungeon, casa, Champion spawn) e `CanFit`; cria o barco e envia os objetos normais (`0xF3`/`0x1A`, mobiles).

## 3. Pacotes

### 3.1 `0xF6` Boat Moving (S->C, `var`)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xF6` | |
| 1 | u16 | tamanho total | `18 + 10 * count` |
| 3 | u32 | serial do barco | |
| 7 | u8 | velocidade | ver 5.1 |
| 8 | u8 | direção de deslocamento | enum de direção (0 = N, 1 = NE, 2 = E, 3 = SE, 4 = S, 5 = SW, 6 = W, 7 = NW) |
| 9 | u8 | direção para a qual o barco aponta (facing) | só 0, 2, 4, 6 |
| 10 | i16 | x do barco | posição final |
| 12 | i16 | y do barco | posição final |
| 14 | i16 | z do barco | |
| 16 | u16 | `count` | entidades abaixo (o barco não entra) |
| 18 | entrada x `count` | | 10 bytes cada |

Entrada:

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u32 | serial (item ou mobile) |
| 4 | i16 | x final |
| 6 | i16 | y final |
| 8 | i16 | z final |

A lista inclui todos os mobiles e itens a bordo, incluindo as peças do próprio barco (timoneiro, pranchas, porão). Os valores são posições finais, não deltas.

Comportamento do cliente (ClassicUO): ignora o pacote se o serial do barco não é um item conhecido; entradas cujo serial não é conhecido são puladas; a entrada do serial do próprio jogador atualiza o centro do alcance de visão. Em modo suave empilha o passo (descarta os mais antigos quando há mais de 5 pendentes por barco) e anima as entidades como deslocamento relativo ao barco; em modo normal aplica `SetInWorldTile(x, y, z)` ao multi e a cada entidade e regenera a casa/multi.

### 3.2 `0xF7` Packet container (S->C, `var`)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0xF7` |
| 1 | u16 | tamanho total |
| 3 | u16 | `count` |
| 5 | ... | `count` pacotes `0xF3` completos (id incluído), 26 bytes cada |

Cada `0xF3` interno usa o layout normal do Object Info (SA/novo): `u8 0xF3`, `u16 0x0001`, `u8 tipo` (`0x00` item, `0x01` mobile, `0x02` multi, `0x03` item danificável), `u32 serial`, `u16 gráfico`, `u8 0`, `u16 quantidade`, `u16 quantidade`, `u16 x & 0x7FFF`, `u16 y & 0x3FFF`, `i8 z`, `u8 luz`, `u16 matiz`, `u8 flags`, `u16 0`. O cliente só aceita `0xF3` dentro do `0xF7`; outro id interrompe o processamento. `0xF7` não consta de [packets.md](packets.md) (a tabela cobre até `0xF6`).

### 3.3 `0xBF/0x33` Multi mouse movement (C->S, 12 bytes)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xBF` | |
| 1 | u16 | `0x000C` | |
| 3 | u16 | `0x0033` | subcomando |
| 5 | u32 | serial do jogador | |
| 9 | u8 | direção | direção absoluta desejada |
| 10 | u8 | direção (repetida) | |
| 11 | u8 | velocidade | `0` = parar, `1` = devagar (andar), `2` = rápido (correr) |

O ClassicUO envia a direção duas vezes e usa velocidade 2 quando a distância do mouse ao centro é >= 190 pixels; ao soltar o botão envia velocidade 0 com a direção atual do personagem. ServUO ignora o segundo byte de direção; Sphere lê os dois (o primeiro como direção de proa, o segundo como direção de movimento) e limita velocidades acima de 2 a 2.

O servidor só aceita o comando se o jogador é o piloto atual (ServUO: está montado em `BaseBoat`; Sphere: está na região do barco e é o `m_Pilot`).

### 3.4 `0x99` Multi Placement (S->C)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0x99` |
| 1 | u8 | `allowGround` |
| 2 | u32 | id do cursor (`TargetID`) |
| 6 | u8 | flags |
| 7 | u8[11] | zeros |
| 18 | u16 | índice do multi |
| 20 | i16 | offset x |
| 22 | i16 | offset y |
| 24 | i16 | offset z |
| 26 | u8[4] | zeros (somente clientes High Seas) |

Tamanho: 30 bytes para clientes High Seas, 26 bytes para os anteriores (ServUO `MultiTargetReqHS` e `MultiTargetReq`). A resposta é o `0x6C` de 19 bytes.

### 3.5 `0x6C` Target Response (C->S, 19 bytes)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0x6C` |
| 1 | u8 | tipo (`0` objeto, `1` posição) |
| 2 | u32 | id do cursor |
| 6 | u8 | flags |
| 7 | u32 | serial |
| 11 | i16 | x |
| 13 | i16 | y |
| 15 | i16 | z |
| 17 | u16 | gráfico |

Cancelamento: `x = y = -1` e serial inválido.

### 3.6 `0xAD` Unicode Speech Request: comandos de voz (C->S)

Os comandos do barco (`forward`, `stop`, `turn left` etc.) chegam como palavras-chave do cliente (tabela `speech.mul`), não como texto interpretado pelo servidor.

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xAD` | |
| 1 | u16 | tamanho | |
| 3 | u8 | tipo | bit `0xC0` (`Encoded`) indica palavras-chave |
| 4 | u16 | matiz | |
| 6 | u16 | fonte | |
| 8 | char[4] | idioma | |
| 12 | u16 | `(contagem << 4) \| (primeiros 4 bits do primeiro id)` | só se `Encoded` |
| 14 | ... | ids de 12 bits empacotados | alternando u8 e u16, 1.5 byte por id |
| fim | UTF-8 (Encoded) ou UTF-16BE terminado em 0 | texto | |

O servidor aceita no máximo 50 palavras-chave, texto de 1 a 128 caracteres.

### 3.7 Pacotes auxiliares

| Pacote | Uso |
|---|---|
| `0x1D` Delete Object (5 bytes) | remove barco e itens do cliente antes do `0xF7` (ServUO) |
| `0x2E` Equip (15 bytes) | item equipado em camada; o item virtual `0x3E96` do piloto chega ao cliente pelo caminho comum de equipamento (`0x2E` para o dono, `0x78`/`0xD3` para os demais) |
| `0xB0`/`0xDD` | gump de colocação e gump de dry dock |
| `0x9A`/`0xC2` | prompt de texto para renomear o barco |
| `0xBF/0x19` tipo 5 (19 bytes) | "BoatDriverLock" definido no ServUO, não usado no fluxo |

## 4. Estado do barco

- **Facing** (proa): N, E, S ou W. Cada orientação tem seu próprio id de multi (`NorthID`, `EastID`, `SouthID`, `WestID`); girar troca o `ItemID` do barco.
- **Direção de deslocamento**: sem piloto, `(facing + direçãoRelativa) & 7`; com piloto (mouse), direção absoluta.
- **Direções relativas**: `Forward` = N (0), `ForwardRight` = NE (1), `Right` = E (2), `BackwardRight` = SE (3), `Backward` = S (4), `BackwardLeft` = SW (5), `Left` = W (6), `ForwardLeft` = NW (7).
- **Drift**: movimento em direção diferente de `Forward`, `ForwardLeft`, `ForwardRight`: o barco desliza sem girar. Usa o mesmo tempo de passo que o movimento normal em ModernUO HS; em ServUO usa o intervalo de drift (ver 5).
- **Virada**: múltiplos de 90 graus (`+2`, `-2`, `-4` em unidades de direção). O comando inicia um temporizador de 0,5 s; se o barco estava andando e não há piloto, ele retoma o movimento na mesma direção depois de virar. Com piloto, a direção do `Turn` é absoluta. Com mouse, o servidor só vira se a velocidade for 2 e a direção pedida for diferente de proa, proa+1 e proa-1; para direções diagonais usa a direção cardinal anterior.
- **Parada por obstáculo**: se o primeiro tile falha em `CanFit`, o passo é cancelado (cliloc `501424`); se só um tile posterior falha, o passo é encurtado.
- **Rotação**: `SetFacing` realoca timoneiro, porão, pranchas e todos os que estão a bordo por rotação de coordenadas; esse caminho **não** usa `0xF6`: cada entidade é reenviada pelo caminho comum de atualização, e o barco troca de `ItemID`.
- **Wrap do mapa**: se o barco sai do retângulo do mapa, as coordenadas são enroladas e o passo passa a ter deslocamento maior que 1: usa `Teleport`, sem `0xF6`.

## 5. Velocidade, intervalo e deslocamento

### 5.1 Byte de velocidade do `0xF6`

| Valor | Significado | Intervalo do cliente (ClassicUO) |
|---|---|---|
| `0x01` | um passo (comando `OneMove`) | 500 ms (valor padrão) |
| `0x02` | lento / bote (rowboat) / drift lento | 1000 ms |
| `0x03` | normal | 500 ms |
| `0x04` | rápido | 250 ms |
| > `0x04` | | `valor * 10` ms |

Esses valores não coincidem com o byte de velocidade do pedido `0xBF/0x33` (0/1/2).

### 5.2 ServUO (padrão High Seas)

Intervalos: `FastInterval = 250 ms`, `NormalInterval = 500 ms`, `SlowInterval = 1000 ms`. Todos os passos andam 1 tile.

| Situação | Intervalo | Byte `0xF6` |
|---|---|---|
| Rápido, para a frente | 250 ms | `0x04` |
| Rápido, drift | 500 ms | `0x03` |
| Lento, drift | 1000 ms | `0x02` |
| Lento, para a frente | 500 ms | `0x03` |
| `OneMove` | 250 ms (frente) ou 500 ms (drift) | `0x01` |
| Mouse, velocidade 1 | 1000 ms | `0x02` |
| Mouse, velocidade 2 | 250 ms | `0x04` |
| Bote, barco danificado (`DamageTaken >= Moderately`) ou em reparo de emergência | 1000 ms | `0x02` |

Comando de mouse com velocidade 0 (ou qualquer intervalo 0): `StopMove`.

### 5.3 ModernUO

| Modo | Slow | Fast | Drift slow | Drift fast | Tiles por passo (fast) |
|---|---|---|---|---|---|
| HS | 0,50 s | 0,25 s | 0,50 s | 0,25 s | 1 |
| Sem HS | 0,75 s | 0,75 s | 1,50 s | 0,75 s | 3 |

Byte `0xF6`: `0x04` para rápido, `0x03` para lento, `0x01` para `OneMove`.

### 5.4 Sphere

`_eSpeedMode` (1 a 4) é o byte enviado no `0xF6`: `0x01` um tile, `0x02` bote, `0x03` lento, `0x04` rápido. O atraso do próximo passo é `period * 100 ms` no modo lento e metade disso nos demais; `period` e `tiles` vêm da definição do barco (`ShipSpeed`).

## 6. Regras do servidor

1. Enviar o `0xF6` antes de alterar as posições reais; caso contrário o cliente HS também recebe os pacotes individuais e vê o barco andar duas vezes.
2. Só enviar `0xF6` para clientes que já conhecem o barco e suas entidades. Entidades que entram no campo de visão com o passo (inclusive quando o cliente está a bordo) continuam precisando do world packet normal.
3. Passo com deslocamento maior que 1 em algum eixo vai pelo caminho de `Teleport`, sem `0xF6`.
4. Com `0xF6` é obrigatório suprimir os pacotes de movimento individuais para os clientes HS durante a alteração das posições (ServUO/ModernUO: `NoMoveHS`). Clientes anteriores continuam recebendo os individuais.
5. Para clientes anteriores a 7.0.9.0, quem está a bordo deve receber o `0x20` (ou o formato antigo) e ter a sequência de movimento zerada a cada passo.
6. Pedidos `0xBF/0x33` de quem não é o piloto devem ser ignorados.
7. Sphere ignora um pedido de mouse enquanto o temporizador do barco está armado (`fWheelMove && IsTimerSet()`), o que limita a taxa de passos ao intervalo do barco.
8. Ao parar (velocidade 0), cancelar o temporizador de movimento; ao receber nova direção, substituir o temporizador.
9. Remoção do barco, morte ou desconexão do piloto remove o item virtual de montaria.
10. Um barco só existe uma vez por jogador sob High Seas (mensagem `1116758`), exceto botes.

## 7. Comandos de cliente e interfaces

### 7.1 Comandos de voz (ids de palavra-chave tratados em `BaseBoat.OnSpeech`)

Aceitos apenas de quem está a bordo, tem comando sobre o barco e quando não há piloto.

| Id | Ação |
|---|---|
| `0x42` | definir nome |
| `0x43` | remover nome |
| `0x44` | dizer o nome |
| `0x45`, `0x68` | seguir em frente (rápido) |
| `0x46` | para trás |
| `0x47` | esquerda |
| `0x48` | direita |
| `0x4B` | frente-esquerda |
| `0x4C` | frente-direita |
| `0x4D` | trás-esquerda |
| `0x4E` | trás-direita |
| `0x4F`, `0x69` | parar |
| `0x50` a `0x57` | mesmos deslocamentos de `0x47`, `0x48`, `0x45`, `0x46`, `0x4B`, `0x4C`, `0x4E`, `0x4D` em velocidade lenta |
| `0x58` a `0x5F` | passo único (`OneMove`) |
| `0x49`, `0x65` | virar à direita |
| `0x4A`, `0x66` | virar à esquerda |
| `0x67` | dar meia-volta |
| `0x6A` | baixar âncora (apenas sem HS) |
| `0x6B` | levantar âncora (apenas sem HS) |
| `0x60` | próximo ponto de navegação |
| `0x61` | iniciar o curso |
| `0x62` | continuar o curso |
| `0x63` | ir ao ponto indicado no texto (`goto`) |
| `0x64` | passo único até o ponto (`single`) |
| `0x0F` | rastrear embarcações (`start` / `stop` no texto; só HS) |

A ordem de `0x50` a `0x57` em ServUO é: esquerda, direita, frente, trás, frente-esquerda, frente-direita, trás-direita, trás-esquerda.

Sphere expõe o mesmo comportamento como verbos de script (`SHIPFORE`, `SHIPBACK`, `SHIPTURN`, `SHIPSTOP`, `SHIPANCHORDROP`, `SHIPANCHORRAISE`, `SHIPFACE`, `SHIPMOVE`, `SHIPGATE` e outros), ligados a definições de fala.

### 7.2 Gumps, alvos e prompts

| Interface | Pacotes | Observação |
|---|---|---|
| Direção de colocação | gump (`0xB0`/`0xDD`) + `0xB1` | ServUO `BoatPlacementGump`, botões 1 a 4 (oeste, norte, sul, leste); botão 0 cancela |
| Posição de colocação | `0x99` + `0x6C` | multi-alvo, `InternalTarget` |
| Dry dock | gump `ConfirmDryDockGump` + `0xB1` | botão 2 = continuar, 1 = cancelar; opção aparece no menu de contexto do timoneiro |
| Renomear | prompt de texto (`0x9A`/`0xC2`) | `RenameBoatPrompt` -> `EndRename` |
| Prancha | duplo clique (`0x06`) | `Plank.OnDoubleClick` abre/fecha; bloqueado enquanto pilota |

## 8. Exemplo

Barco serial `0x40000010`, em `(1500, 1600, -5)`, apontando para norte, andando para norte em velocidade rápida; a bordo, o jogador `0x00000007` e o timoneiro `0x40000011`. O passo desloca `y` em -1:

```
F6 00 26                  tamanho = 18 + 2*10 = 38
40 00 00 10               serial do barco
04                        velocidade rápida
00                        direção de deslocamento: N
00                        facing: N
05 DC                     x = 1500
06 3F                     y = 1599
FF FB                     z = -5
00 02                     2 entidades
00 00 00 07  05 DC 06 3E  FF FE      jogador em (1500, 1598, -2)
40 00 00 11  05 DC 06 3D  FF FB      timoneiro em (1500, 1597, -5)
```

Pedido do piloto (mouse, correr, sentido definido pelo cliente):

```
BF 00 0C 00 33  00 00 00 07  00 00 02
              serial   dir dir vel
```

Parada ao soltar o botão:

```
BF 00 0C 00 33  00 00 00 07  00 00 00
```

## 9. Armadilhas

- A lista do `0xF6` não inclui o barco; o `count` e o tamanho total não devem contá-lo. ModernUO e ServUO incluem, em compensação, o timoneiro, as pranchas e o porão na lista.
- Coordenadas são finais, não deltas. Enviar as antigas faz o cliente andar para trás.
- O byte de velocidade do `0xF6` e o do pedido `0xBF/0x33` usam escalas diferentes.
- Mandar `0xF6` sem que o cliente conheça uma entidade a ignora silenciosamente: ela fica parada.
- Misturar `0xF6` e pacotes de movimento individuais para o mesmo cliente gera passo duplo.
- Passos maiores que 1 tile por eixo (ou com wrap) não devem usar `0xF6`; use remoção/recriação (`Teleport`).
- Virar não passa por `0xF6`: o `ItemID` do multi muda e todas as entidades são reposicionadas.
- Sem o item virtual `0x3E96` na camada de montaria o cliente continua enviando `0x02` e não `0xBF/0x33`.
- O cliente assume que o centro do alcance acompanha a entrada do jogador no `0xF6`; omitir o jogador da lista deixa a visão parada.
- A mesma velocidade do `0xF6` determina a duração da animação no cliente; se ela não corresponde ao intervalo real do servidor há solavancos ou atraso.
- O `0xF7` é um contêiner de `0xF3`, não de pacotes arbitrários.

## 10. ServUO x Sphere

| Aspecto | ServUO / ModernUO | Sphere |
|---|---|---|
| Quem recebe `0xF6` | HS (7.0.9.0+) em alcance de atualização | HS (7.0.9.0+) ou Enhanced, a menos que `OF_NoSmoothSailing` |
| Entidades listadas | tudo a bordo exceto o barco | todos os objetos de `ListObjs` exceto o primeiro (o barco) |
| Posição do barco no pacote | `boat + offset` | posição já atualizada |
| Ordem | envia antes de mover | move antes, e então envia a cada cliente |
| Cliente não HS | `Teleport` (item/mobile individual) | `addItem`/`addCharMove`/`addChar` por entidade |
| Pedido de mouse | `BoatMovementRequest` registrado em `0xBF/0x33`; usa a direção do primeiro byte | `PacketWheelBoatMove`; lê proa e movimento; ignora enquanto o temporizador está armado |
| Piloto | item `0x3E96` em `Layer.Mount` | memória `ITEMID_SHIP_PILOT` (`0x3E96`) em `LAYER_HORSE` |
| Velocidade | `clientSpeed` 1 a 4 conforme combinação rápido/lento/drift | `_eSpeedMode` 1 a 4 |
| Comandos | palavras-chave de fala em `OnSpeech` | verbos de script `SHIP*` |
| `0xF7` | enviado a clientes HS fora do alcance | definido (`PacketContainer`), sem uso na rota de movimento observada |

## 11. Não especificado

- Tabela de direção usada pelo ClassicUO para o byte de direção do `0xBF/0x33` (o servidor trata o byte como enum de direção).
- Significado do subcomando `0xBF/0x19` tipo 5 com o valor emitido por `BoatDriverLock`: o cliente trata como animação do mobile; o ServUO não o usa.
- Pacote `0x2E` `BoatEquipPacket` (definido em ServUO, nunca enviado).
- Quantidade máxima de entidades a bordo por Sphere (`MAX_MULTI_LIST_OBJS`).

## 12. Referência de implementação

| Peça | Arquivo / símbolo |
|---|---|
| ServUO: lógica do barco | `Scripts/Multis/Boats/BaseBoat.cs`: `Move`, `DoMovement`, `StartMove`, `StopMove`, `OneMove`, `StartTurn`, `Turn`, `SetFacing`, `Teleport`, `OnSpeech`, `GetMovementInterval`, `OnMousePilotCommand`, `LockPilot`, `RemovePilot`, `ForceRemovePilot`, `SendContainerPacket`, `MoveBoatHS`, `PacketContainer`, `GetEntitiesOnBoard` |
| ServUO: pedido do mouse | `Scripts/Services/Expansions/High Seas/Network/PacketHandlers.cs` (`BoatMovementRequest.MultiMouseMovementRequest`) |
| ServUO: pacotes HS | `Scripts/Services/Expansions/High Seas/Network/Packets.cs` (`BoatDriverLock`, `BoatEquipPacket`) |
| ServUO: piloto e interfaces | `Scripts/Services/Expansions/High Seas/Multis/BoatMountItem.cs`, `ShipWheel.cs`, `Scripts/Multis/Boats/TillerMan.cs`, `Plank.cs`, `ConfirmDryDockGump.cs`, `RenameBoatPrompt.cs`, `BaseBoatDeed.cs`, `Scripts/Services/Expansions/High Seas/Gumps/ShipPlacementGump.cs` |
| ServUO: flags e suspensão | `Server/Network/NetState.cs` (`ProtocolChanges.HighSeas`, `Version7090`), `Server/Mobile.cs` (`SetLocation`, `NoMoveHS`), `Server/Item.cs` (`NoMoveHS`), `Server/Targeting/MultiTarget.cs`, `Server/Network/Packets.cs` (`MultiTargetReq`, `MultiTargetReqHS`), `Server/Network/PacketHandlers.cs` (`UnicodeSpeech`, `TargetResponse`) |
| ModernUO | `Projects/UOContent/Multis/Boats/BoatPackets.cs` (`CreateMoveBoatHS`, `SendMoveBoatHS`, `SendMoveBoatHSUsingCache`, `SendDisplayBoatHS`), `Projects/UOContent/Multis/Boats/BaseBoat.cs` (`Move`, `StartMove`, `OneMove`, `OnSpeech`, `GetMovingEntities`, `Teleport`, `SetFacing`), `Projects/Server/Network/Packets/PacketContainerBuilder.cs`, `Projects/Server/Network/Packets/OutgoingEntityPackets.cs` (`CreateWorldEntity`), `Projects/Server/Network/NetState/ProtocolChanges.cs` |
| Sphere | `src/network/send.cpp` (`PacketMoveShip`, `PacketContainer`), `src/network/receive.cpp` (`PacketWheelBoatMove`), `src/game/components/CCMultiMovable.cpp` (`MoveDelta`, `SetMoveDir`, `SetNextMove`, `SetPilot`, `ListObjs`, `r_Verb`), `src/game/components/CCMultiMovable.h`, `src/game/uo_files/uofiles_enums.h` (`ShipMovementType`, `ShipMovementSpeed`), `src/common/sphereproto.h` (`MINCLIVER_HS`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`BoatMoving`, `PacketList`), `src/ClassicUO.Client/Game/Managers/BoatMovingManager.cs`, `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_MultiBoatMoveRequest`), `src/ClassicUO.Client/Game/GameObjects/Mobile.cs` (`IsDrivingBoat`), `src/ClassicUO.Client/Game/Scenes/GameSceneInputHandler.cs` |
