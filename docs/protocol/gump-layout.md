# Comandos de layout de gump (client 7.0.x)

O layout de um gump (pacotes `0xB0` Gump Menu Dialog e `0xDD` Compressed Gump) é uma sequência de blocos de texto entre chaves, `{ comando arg arg ... }`. O cliente lê a primeira palavra do bloco e a compara por igualdade exata com os comandos abaixo; blocos com comando desconhecido são ignorados.

Todos os argumentos são inteiros decimais, exceto onde indicado. As coordenadas `x` e `y` são relativas ao canto do gump.

## Comandos

| Comando | Argumentos | Significado | Exemplo |
|---|---|---|---|
| `page` | `n` | Inicia a página `n`; elementos seguintes pertencem a ela (página 0 é sempre visível) | `{ page 1 }` |
| `nodispose` | — | Desativa o descarte do gump (`Disposable = false` no ServUO) | `{ nodispose }` |
| `nomove` | — | O gump não pode ser arrastado | `{ nomove }` |
| `noclose` | — | Desativa o fechamento do gump (`Closable = false` no ServUO) | `{ noclose }` |
| `group` | `n` | Inicia o grupo `n` de botões de rádio | `{ group 1 }` |
| `endgroup` | — | Encerra o grupo corrente | `{ endgroup }` |
| `gumppic` | `x y gumpId [hue=N] [class=NOME]` | Imagem de gump. `hue=` aplica matiz; `class=VirtueGumpItem` cria o item de virtude (duplo clique envia `0xB1`) | `{ gumppic 0 0 2600 hue=33 }` |
| `tilepic` | `x y itemId` | Imagem de item (art) | `{ tilepic 10 10 3823 }` |
| `tilepichue` | `x y itemId hue` | Imagem de item com matiz | `{ tilepichue 10 10 3823 33 }` |
| `text` | `x y hue textId` | Texto estático; `textId` indexa a lista de textos do pacote | `{ text 40 20 0 0 }` |
| `croppedtext` | `x y width height hue textId` | Texto cortado na caixa indicada | `{ croppedtext 40 20 200 20 0 0 }` |
| `textentry` | `x y width height hue entryId textId` | Campo de entrada de texto com valor inicial `textId` | `{ textentry 40 60 200 20 0 1 2 }` |
| `textentrylimited` | `x y width height hue entryId textId limit` | Campo de texto com limite de caracteres | `{ textentrylimited 40 60 200 20 0 1 2 30 }` |
| `radio` | `x y gumpIdOff gumpIdOn initialState switchId` | Botão de rádio | `{ radio 10 100 208 209 0 1 }` |
| `button` | `x y gumpIdUp gumpIdDown type param1 param2` | Botão. `type` 1 = envia resposta; 0 = troca de página (`param1` = página); `param2` = `buttonId` | `{ button 10 140 4005 4007 1 0 1 }` |
| `buttontileart` | `x y gumpIdUp gumpIdDown type param1 param2 itemId hue width height` | Botão com imagem de item sobreposta | `{ buttontileart 10 140 4005 4007 1 0 1 3823 0 40 40 }` |
| `checkbox` | `x y gumpIdOff gumpIdOn initialState switchId` | Caixa de seleção | `{ checkbox 10 180 210 211 0 2 }` |
| `resizepic` | `x y gumpId width height` | Fundo redimensionável composto por 9 partes do gump `gumpId` | `{ resizepic 0 0 9200 300 200 }` |
| `htmlgump` | `x y width height textId background scrollbar` | Área de texto HTML; `background` e `scrollbar` são 0 ou 1 | `{ htmlgump 10 10 280 100 3 0 1 }` |
| `xmfhtmlgump` | `x y width height cliloc background scrollbar` | Área HTML com texto vindo do cliloc | `{ xmfhtmlgump 10 10 280 100 1060635 0 1 }` |
| `xmfhtmlgumpcolor` | `x y width height cliloc background scrollbar color` | Como `xmfhtmlgump`, com cor do texto | `{ xmfhtmlgumpcolor 10 10 280 100 1060635 0 1 32767 }` |
| `xmfhtmltok` | `x y width height background scrollbar color cliloc [@args@]` | Cliloc com argumentos separados por `@` | `{ xmfhtmltok 10 10 280 100 0 1 32767 1060635 @a@b@ }` |
| `checkertrans` | `x y width height` | Região translúcida | `{ checkertrans 0 0 300 200 }` |
| `gumppictiled` | `x y width height gumpId` | Imagem de gump repetida em mosaico | `{ gumppictiled 0 0 300 200 2624 }` |
| `tooltip` | `cliloc` | Tooltip (cliloc) do elemento anterior | `{ tooltip 1042971 }` |
| `itemproperty` | `serial` | Associa as propriedades (tooltip) do item `serial` ao elemento anterior | `{ itemproperty 1073741825 }` |
| `mastergump` | `id` | Gumps com o mesmo `mastergump` compartilham a posição salva ao serem reabertos | `{ mastergump 100 }` |

## Observações

- `noresize`, `picinpic`, `gumppichued`, `gumppicphued` e `virtue*` não existem no cliente 7.0.10.3 e são ignorados.
- A única `class` reconhecida em `gumppic` é `VirtueGumpItem`. O duplo clique envia `0xB1` (Gump Menu Response) com o serial do jogador, o gump id `0x1CD` e o id da virtude, sem switches nem textos.
- Sem `mastergump`, cada tipo de gump lembra a própria posição.
- O texto de `text`, `croppedtext`, `textentry`, `htmlgump` etc. fica fora do bloco de layout: `textId` é o índice na lista de linhas de texto enviada junto com o layout.
