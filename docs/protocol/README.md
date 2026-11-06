# Documentação do protocolo de rede do Ultima Online

Referência dos pacotes do cliente 7.0.x e de fluxos que combinam vários pacotes.

## Índice de documentos

### Tabelas
- [packets.md](packets.md): tabelas de pacotes servidor->cliente e cliente->servidor (id, tamanho, nome) e layouts de `0xB4`, `0x84`, `0xF1`/`0xF2`.
- [gump-layout.md](gump-layout.md): comandos de layout de gump (`{ cmd arg ... }`) dos pacotes `0xB0` e `0xDD`.

- [packets/README.md](packets/README.md): formato campo a campo de cada pacote e de cada subcomando `0xBF`/`0xD7`.

### Fluxos com vários pacotes
- [house-customization.md](house-customization.md): editor de casas customizadas (`0xBF/0x20`, `0xBF/0x1D`, `0xBF/0x1E`, `0xD8`, subcomandos `0xD7`), estrutura de planos, compressão e revisão.
- [boats-smooth-sailing.md](boats-smooth-sailing.md): movimento de barcos (`0xF6`, `0xF7`, `0xBF/0x33`, `0x99`), velocidade, virada, piloto, comparação com o movimento legado.
- [gump-open-response.md](gump-open-response.md): abertura de gump (`0xB0`/`0xDD`), resposta (`0xB1`) com switches e campos de texto, fechamento pelo servidor (`0xBF/0x04`).
- [login-flow.md](login-flow.md): fluxo de login (seed `0xEF`, `0x80`, `0xA8`, `0xA0`, `0x8C`, `0x91`, `0xA9`, seleção, criação e exclusão de personagem, `0xBD`), criptografia e compressão, entrada no mundo (`0x1B` a `0x55`).

## Como ler

- Inteiros são big-endian na rede, exceto quando indicado. `u8`, `u16`, `u32` = sem sinal; `i8`, `i16`, `i32` = com sinal.
- Offsets são contados a partir do byte 0 do pacote, que é o id. A coluna "Tamanho" de [packets.md](packets.md) é o total de bytes, incluindo o id; `var` indica tamanho variável, e nesse caso os bytes 1 e 2 trazem o tamanho total (`u16`).
- `0xBF` e `0xD7` são envelopes: `0xBF` = `[id][u16 tamanho][u16 subcomando][dados]`; `0xD7` = `[id][u16 tamanho][u32 serial][u16 subcomando][argumentos][0x0A]`. Nos documentos, um subcomando é escrito como `0xBF/0x20` ou `0xD7/0x06`.
- `S->C` é servidor para cliente e `C->S` é cliente para servidor.
- Cada documento de fluxo segue a mesma ordem: visão geral e versões de cliente, sequência numerada de mensagens, layout byte a byte dos pacotes, estrutura de dados, regras do servidor, exemplo com bytes, armadilhas comuns, diferenças entre implementações, itens não especificados e referência de implementação (caminhos de arquivo).
- Exemplos em hexadecimal estão separados por espaço, um byte por par.
- Identificadores de código (nomes de classes, métodos, flags) ficam em inglês, como nos repositórios de referência.
- Quando uma implementação difere de outra (ServUO, ModernUO, Sphere/Source-X, ClassicUO), a diferença é descrita na seção própria do documento.
