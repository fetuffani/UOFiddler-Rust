# Custom House Designer (personalização de casas)

Fluxo multi-pacote do editor de casas customizadas (AOS em diante). O servidor coloca o cliente em "modo de design", envia a estrutura inteira da casa em um pacote comprimido (`0xD8`) e recebe cada edição como um comando curto (`0xD7`). Este documento descreve os pacotes, a estrutura de dados do `0xD8`, as regras de validação e as respostas esperadas.

Convenções: inteiros big-endian na rede; offsets contados a partir do byte 0 (o byte de id). `S->C` = servidor para cliente, `C->S` = cliente para servidor. Tamanhos e ids de pacotes de nível mais alto estão em [packets.md](packets.md).

## 1. Visão geral e habilitação

| Elemento | Pacote | Direção | Observação |
|---|---|---|---|
| Entrar/sair do modo de design | `0xBF` subcomando `0x20` | S->C | Tipo `0x04` = begin, `0x05` = end |
| Número de revisão da casa | `0xBF` subcomando `0x1D` | S->C | Chave de cache do cliente |
| Pedido de detalhes da casa | `0xBF` subcomando `0x1E` | C->S | Enviado pelo cliente quando a revisão difere |
| Conteúdo completo da casa | `0xD8` | S->C | Comprimido (zlib), por plano |
| Comandos de edição | `0xD7` | C->S | Envelope "encoded" com subcomando de 16 bits |

Habilitação:

- Sphere (`CItemMultiCustom`): `0xBF/0x20` e `0xD8` só são enviados a cliente com versão >= 4.0.0 (`MINCLIVER_CUSTOMMULTI = 4000000`), KR ou Enhanced Client. Sem isso, `BeginCustomize` retorna sem fazer nada.
- ServUO/ModernUO: não há filtro por versão de cliente nos pacotes da casa; o que cada peça de construção é válida depende do flag de expansão do servidor (`ExpansionInfo.CustomHousingFlag`, valores `HousingFlags`: AOS `0x10`, SE `0x40`, ML `0x80`, Crystal `0x200`, SA `0x10000`, HS `0x20000`, Gothic `0x40000`, Rustic `0x80000`, Jungle `0x100000`, Shadowguard `0x200000`, TOL `0x400000`).
- O comando de telhado (`0xD7/0x13`) só é aceito do jogador se o servidor roda com SE ou superior (`Core.SE`); GMs sempre podem.
- O cliente só processa o `0xD8` se já conhece o item da fundação (multi) e suas dimensões em `multi.mul`; `0xD8` para serial desconhecido é descartado. Envie o objeto multi (`0xF3`/`0x1A`) antes do `0xD8`.

## 2. Envelopes

### 2.1 `0xBF` (General Information)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | tamanho total |
| 3 | u16 | subcomando |
| 5 | ... | dados do subcomando |

### 2.2 `0xD7` (Encoded Command, C->S)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xD7` | |
| 1 | u16 | tamanho total | pacote `var` |
| 3 | u32 | serial da entidade alvo | o cliente envia o serial do próprio personagem; o servidor resolve com `World.FindEntity` e repassa ao handler |
| 7 | u16 | subcomando | ver 3.5 |
| 9 | ... | argumentos "encoded" | ver abaixo |
| fim | u8 | `0x0A` | terminador enviado pelo cliente; o servidor não o lê |

Argumentos "encoded", lidos em sequência pelo `EncodedReader`:

| Tag (u8) | Valor | Uso |
|---|---|---|
| `0x00` | i32 | inteiro (usado por todos os comandos de casa) |
| `0x02` | u16 comprimento + string UTF-16 | texto (não usado pelo designer) |
| `0x03` | i16 x, i16 y, u8 z | ponto (não usado pelo designer) |

Se a tag lida não for a esperada, o leitor devolve 0/vazio. Valores negativos trafegam em complemento de dois de 32 bits (`x = -2` vira `FF FF FF FE`).

O comando de nível (`0x12`) é enviado pelo cliente como `u32 0` seguido de `u8 andar`: lido como tag `0x00` + i32, o resultado é o número do andar.

## 3. Pacotes

### 3.1 `0xBF/0x20` Begin/End customization (S->C, 17 bytes)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xBF` | |
| 1 | u16 | `0x0011` | |
| 3 | u16 | `0x0020` | subcomando |
| 5 | u32 | serial da fundação | |
| 9 | u8 | tipo | `0x04` begin, `0x05` end |
| 10 | u16 | `0x0000` | gráfico |
| 12 | u16 | `0xFFFF` | x |
| 14 | u16 | `0xFFFF` | y |
| 16 | u8 | `0xFF` | z |

Os tipos `1` (update), `2` (remove) e `3` (update multi pos) são lidos pelo cliente com o mesmo layout, mas sem efeito. Ao receber `0x04` o cliente abre o gump do designer (se já não estiver aberto); ao receber `0x05` fecha o gump do serial indicado.

### 3.2 `0xBF/0x1D` Design state general / revision (S->C, 13 bytes)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x000D` |
| 3 | u16 | `0x001D` |
| 5 | u32 | serial da casa |
| 9 | u32 | revisão |

Regra do cliente: se não possui a casa, se ela não é "custom" ou se `revisão != revisão em cache`, o cliente agenda um `0xBF/0x1E`. Se a revisão coincide, apenas regenera a casa a partir do cache local e não pede nada.

### 3.3 `0xBF/0x1E` Query design details (C->S, 9 bytes)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x0009` |
| 3 | u16 | `0x001E` |
| 5 | u32 | serial da casa |

Resposta: um `0xD8` com o estado visível (`CurrentState`) para espectadores, ou o estado de design (`DesignState`) se quem pergunta é o jogador que está editando esta fundação. O servidor só responde se o jogador está no mesmo mapa, dentro do alcance de atualização da fundação e se `CanSee`.

### 3.4 `0xD8` Custom House (S->C, `var`)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xD8` | |
| 1 | u16 | tamanho total | |
| 3 | u8 | tipo de compressão | `0x03` = zlib por plano (único valor emitido) |
| 4 | u8 | flag de resposta | `0x01` ou `0x00`; ServUO usa `0x01` nas respostas a comandos e `0x00` no commit/close; ModernUO e Sphere sempre `0x00`. O cliente lê e ignora |
| 5 | u32 | serial da casa | |
| 9 | u32 | revisão | |
| 13 | u16 | quantidade total de componentes (itens) | soma dos itens de todos os planos e blocos |
| 15 | u16 | comprimento do buffer | `1 + soma(4 + comprimento comprimido de cada bloco)`; vale `tamanho_total - 17` |
| 17 | u8 | quantidade de blocos | planos usados + blocos de offsets |
| 18 | ... | blocos | repetidos `quantidade de blocos` vezes |

Cada bloco:

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | índice do bloco | ver 4.2 |
| 1 | u8 | tamanho descomprimido (bits 0-7) | |
| 2 | u8 | tamanho comprimido (bits 0-7) | |
| 3 | u8 | `((descomprimido >> 4) & 0xF0) \| ((comprimido >> 8) & 0x0F)` | nibble alto = bits 8-11 do descomprimido; nibble baixo = bits 8-11 do comprimido |
| 4 | u8[] | dados zlib | `comprimido` bytes |

Os dois tamanhos têm 12 bits (máximo 4095). Como leitor, o cliente monta `dlen = b1 | ((b3 & 0xF0) << 4)` e `clen = b2 | ((b3 & 0x0F) << 8)`; o byte `b0` é dividido em nibble alto (modo) e nibble baixo (plano ou número do bloco).

### 3.5 `0xD7` subcomandos do designer (C->S)

| Sub | Nome | Argumentos (tag `0x00` + i32 cada) | Efeito |
|---|---|---|---|
| `0x02` | Backup | nenhum | Copia o design atual para o slot de backup |
| `0x03` | Restore | nenhum | Substitui o design atual pelo backup |
| `0x04` | Commit | nenhum | Abre o gump de confirmação de custo (ServUO) ou aplica (Sphere) |
| `0x05` | Delete component | `itemId`, `x`, `y`, `z` | Remove um componente |
| `0x06` | Build component | `itemId`, `x`, `y` | Adiciona parede, piso, janela, porta ou outro item |
| `0x0A` | (ação) | nenhum | O cliente tem o envio; ServUO registra como no-op |
| `0x0C` | Close | nenhum | Sai do modo de design sem commit |
| `0x0D` | Stairs | `multiId`, `x`, `y` | Adiciona uma escada (multi) |
| `0x0E` | Sync | nenhum | Pede reenvio completo (`0xD8`) |
| `0x0F` | (ação) | nenhum | ServUO registra como no-op |
| `0x10` | Clear | nenhum | Restaura a fundação vazia |
| `0x12` | Level | `andar` (1 a 4) | Troca de andar |
| `0x13` | Roof | `itemId`, `x`, `y`, `z` | Adiciona peça de telhado |
| `0x14` | Roof delete | `itemId`, `x`, `y`, `z` | Remove peça de telhado |
| `0x1A` | Revert | nenhum | Volta ao estado atualmente visível (descarta edições não commitadas) |

`x` e `y` são deslocamentos com sinal em relação à origem do multi da fundação (a posição do item). `itemId` é o id de item (art) exceto em `0x0D`, onde é o índice do multi (sem o offset `0x4000`; Sphere soma `ITEMID_MULTI` internamente).

Exemplos de tamanho: `0x06` = 25 bytes (`1+2+4+2 + 3*5 + 1`), `0x05` e `0x13`/`0x14` = 30 bytes, `0x0D` = 25 bytes, `0x12` = 15 bytes, comandos sem argumento = 10 bytes.

## 4. Estrutura de dados do `0xD8`

### 4.1 Sistema de coordenadas

- A origem é o multi da fundação. `xMin`, `yMin`, `xMax`, `yMax` são os limites do `MultiComponentList` do estado enviado. A fundação do servidor tem uma linha a mais no sul (`Height + 1`) que o multi do `multi.mul`: é onde ficam sign e signpost. `width = xMax - xMin + 1`, `height = yMax - yMin + 1` (já incluindo a linha extra).
- Altura (Z) por andar `L` (1 a `MaxLevels`): `z = (L - 1) * 20 + 7`, ou seja `7, 27, 47, 67`. A linha extra do sul e o chão da fundação ficam em `z = 0`.
- `MaxLevels` = 4 se `width >= 14` ou `height >= 14` do multi, senão 3 (ServUO). Sphere calcula o limite por `GetLevelCount`.

### 4.2 Planos e blocos (emissão de ServUO/ModernUO)

Cada componente `(itemId, ox, oy, oz)` recebe `x = ox - xMin`, `y = oy - yMin` e é classificado:

| `oz` | Plano base |
|---|---|
| 0 | 0 |
| 7 | 1 |
| 27 | 2 |
| 47 | 3 |
| 67 | 4 |
| outro | bloco de offsets |

`piso = (Height do tiledata do item <= 0)`. Em seguida:

| Plano | Condição | Dimensão da coluna `size` | Ajuste | Índice de bloco (`b0`) | Bytes descomprimidos |
|---|---|---|---|---|---|
| 0 | `oz = 0` | `height` | nenhum | `0x20 \| 0` | `width * height * 2` |
| 1 a 4 | `oz` em 7/27/47/67 e piso | `height - 2` | `x -= 1; y -= 1` | `0x20 \| plano` | `(width-1) * (height-2) * 2` |
| 5 a 8 | `oz` em 7/27/47/67 e não piso | `height - 1` | `plano += 4` | `0x20 \| plano` | `width * (height-1) * 2` |

Posição na grade: `index = (x * size + y) * 2`, escrevendo `itemId` como u16 big-endian. O vetor é column-major (x externo, y interno). Célula vazia = `0x0000`.

Um componente vai para o bloco de offsets (em vez da grade) quando `oz` não é um dos cinco valores, `x < 0`, `y < 0`, `y >= size` ou `index + 1 >= 0x400` (limite de 1024 bytes por grade).

Bloco de offsets (stairs/pieces fora da grade): cada entrada ocupa 5 bytes e o bloco comporta no máximo 750 entradas (3750 bytes brutos); entradas extras abrem outro bloco.

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u16 | `itemId` |
| 2 | i8 | x (relativo à origem do multi) |
| 3 | i8 | y |
| 4 | i8 | z |

Índice de bloco desses blocos: `9 + n` (`n` = 0, 1, ...), sem o bit `0x20`.

O cliente interpreta `b0` assim: nibble alto = modo (`2` = grade, `0` = lista de 5 bytes, `1` = lista de 4 bytes `id,x,y` com z derivado do plano); nibble baixo = plano. Plano 0 usa z = 0; planos 1 a 4 e 5 a 8 usam `z = ((plano - 1) mod 4) * 20 + 7`. Para modo 2 ele reconstrói `x = i / alturaColuna + offX` e `y = i % alturaColuna + offY`, com `alturaColuna` derivada das dimensões do multi local (plano 0: `maxY - minY + 2`; 1 a 4: `maxY - minY`; 5 a 8: `maxY - minY + 1`) e offX/offY = `minX/minY` (+1 nos planos 1 a 4). Isso exige que o servidor use as mesmas dimensões do multi do cliente (mais a linha extra do sul).

### 4.3 Compressão

Zlib com cabeçalho padrão (`78 9C` no nível default). Cada plano e cada bloco de offsets é comprimido separadamente. Se a compressão falhar o ServUO escreve o bloco com tamanho 0 (mantém o bloco no pacote); Sphere omite o plano e registra erro.

Blocos de planos ficam em ordem crescente (0..8) e os blocos de offsets depois deles. Planos sem componentes na grade não são emitidos.

### 4.4 Quais ids são "multi" e quais são "stair"

- **Multi**: o comando `0x0D` carrega o índice de um multi de escada (`stairs.txt`, colunas `MultiNorth`, `MultiEast`, `MultiSouth`, `MultiWest`). O servidor expande o multi em itens do `MultiComponentList` com offsets do multi mais `(x, y, z do andar atual)`, ignorando entradas com `itemId == 1`. Os multis `7668..7675` (TOL) não existem em `MultiData` e são expandidos por tabelas internas.
- **Stair (item)**: ids de blocos e degraus de escada em listas fixas (`m_BlockIDs`, `m_StairSeqs`, `m_StairIDs`). Sequência N, W, S, E; `IsStair(id, ref dir)` retorna a direção (0 = N, 1 = W, 2 = S, 3 = E). Servem apenas para remover a escada inteira (4 degraus) quando o cliente apaga um degrau.
- Os itens de escada ficam em `z` diferentes de 7/27/47/67 (offsets de 5 em 5) e por isso trafegam no bloco de offsets, não na grade.
- **Fixture**: portas e teleporters (faixas de ids em `DesignState.IsFixture`). Durante o design são componentes normais do `0xD8`; no commit o servidor os retira do multi e os cria como itens reais.

### 4.5 Revisão

- Cada fundação mantém `LastRevision` (persistido). Qualquer modificação do estado de design faz `Revision = ++LastRevision` e invalida o pacote `0xD8` em cache do estado.
- O cliente só consulta a revisão em `0xBF/0x1D`. Revisão igual = reutiliza o cache; diferente = `0xBF/0x1E`.
- Para o jogador que está editando, o servidor anuncia a revisão do estado de design; para os demais, a do estado visível.
- Sphere mantém quatro desenhos (`main`, `working`, `backup`, `revert`), cada um com `m_iRevision`; o commit só acontece se `working.revision != main.revision`; backup/restore/revert são ignorados se as revisões forem iguais (ou se a revisão do slot for 0).

## 5. Sequências de mensagens

### 5.1 Entrar no modo de design

1. C->S `0x06` (duplo clique no sign) ou `0xB1` (botão "Customize This House" do gump de casa; específico do servidor).
2. Servidor valida (vivo, fora de combate, dono, sem vendors alugados etc.), realoca itens e mobiles da casa para fora (`BanLocation`) e cria o contexto de design (nível 1; jogador fica oculto e é movido para `fundação + (0, 0, 7)`).
3. S->C `0x1D` (Delete Object) para fixtures, signpost, sign hanger e sign da casa (ServUO).
4. S->C `0xBF/0x20` tipo `0x04`.
5. S->C objeto multi da fundação + `0xBF/0x1D` com a revisão do design.
6. S->C `0xD8` com o estado de design (`response = 0x01`).
7. Cliente abre o gump do designer. Se a revisão difere do cache, envia `0xBF/0x1E` e recebe novo `0xD8`.

Sphere envia `0xBF/0x20`, depois o multi, depois `0xD8`, e só então move/oculta o jogador e remove os itens dinâmicos da casa da vista do cliente.

### 5.2 Edição (repetida)

1. C->S `0xD7` (`0x06` build, `0x05` delete, `0x0D` stairs, `0x13`/`0x14` roof, `0x12` level, `0x02` backup etc.).
2. O servidor aplica no estado de design e incrementa a revisão. Nenhum pacote de retorno se o comando foi válido (o cliente já exibe o resultado localmente).
3. Se o comando é inválido, o servidor reenvia S->C `0xD8` completo para o cliente desfazer a previsão local.

### 5.3 Saída

| Caminho | Sequência |
|---|---|
| Commit | C->S `0xD7/0x04` -> S->C gump de confirmação (preço, saldo) -> C->S `0xB1` botão 1 -> S->C `0xBF/0x20` tipo `0x05`, multi atualizado, `0xBF/0x1D`, `0xD8` com `response = 0` |
| Close | C->S `0xD7/0x0C` -> S->C `0xBF/0x20` tipo `0x05`, multi, `0xD8` (estado visível, `response = 0`) |
| Disconnect | contexto removido no evento de desconexão; nada é enviado |

Em ambos o servidor reexibe fixtures/sign, ejeta todos os itens e mobiles da casa para `BanLocation` e restaura as entidades realocadas.

## 6. Regras do servidor

1. **Contexto obrigatório.** Todo handler de `0xD7` (exceto verificar) retorna sem fazer nada se o jogador não está em contexto de design.
2. **Estados.** A fundação mantém três estados: visível (`CurrentState`), design (`DesignState`) e backup (`BackupState`). Edições vão somente no design. O commit copia o design para o visível.
3. **Build (`0x06`).** Se o jogador não é GM e o item não está na tabela de componentes válidos (`Data/Components/*.txt` filtrada pelo flag de expansão) ou é peça de telhado, reenvia o estado e retorna. `z` é calculado pelo servidor a partir do andar do contexto; para células na linha extra do sul (`y + center.Y == height - 1`) usa `z = 0`. O cliente não envia `z`.
4. **Delete (`0x05`).** Se `z == 0` e `(x, y)` está dentro da grade da fundação (`ay < height - 1`), o componente não é removível: reenvia o estado. Se o item é parte de escada, remove os 4 degraus da escada e reenvia o estado. Se a célula interna (`ax >= 1`, `ay >= 1`, `ay < height - 1`) fica sem piso em `z = 7`, o servidor insere o tile de terra `0x31F4`.
5. **Stairs (`0x0D`).** Valida o índice em `stairs.txt` (coluna multi); se inválido, reenvia o estado. Não reenvia nada em caso de sucesso.
6. **Roof (`0x13`).** Requer SE+ ou GM. `z` deve ser múltiplo de 3 entre `-3` e `12`; fora disso vira `-3`; o `z` final soma o `z` do andar. Antes de inserir, remove peças de telhado já existentes na mesma célula e andar. Valida `TileFlag.Roof` e a tabela `roof.txt`.
7. **Roof delete (`0x14`).** O item deve ter `TileFlag.Roof`; caso contrário reenvia o estado.
8. **Level (`0x12`).** Andar fora de `1..MaxLevels` vira 1 (ServUO) ou é limitado ao máximo (Sphere). O servidor move o jogador para `fundação.Z + zDoAndar` e reenvia o multi + revisão.
9. **Clear (`0x10`).** Novo estado = fundação vazia; revisão incrementada; reenvia multi + `0xBF/0x1D` + `0xD8`.
10. **Restore (`0x03`) / Revert (`0x1A`).** Substitui o design por cópia do backup (restore) ou do estado visível (revert, com fixtures "congelados" de volta como componentes); incrementa a revisão; reenvia multi + `0xBF/0x1D` + `0xD8`.
11. **Backup (`0x02`).** Copia o design para o slot de backup; sem resposta.
12. **Sync (`0x0E`).** Reenvia o `0xD8` do design.
13. **Commit.** Custo = `CustomizationCost + (componentes do design - (componentes visíveis + fixtures)) * 500` (a primeira parcela é 0 sob AOS). Se positivo, debita do banco do jogador; se negativo, deposita. GM não paga. Sem saldo: mensagem `1061903` e o commit é recusado, ficando o jogador em modo de design.
14. **Restrições durante o design.** Fala bloqueada (mensagem `1061925`), ações de movimentação limitadas à área da fundação com `Z` travado no andar atual, demais comandos recebem `1062206`, o jogador fica oculto e não pode revelar-se.
15. **Entidades dentro da casa.** São movidas para fora ao entrar e ao sair do design; mannequins/stewards são re-deeded.
16. **Desconexão.** O servidor remove o contexto e ejeta todos da casa. O estado de design não é descartado (continua em `DesignState` e é reexibido na próxima entrada).
17. **Cache.** O `0xD8` serializado pode ser reaproveitado enquanto a revisão do estado não muda.
18. **Tempo/frequência.** Não há limite de taxa nos handlers; ServUO apenas comprime em thread separada e usa o pacote em cache.

## 7. Exemplo

Casa com `xMin = -3`, `yMin = -3`, `xMax = 3`, `yMax = 4` (`width = 7`, `height = 8`), serial `0x40000123`, revisão 5, e três componentes:

- piso `0x31F4` em `(-2, -2, 7)`: piso (altura <= 0) -> plano 1, `x = 1, y = 1`, depois `x = 0, y = 0`, `size = 6`, `index = 0`; plano com `6 * 6 * 2 = 72` bytes.
- parede `0x0006` em `(-3, -3, 7)` (assumindo altura > 0): plano `1 + 4 = 5`, `x = 0, y = 0`, `size = 7`, `index = 0`; plano com `7 * 7 * 2 = 98` bytes.
- item `0x0751` em `(0, 0, 12)`: `z = 12` não é plano -> bloco de offsets `07 51 00 00 0C`.

Pacote (`zlib` nível 6):

```
D8 00 45                  id, tamanho total = 69
03 01                     compressão 0x03, flag de resposta 0x01
40 00 01 23               serial
00 00 00 05               revisão 5
00 03                     3 componentes
00 34                     comprimento do buffer = 52 (= 69 - 17)
03                        3 blocos
21 48 0D 00  78 9C 33 FC C2 40 15 00 00 51 BC 01 26   plano 1 (0x20|1), 72 -> 13 bytes
25 62 0D 00  78 9C 63 60 63 A0 31 00 00 02 A8 00 07   plano 5 (0x20|5), 98 -> 13 bytes
09 05 0D 00  78 9C 63 0F 64 60 E0 01 00 01 78 00 65   bloco de offsets (9), 5 -> 13 bytes
```

Cabeçalho de bloco `21 48 0D 00`: índice `0x21`, descomprimido `0x048 = 72`, comprimido `0x00D = 13`, byte final `00` (bits altos de ambos = 0).

Entrada no modo de design (revisão 5, mesmo serial):

```
BF 00 11 00 20 40 00 01 23 04 00 00 FF FF FF FF FF   begin
BF 00 0D 00 1D 40 00 01 23 00 00 00 05               revisão
D8 ...                                               estado de design
```

Cliente constrói uma parede `0x0006` em `(1, -2)` e depois troca para o andar 2:

```
D7 00 19 00 00 00 01 00 06 00 00 00 00 06 00 00 00 00 01 00 FF FF FF FE 0A
D7 00 0F 00 00 00 01 00 12 00 00 00 00 02 0A
```

Cliente fecha sem commit e o servidor encerra:

```
D7 00 0A 00 00 00 01 00 0C 0A
BF 00 11 00 20 40 00 01 23 05 00 00 FF FF FF FF FF
```

## 8. Armadilhas

- A grade é column-major `(x * size + y)`, não `(y * width + x)`.
- Os tamanhos por plano não são iguais: o plano 0 é `width * height`, os pisos `(width-1) * (height-2)` e as paredes `width * (height-1)`. Um tamanho errado desalinha tudo sem erro de zlib.
- Incluir a linha extra do sul em `height`; sem ela as dimensões divergem das calculadas pelo cliente.
- Os blocos de offsets usam índice `9 + n` sem `0x20`; os planos usam `0x20 | plano`.
- Os dois tamanhos de 12 bits dividem o 4º byte; escrever `(size >> 8)` e `(clen >> 8)` em nibbles trocados é um erro comum.
- `y` negativo no `0xD7` chega como i32 em complemento de dois; leia como inteiro com sinal.
- O comando de nível carrega o andar como `u32 0, u8 floor`; lido como tag + i32 resulta no andar, mas lido como u32 puro resulta em 0.
- Resposta a edição inválida deve ser o `0xD8` completo; reenviar só `0xBF/0x1D` não faz o cliente desfazer a edição.
- Não enviar `0xD8` antes do multi da fundação existir no cliente.
- Esquecer de re-enviar fixtures, sign e signpost ao sair do design deixa a casa sem porta/placa.
- Sphere e ServUO divergem no cálculo do andar (ver 9); não assuma que o cliente envia `z` em `0x06`.

## 9. Diferenças ServUO x Sphere

| Aspecto | ServUO / ModernUO | Sphere (Source-X) |
|---|---|---|
| Estados | visível, design, backup; revert = cópia do visível | `main`, `working`, `backup`, `revert` |
| Andar | contexto de design guarda `Level`; `0x12` o atualiza | derivado do Z do jogador (`GetPlane`) |
| `0xD8` plano | planos 0..8 (pisos 1-4, paredes 5-8) + blocos 9+ | gera planos 0..4 só com itens altos alinhados ao plano; pisos, itens fora da grade e itens fora do nível vão para blocos 9+ |
| Tamanho da grade | `width*height*2` etc. fixos por plano | `(maxIndex + 1) * 2` por plano |
| Commit | gump de confirmação com preço e banco | aplica diretamente; triggers de script (`HouseDesignCommitItem`) |
| Falha de validação | reenvia o `0xD8` | reenvia o `0xD8` (`SendStructureTo`) |
| Stairs sucesso | sem reenvio | reenvia o `0xD8` |
| Gating de cliente | nenhum na rede; `Core.SE` para roof | versão >= 4.0.0, KR ou Enhanced |
| Restore/Revert idêntico | sempre aplica | ignora se revisões forem iguais |

## 10. Não especificado

- Efeito do byte de "flag de resposta" do `0xD8` e dos subcomandos `0xD7/0x0A` e `0xD7/0x0F`: o cliente lê ou envia e o servidor ignora.
- Semântica dos tipos `1`, `2` e `3` de `0xBF/0x20`.
- Valor exato de `MaxLevels` em Sphere (função `GetLevelCount`).
- Modo `1` (lista de 4 bytes `id,x,y`) de blocos: existe no cliente, nenhum servidor consultado o emite.

## 11. Referência de implementação

| Peça | Arquivo / símbolo |
|---|---|
| ServUO: handlers, estados, contexto, pacotes | `Scripts/Multis/HouseFoundation.cs`: `HouseFoundation.Initialize`, `BeginCustomize`, `Designer_Build/Delete/Stairs/Roof/RoofDelete/Level/Clear/Restore/Revert/Backup/Close/Commit/Sync`, `EndConfirmCommit`, `QueryDesignDetails`, `DesignState`, `DesignContext`, `BeginHouseCustomization`, `EndHouseCustomization`, `DesignStateGeneral`, `DesignStateDetailed`, `ConfirmCommitGump` |
| ServUO: validação | `Scripts/Multis/ComponentVerification.cs`, `Data/Components/*.txt`, `Server/ExpansionInfo.cs` (`HousingFlags`) |
| ServUO: restrições do jogador | `Scripts/Mobiles/PlayerMobile.cs` (`CheckMovement`, `OnLocationChange`, `EventSink_Disconnected`, `RevealingAction`) |
| ServUO: envelope encoded | `Server/Network/PacketHandlers.cs` (`EncodedCommand`, `ExtendedCommand`), `Server/Network/EncodedReader.cs` |
| ServUO: entrada | `Scripts/Gumps/HouseGumpAOS.cs` (botão "Customize This House" -> `BeginCustomize`) |
| ModernUO | `Projects/UOContent/Multis/Houses/HouseFoundation.cs` (mesmos `Designer_*`, `DesignState`); `Projects/UOContent/Multis/Houses/HousePackets.cs` (`SendBeginHouseCustomization`, `SendEndHouseCustomization`, `SendDesignStateGeneral`, `CreateHouseDesignStateDetailed`); `Projects/UOContent/Network/Packets/IncomingHousePackets.cs` (`0xFB`) |
| Sphere | `src/game/items/CItemMultiCustom.cpp` (`BeginCustomize`, `EndCustomize`, `SwitchToLevel`, `CommitChanges`, `AddItem`, `AddStairs`, `AddRoof`, `RemoveItem`, `SendStructureTo`, `BackupStructure`, `RestoreStructure`, `RevertChanges`, `ResetStructure`, `IsValidItem`); `src/network/send.cpp` (`PacketHouseDesignVersion`, `PacketHouseBeginCustomise`, `PacketHouseEndCustomise`, `PacketHouseDesign`); `src/network/receive.cpp` (`PacketHouseDesignReq`, `PacketHouseDesignBackup`, `...Restore`, `...Commit`, `...DestroyItem`, `...PlaceItem`, `...Exit`, `...PlaceStair`, `...Sync`, `...Clear`, `...Switch`, `...PlaceRoof`, `...DestroyRoof`, `...Revert`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`CustomHouse` para `0xD8`, `ReadUnsafeCustomHouseData`, `0xBF` casos `0x1D` e `0x20`); `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_CustomHouse*`); `src/ClassicUO.Client/Game/Managers/HouseCustomizationManager.cs`; `src/ClassicUO.Client/Game/UI/Gumps/HouseCustomizationGump.cs`; `src/ClassicUO.Client/Game/Data/CustomHouse.cs` |
