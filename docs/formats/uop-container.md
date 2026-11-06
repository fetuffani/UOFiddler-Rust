# Contêiner UOP (MYP)

## Finalidade

O UOP é o contêiner de arquivos usado pelo cliente Ultima Online para agrupar muitos recursos em um único arquivo (animações, tabelas, texto, imagens, mapas). Cada recurso é uma entrada identificada por um hash de 64 bits calculado sobre um caminho lógico (por exemplo `build/animationlegacyframe/000009/01.bin`). O nome em si não é gravado no arquivo.

Arquivos: `*.uop` (por exemplo `AnimationFrame1.uop`, `AnimationSequence.uop`, `MainMisc.uop`, `string_dictionary.uop`, `tileart.uop`).

Little-endian em todo o formato.

## Layout geral

```
[cabeçalho do arquivo  : 40 bytes]
[bloco 0               : 12 + capacidade * 34 bytes]
[dados das entradas do bloco 0]
[bloco 1]
[dados das entradas do bloco 1]
...
```

Os blocos são encadeados por ponteiro. O primeiro bloco começa no offset indicado no cabeçalho (40 nos arquivos observados). Cada bloco é seguido pelos dados das entradas que ele descreve, de modo que o bloco seguinte fica no meio do arquivo (por exemplo, em `AnimationFrame1.uop` o bloco 1 está em 83914468 e o bloco 2 em 89789380).

## Cabeçalho do arquivo (40 bytes)

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | char[4] | signature | `"MYP\0"` (`4D 59 50 00`) |
| 4 | u32 | version | 4 nos arquivos do cliente; o conversor do Fiddler grava 5 |
| 8 | u32 | misc | `0xFD23EC43` em todos os arquivos observados |
| 12 | u64 | first_block_offset | offset absoluto do primeiro bloco (40) |
| 20 | u32 | block_capacity | número de slots por bloco (100) |
| 24 | u32 | file_count | total de entradas em uso no arquivo |
| 28 | u8[12] | reserved | zeros |

`file_count` é igual à soma das entradas em uso de todos os blocos (287 em `AnimationFrame1.uop`, que tem blocos com 100, 100 e 87 entradas).

## Bloco

Um bloco ocupa sempre `12 + block_capacity * 34` bytes (3412 com capacidade 100). Slots não usados ficam zerados.

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u32 | count | número de slots em uso neste bloco |
| 4 | u64 | next_block | offset absoluto do próximo bloco; 0 no último |
| 12 | entrada[block_capacity] | entries | tabela de entradas (34 bytes cada) |

Para ler: começar em `first_block_offset`; ler `count` entradas a partir de `bloco + 12`; seguir `next_block` até 0. Ignorar slots com `offset == 0`.

## Entrada da tabela (34 bytes)

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u64 | offset | posição absoluta do início da entrada no arquivo; 0 = slot vazio |
| 8 | u32 | header_length | bytes de cabeçalho por entrada antes dos dados (12 nos arquivos do cliente; 0 nos gerados pelo conversor) |
| 12 | u32 | compressed_length | tamanho dos dados gravados |
| 16 | u32 | decompressed_length | tamanho dos dados após descompressão |
| 20 | u64 | name_hash | hash Jenkins de 64 bits do caminho lógico (ver abaixo) |
| 28 | u32 | data_hash | Adler-32 dos `header_length` bytes de cabeçalho da entrada (arquivos do cliente); 0 nos gerados pelo conversor |
| 32 | u16 | compression | 0 = sem compressão, 1 = zlib |

## Região de dados

Os dados de uma entrada ficam em `offset + header_length`, com `compressed_length` bytes.

- `compression == 0`: os bytes são o conteúdo (`compressed_length == decompressed_length`).
- `compression == 1`: os bytes são um fluxo zlib (RFC 1950) que descomprime para `decompressed_length` bytes.

Nos arquivos de exemplo todas as entradas usam `compression == 1`. As entradas são gravadas em ordem crescente de offset, na mesma ordem da tabela.

### Cabeçalho por entrada (12 bytes)

Nos arquivos do cliente cada entrada começa com 12 bytes antes dos dados (`header_length = 12`):

| Offset | Tipo | Campo | Significado |
|---:|---|---|---|
| 0 | u16 | - | 3 em todas as entradas observadas |
| 2 | u16 | - | 8 em todas as entradas observadas |
| 4 | u8[8] | - | varia por entrada; não interpretado |

Campos não interpretados: os 8 últimos bytes do cabeçalho por entrada.

## Hash de nome (Jenkins de 64 bits)

O hash é calculado sobre os bytes do caminho em minúsculas, sem barra inicial e com `/` como separador. O resultado é `(b << 32) | c`. Para o texto vazio o resultado é `c << 32`, com `c = 0xDEADBEEF`.

Aritmética em u32 com overflow (wrapping); `rotl` é rotação à esquerda de 32 bits.

```
len = tamanho em bytes
a = b = c = 0xDEADBEEF + len

enquanto len > 12:
    a += u32le(k[0..4]);  b += u32le(k[4..8]);  c += u32le(k[8..12])
    a -= c;  a ^= rotl(c, 4);   c += b
    b -= a;  b ^= rotl(a, 6);   a += c
    c -= b;  c ^= rotl(b, 8);   b += a
    a -= c;  a ^= rotl(c, 16);  c += b
    b -= a;  b ^= rotl(a, 19);  a += c
    c -= b;  c ^= rotl(b, 4);   b += a
    k += 12;  len -= 12

se len == 0: retorna c << 32

# restante (1..12 bytes), preenchido com zeros até 12
a += u32le(rest[0..4]);  b += u32le(rest[4..8]);  c += u32le(rest[8..12])

c ^= b;  c -= rotl(b, 14)
a ^= c;  a -= rotl(c, 11)
b ^= a;  b -= rotl(a, 25)
c ^= b;  c -= rotl(b, 16)
a ^= c;  a -= rotl(c, 4)
b ^= a;  b -= rotl(a, 14)
c ^= b;  c -= rotl(b, 24)

retorna (b << 32) | c
```

Valor de referência: `hash("build/animationlegacyframe/000009/01.bin") = 0x994AE0E43E27CB16`.

## Padrões de nome existentes

| Arquivo | Padrão do caminho lógico | Observação |
|---|---|---|
| AnimationFrame1..4.uop | `build/animationlegacyframe/{body:06}/{action:02}.bin` | ação 0 a 78 nos exemplos |
| AnimationSequence.uop | `build/animationsequence/{id:08}.bin` | id 0 a 1247 (esparso) |
| string_dictionary.uop | `build/stringdictionary/string_dictionary.bin` | entrada única |
| tileart.uop | `build/tileart/{id:08}.bin` | ids 0 a 39212, 10000000 a 10001247 e 99999999 |
| artLegacyMUL.uop | `build/artlegacymul/{i:08}.tga` | i = índice do art.mul |
| gumpartLegacyMUL.uop | `build/gumpartlegacymul/{i:08}.tga` | i = índice do gumpart.mul |
| soundLegacyMUL.uop | `build/soundlegacymul/{i:08}.dat` | i = índice do sound.mul |
| map{N}LegacyMUL.uop | `build/map{N}legacymul/{i:08}.dat` | i = número do pedaço de 0xC4000 bytes |
| MultiCollection.uop | `build/multicollection/{id:06}.bin` e `build/multicollection/housing.bin` | |
| MainMisc.uop | nomes não estabelecidos | identificar pelo hash (ver `uop-files.md`) |

Os arquivos `*LegacyMUL.uop`, `MultiCollection.uop` e `GameData.uop` seguem as convenções acima, mas não existem no diretório de exemplo.

O mapa de recursos em `MainMisc.uop` lista, para cada tipo de recurso, o diretório e o sufixo no formato empacotado (por exemplo `Build/AnimationSequence/` + 8 dígitos + `.bin`, `Build/Paperdoll/` + 6 dígitos + `/` + 2 dígitos + `.bin`). Os nomes de caminho são sempre convertidos para minúsculas antes do hash.

## Enumeração de entradas sem nome

O arquivo não guarda nomes, apenas `name_hash`. Para obter um nome:

1. Ler todas as entradas (percorrendo os blocos) e montar um dicionário `name_hash -> entrada`.
2. Gerar os nomes candidatos do padrão do arquivo (por exemplo `body` de 0 a 1999 e `action` de 0 a 79), calcular o hash de cada um e procurar no dicionário.
3. Entradas sem correspondência permanecem identificadas apenas por `name_hash` (exibir como `{hash:016x}`) e pela posição na tabela.

Pontos de atenção:

- O hash é sensível a maiúsculas: o caminho deve estar em minúsculas (`build/stringdictionary/string_dictionary.bin`, não `Build/StringDictionary/...`).
- Em `tileart.uop`, a entrada `build/tileart/99999999.bin` está fora das faixas de id normais.
- Em `MainMisc.uop` nenhum padrão de nome foi estabelecido; as 4 entradas são identificadas pelo hash (ver `uop-files.md`).

## Conversão para MUL

Os `*LegacyMUL.uop` mapeiam um-para-um os arquivos `.mul/.idx` clássicos. Ver `uop-files.md`, seção "UOPs de conteúdo legado", para o layout de cada tipo e a ferramenta `uop_convert`.

## Campos não interpretados

- Últimos 8 bytes do cabeçalho de 12 bytes por entrada.
- Algoritmo de `data_hash` para entradas com `header_length == 0` (o conversor grava 0).
- Significado do campo `misc` (`0xFD23EC43`).

Leitor no Fiddler: `uo_core::client_files` (`read_uop`, `read_uop_entry`, `resolve_uop_names`) e `uo_core::hash` (`hash_file_name`); escrita em `uo_core::uop_convert` (`write_uop`).
