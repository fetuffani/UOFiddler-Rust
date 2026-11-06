# Gump: abertura, resposta, switches e campos de texto

Fluxo de um gump genérico: o servidor descreve a janela (layout + linhas de texto), o cliente a exibe e devolve em um único pacote o botão clicado, os switches ligados (rádios e caixas de seleção) e o conteúdo dos campos de texto. A sintaxe do layout está em [gump-layout.md](gump-layout.md).

Convenções: inteiros big-endian; texto de linhas e de campos em UTF-16 big-endian sem terminador; offsets a partir do byte 0 (id).

## 1. Visão geral e variantes

| Pacote | Direção | Uso |
|---|---|---|
| `0xB0` Gump Menu Dialog | S->C | Layout em ASCII e linhas sem compressão |
| `0xDD` Compressed Gump | S->C | Mesmo conteúdo, com layout e linhas comprimidos (zlib) |
| `0xB1` Gump Menu Response | C->S | Resposta do cliente |
| `0xBF/0x04` Close Gump | S->C | Fecha um gump por tipo (opcionalmente simulando um botão) |

ServUO escolhe `0xDD` para clientes com a flag `Unpack` (>= 5.0.0a) e `0xB0` para os anteriores; ModernUO sempre emite `0xDD`.

## 2. Sequência

1. S->C `0xDD` (ou `0xB0`): `serial` do gump (único por instância), `typeId` (identifica a classe), posição `(x, y)`, layout e linhas de texto. O servidor guarda o gump na lista do `NetState`, junto com a contagem de switches e de campos de texto declarados no layout.
2. O jogador interage. O cliente guarda localmente o estado de cada rádio/caixa e o texto de cada campo.
3. C->S `0xB1`: ao clicar um botão do tipo "resposta", o cliente envia `serial`, `typeId`, o `buttonId` e as listas de switches ligados e de campos de texto. Botão `0` significa fechar/cancelar.
4. O servidor localiza o gump por `(serial, typeId)`, valida a resposta, remove o gump da lista e chama o tratador (`OnResponse`).
5. Opcional: S->C `0xBF/0x04` com `typeId` e `buttonId` para fechar o gump pelo servidor. O cliente fecha o gump correspondente e, se `buttonId != 0`, simula o clique nesse botão (e, portanto, envia um `0xB1`).

Botões de troca de página (`type 0`) são tratados só no cliente e não geram `0xB1`.

## 3. Pacotes

### 3.1 `0xB0` (S->C, `var`)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xB0` | |
| 1 | u16 | tamanho total | |
| 3 | u32 | serial do gump | |
| 7 | u32 | `typeId` | |
| 11 | u32 | x | |
| 15 | u32 | y | |
| 19 | u16 | `layoutLength` | bytes do layout; a variante padrão do ServUO inclui o terminador `0x00` na contagem, a variante "fast" não escreve terminador |
| 21 | char[`layoutLength`] | layout ASCII | `{ cmd args }` repetido |
| ... | u16 | `lineCount` | |
| ... | por linha | `u16 length` (caracteres) + `length * 2` bytes UTF-16BE | |

### 3.2 `0xDD` (S->C, `var`)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xDD` | |
| 1 | u16 | tamanho total | |
| 3 | u32 | serial do gump | |
| 7 | u32 | `typeId` | |
| 11 | u32 | x | |
| 15 | u32 | y | |
| 19 | u32 | `4 + comprimento comprimido do layout` | |
| 23 | u32 | tamanho do layout descomprimido | inclui o `0x00` final do layout |
| 27 | u8[] | layout comprimido (zlib) | |
| ... | u32 | `lineCount` | |
| ... | u32 | `4 + comprimento comprimido das linhas` | com `lineCount == 0` o servidor escreve apenas um `u32 0` no lugar deste bloco |
| ... | u32 | tamanho das linhas descomprimidas | |
| ... | u8[] | linhas comprimidas (zlib) | cada linha: `u16 length` (caracteres) + UTF-16BE |

Se o bloco a comprimir é vazio, o servidor escreve um único `u32 0` no lugar do par de campos. Se a compressão falha, o ModernUO escreve `u32 4` e `u32 0`.

### 3.3 `0xB1` Gump Menu Response (C->S, `var`)

| Offset | Tipo | Campo | Notas |
|---|---|---|---|
| 0 | u8 | `0xB1` | |
| 1 | u16 | tamanho total | |
| 3 | u32 | serial do gump | |
| 7 | u32 | `typeId` | |
| 11 | u32 | `buttonId` | `0` = fechar |
| 15 | u32 | `switchCount` | |
| 19 | u32[`switchCount`] | `switchId` | só os rádios e caixas **marcados** |
| ... | u32 | `textCount` | |
| ... | por campo | `u16 entryId`, `u16 length` (caracteres), `length * 2` bytes UTF-16BE | todos os campos de texto do gump, mesmo vazios |

O cliente limita cada campo a 239 caracteres.

### 3.4 `0xBF/0x04` Close Gump (S->C, 13 bytes)

| Offset | Tipo | Campo |
|---|---|---|
| 0 | u8 | `0xBF` |
| 1 | u16 | `0x000D` |
| 3 | u16 | `0x0004` |
| 5 | u32 | `typeId` |
| 9 | u32 | `buttonId` |

## 4. Mapeamento layout -> resposta

| Elemento do layout | O que volta no `0xB1` |
|---|---|
| `button` (type 1) | `buttonId` = `param2` do comando |
| `radio`, `checkbox` | `switchId` incluído em `switches` apenas se o elemento está marcado |
| `textentry`, `textentrylimited` | par `(entryId, texto atual)` sempre presente |
| `button` (type 0) | nenhum pacote (troca de página local) |
| Gump virtue (`typeId = 461`, `0x1CD`) | caso especial sem gump registrado no servidor: `serial` = jogador alvo, `buttonId` = id da virtude (ModernUO também aceita `buttonId = 1` com um switch para pedir o gump de virtudes) |

## 5. Regras do servidor

1. Procurar o gump por `serial` e `typeId` na lista do `NetState`; se não existir, ignorar a resposta (exceto o caso virtue acima).
2. `buttonId == 0` é sempre válido. Qualquer outro `buttonId` deve existir como botão de resposta no gump (`GumpButton` ou `GumpImageTileButton`); caso contrário a resposta é inválida.
3. `switchCount` não pode exceder o número de rádios e caixas declarados no layout (contados ao montar o pacote); `textCount` não pode exceder o número de campos de texto declarados; cada campo tem no máximo 239 caracteres.
4. Resposta inválida: ServUO fecha a conexão do cliente; ModernUO registra a exceção e descarta a resposta.
5. Resposta válida: remover o gump da lista antes de chamar `OnResponse`, para que o tratador possa reenviar o mesmo gump.
6. Para reaproveitar o `serial` de um gump aberto, feche o anterior primeiro; o `typeId` é por classe, o `serial` por instância.
7. O servidor deve guardar a quantidade de switches e de campos de texto de cada gump para poder validar a resposta.

## 6. Exemplo

Gump `serial = 0x00000001`, `typeId = 0x000000A5`, em `(50, 50)`, com layout `{ page 0 }{ textentry 10 10 100 20 0 1 0 }{ radio 10 40 208 209 0 7 }{ button 10 70 4005 4007 1 0 5 }` e a linha de texto `"x"`.

Resposta do cliente: botão 5, rádio 7 marcado, campo 1 com o texto `"Oi"`:

```
B1 00 23                  tamanho = 35
00 00 00 01               serial
00 00 00 A5               typeId
00 00 00 05               buttonId
00 00 00 01               switchCount
00 00 00 07               switchId 7
00 00 00 01               textCount
00 01 00 02               entryId 1, 2 caracteres
00 4F 00 69               "Oi" em UTF-16BE
```

Resposta para fechar sem escolher nada: `B1 00 17 | serial | typeId | 00 00 00 00 | 00 00 00 00 | 00 00 00 00`.

## 7. Armadilhas

- `switchId` e `entryId` são os índices declarados no layout, não a posição na lista; o cliente envia somente os rádios/caixas marcados.
- O número de caracteres de cada campo de texto está em unidades de 2 bytes; usar bytes quebra o alinhamento seguinte.
- O gump do servidor mantém contadores de switches e de campos; um gump reaberto com layout diferente reutilizando o serial deve recalcular esses contadores.
- No `0xDD`, `lineCount == 0` não é seguido do par de tamanhos: só um `u32 0`. O cliente não lê esse campo.
- Em `0xB0`, a contagem `layoutLength` varia entre implementações (com ou sem `0x00`); o cliente lê exatamente essa quantidade de bytes.
- Esquecer de remover o gump antes de `OnResponse` faz respostas duplicadas serem aceitas.

## 8. Referência de implementação

| Peça | Arquivo / símbolo |
|---|---|
| ServUO | `Server/Gumps/Gump.cs` (`Compile`, `OnResponse`), `Server/Network/Packets.cs` (`DisplayGump`, `DisplayGumpFast`, `DisplayGumpPacked`, `CloseGump`), `Server/Network/PacketHandlers.cs` (`DisplayGumpResponse`), `Server/Gumps/RelayInfo.cs` |
| ModernUO | `Projects/UOContent/Gumps/Base/Legacy/Gump.cs`, `StaticGump.cs`, `DynamicGump.cs`, `OutgoingGumpPackets.cs` (`WritePacked`), `GumpSystem.IncomingPackets.cs` (`DisplayGumpResponse`) |
| ClassicUO | `src/ClassicUO.Client/Network/PacketHandlers.cs` (`OpenGump`, `OpenCompressedGump`, `0xBF` caso `4`), `src/ClassicUO.Client/Game/UI/Gumps/Gump.cs` (`OnButtonClick`), `src/ClassicUO.Client/Network/OutgoingPackets.cs` (`Send_GumpResponse`) |
