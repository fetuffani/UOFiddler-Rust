# Comandos de layout de gump do client 7.0.10.3 (extraídos do binário)

Parser: 0x4cb744..0x4cc917. Fluxo: copia o texto entre `{` e `}`, extrai a 1ª palavra (0x56c800) e compara com cada comando por `repe cmpsb`
(ECX = len+1, ou seja, igualdade exata). Se nenhum casa, o bloco é ignorado. Strings de comando: .rdata 0x66e594..0x66e768. Args via sscanf (0x5fd94f).

| # | Comando | Args (sscanf) | Criação / handler | Notas |
|---|---|---|---|---|
| 1 | page | 1 inteiro (0x5ff60a) | — | muda a página corrente |
| 2 | nodispose | — | [gump+0xd0]=0 | |
| 3 | nomove | — | [gump+0xcc]=0 | |
| 4 | noclose | — | [gump+0xd4]=0 | |
| 5 | group | 1 inteiro | 0x4c7810 | |
| 6 | endgroup | — | zera grupo corrente | |
| 7 | gumppic | parser próprio (tokens) | 0x4cb2f0 -> 0x4c79b0 | aceita `hue=N` e `class=NOME` |
| 8 | tilepic | `%d %d %s` | 0x4c7cd0 | 3º arg lido como string |
| 9 | tilepichue | `%d %d %d %d` | 0x4c7cd0 | x y id hue |
| 10 | text | `%d %d %d %d` | 0x4c7ea0 | |
| 11 | croppedtext | 6 inteiros | 0x4c7fa0 | |
| 12 | textentry | 7 inteiros | 0x4c6460/… | |
| 13 | textentrylimited | 8 inteiros | 0x4c8080 | |
| 14 | radio | 6 inteiros | 0x4c84d0 | |
| 15 | button | 7 inteiros | 0x4c81a0 | cria objeto 0x7c B (mesma base do gumppic) |
| 16 | buttontileart | 11 inteiros | 0x4c82c0 | |
| 17 | checkbox | 6 inteiros | 0x4c83e0 | |
| 18 | resizepic | 5 inteiros | 0x4c7b10 | |
| 19 | htmlgump | 7 inteiros | 0x4c7bf0 | |
| 20 | xmfhtmlgump | 7 inteiros | 0x4c7bf0 + cliloc | |
| 21 | xmfhtmlgumpcolor | 8 inteiros | idem | |
| 22 | xmfhtmltok | parser próprio | idem | cliloc com argumentos |
| 23 | checkertrans | 4 inteiros | 0x4c7dd0 | |
| 24 | gumppictiled | 5 inteiros | 0x4c78d0 | |
| 25 | tooltip | 1 inteiro (cliloc) | — | |
| 26 | itemproperty | 1 inteiro (serial) | 0x43f570/0x440b90 | |
| 27 | mastergump | `%d` (1 inteiro) | 0x4ed570 | grava map1[tipoDoGump] = mestre (map @0xa0e634); mestre compartilha posição salva (map2 @0xa0e640) |

Padrões do gump: "generic gump" (nome default, [gump+8]); campos [gump+0xbc..0xe8] inicializados em 0x4cb6eb.
NÃO existem neste cliente: noresize, picinpic, gumppichued, virtue* (comandos de clientes mais novos).

## gumppic: parâmetros
- `hue=N` (string em 0x6a2e8c = "hue")
- `class=NOME` (palavra "class" em 0x66e594). Único valor reconhecido: **VirtueGumpItem** (0x66e564).
  - Sem class (ou outro valor): objeto GumpPic (0x7c bytes, ctor 0x4c6920, vtable 0x66e4f0).
  - class=VirtueGumpItem: objeto 0x84 bytes, ctor 0x4c7560 (chama 0x4c6920 e acrescenta [+0x7c]=0, [+0x80]=0), vtable 0x66e540.
    Métodos sobrescritos: [0] dtor 0x4c9020; [4] clique 0x4c8c90 (duplo clique via GetDoubleClickTime; chama 0x4c7010); [5] 0x4c70b0 (stub, retorna 0);
    [6] hover 0x4c8a40 (tooltip por nível: tabela 0x6cf780, 16 B/entrada {id 0x69..0x70, art1, art2, art3}); [7] 0x4c8c60 (mouse sai).
  - Duplo clique (0x4c7010) envia pacote **0xB1** (Gump Menu Response): u32 serial do jogador ([0xab48e4]+0xa0), u32 **0x1CD** (id do gump), u32 [elemento+0x24] (id da virtude), sem switches/textos.

## mastergump (0x4cc8d5)
- Parse: sscanf("%d"); chama 0x4ed570(chaveDoGump=[arg do parser], mestre) -> std::map<int,int> map1 @0xa0e634: map1[chave] = mestre.
- Ao fim do parse (0x4cc95c): 0x4eae50 resolve mestre = map1.find(chave) (se não existe, devolve a própria chave) e procura em map2 @0xa0e640; se houver posição salva (x != 0xFFFF) aplica em [gump+0x34],[gump+0x38].
- Ao mover o gump (0x4c6800..0x4c6849): 0x4ed590 grava map2[mestre(chave)] = (x,y).
- Efeito: gumps de tipos diferentes com o mesmo `mastergump N` reabrem na mesma posição lembrada. Sem mastergump, cada tipo lembra a sua. (deduzido do código; não testado em runtime)
- ClassicUO guarda MasterGumpSerial, mas o uso está comentado ("does not work as expected").
