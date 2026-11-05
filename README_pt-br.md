# UOFiddler 🗡️

> 🌐 **Language / Idioma**: [English (README.md)](README.md) | **Português**  
> 👤 **Autor**: Mauricio Nunes + DeepSeek & Gemini  
> 🔗 **GitHub**: [https://github.com/mbnunes/UOFiddlerPy](https://github.com/mbnunes/UOFiddlerPy)

**UOFiddler** é uma reimplementação moderna, ultrarrápida e **100% nativa em Rust** do clássico **UOFiddler** para Ultima Online. Desenvolvido com **egui / eframe**, ele executa de forma estável, fluida e elegante no **Linux, Windows e macOS**, eliminando integralmente qualquer dependência de **.NET runtime**, **Mono**, **Wine** ou **Python**.

---

## ⚡ Destaques e Diferenciais do UOFiddler

A execução do UOFiddler original em C# via Wine ou Mono no Linux causa frequentes falhas de renderização, travamentos e problemas de case-sensitivity no sistema de arquivos. O **UOFiddler** soluciona tudo de forma definitiva:

1. **100% Rust Nativo**: Zero dependências de runtime. Inicialização instantânea (< 50 ms) e consumo mínimo de memória RAM.
2. **Dual-Client Engine (Clássico & Enhanced UOP)**: Alternância em tempo real entre o cliente **Clássico 2D** (`.mul`/`.idx`) e o **Enhanced Client 3D** (`.uop` Mythic MYP v5 / texturas DDS nativas).
3. **Interface Gráfica Acelerada por Hardware (egui/eframe)**: Navegação suave a 60+ FPS por todas as tabelas de **65.536 itens estáticos**, **16.384 land tiles**, **79 tabelas GameData** e estruturas multis complexas.
4. **Detecção Automática Inteligente**: Localiza automaticamente diretórios de instalação do UO Classic e Enhanced (`/opt/hd_jogos/`, `~/.wine/`, Lutris, Steam, pastas locais) com resolução de arquivos imune a maiúsculas/minúsculas.
5. **Navegação Universal por Teclado**: Suporte completo a navegação por setas (**Cima**, **Baixo**, **PageUp**, **PageDown**) com auto-scroll em todas as grades e listas de todas as abas.
6. **Áudio Nativo sem Drivers Extras**: Reprodução de áudio PCM WAV diretamente via `rodio` em servidores PipeWire, PulseAudio e ALSA.
7. **Barra Vertical de Zoom**: Visualização em tamanho real (1:1) com barra vertical de zoom (0.5x a 5.0x) nas abas de Itens, Land Tiles, TileData, Texturas e Animações.

---

## 📑 Visão Geral Completa das 18 Abas

| Aba | Descrição |
|---|---|
| **⚔️ Items** | Visualizador de itens estáticos em modos **Grade** e **Lista**. Alternância dinâmica entre arte clássica 2D e texturas DDS do Enhanced Client (`LegacyTexture.uop`). Testador de Hues (3.000 cores), atributos do TileData e ferramenta vertical de zoom. |
| **🌿 Land Tiles** | 16.384 terrenos com geometria de diamante (losango 44x44). Suporte a terrenos 2D clássicos e texturas 3D do Enhanced Client (`Texture.uop`), flags de física e slider de zoom. |
| **📋 TileData** | Editor completo para ItemData (65.536 itens) e LandData (16.384 terrenos). Suporte duplo a flags clássicas (32 bits) e modernas do **High Seas (64 bits)**. Visualização em grade e tabela técnica. |
| **🖼️ Gumps** | Navegador para `gumpart.mul` e pacotes Mythic `gumpartLegacyMUL.uop`. Sobreposição das caixas de colisão de contêineres (71 caixas), busca e barra vertical de zoom. |
| **🎨 Hues** | Tabela completa com todas as 3.000 matizes de cores do Ultima Online. Inspetor de gradiente com amostragem RGB555 individual de 32 cores por hue. |
| **🧱 Textures** | Texturas de superfície 64x64 e 128x128 de `texmaps.mul`. Modos grade e lista com visualização 1:1 e zoom vertical. |
| **🎯 RadarCol** | Todas as 81.920 cores de radar do minimapa mapeadas para itens e terrenos. Visualização da cor RGB e da arte do tile em tamanho real. |
| **🎭 Animations** | Player de animações de criaturas com suporte a `anim1..6.mul` e `AnimationFrame1..6.uop` (mais de 46.000 entradas). Filtro completo de categorias (Monstros, Marinhos, Animais, Humanos/Equip, UOP Modernos), seleção dinâmica de ações com indicador `✓`, reprodução em 8 direções e exportação de spritesheets. |
| **💬 Cliloc** | Navegador de strings e textos localizados (`cliloc.enu`). Pesquisa rápida por ID numérico ou texto em mais de 100.000 entradas. |
| **🔊 Sounds** | Navegador e player de áudio PCM WAV para `sound.mul`. Reprodução nativa instantânea. |
| **🗺️ Map** | Visualizador interativo de mapas multifacetados (Felucca, Trammel, Ilshenar, Malas, Tokuno, Ter Mur). Canvas com preenchimento total da tela, arrasto fluido com o mouse e **zoom com a rodinha do mouse (*scroll*)** sincronizado com presets contextuais. |
| **🏰 Multis** | Estruturas multi-tile (casas, castelos, navios). Suporte a `multi.mul` clássico e `MultiCollection.uop` do Enhanced Client. Renderizador isométrico em tempo real com corte vertical Z e zoom pelo scroll do mouse. |
| **💡 Light** | Padrões de luz circulares e direcionais (`light.mul`) com visualizador de alto contraste. |
| **🔤 Fonts** | Fontes bitmap ASCII (`fonts.mul`) com campo de teste de digitação interativo. |
| **⭐ Skills** | Tabela de habilidades do jogo (`skills.mul`) com flags de botões de ação e títulos. |
| **🗣️ Speech** | Palavras-chave e gatilhos de comandos de voz (`speech.mul`). |
| **🎬 AnimData** | Tabela de taxas de quadros e intervalos de animação (`animdata.mul`) com pré-visualização estática do item. |
| **🎮 GameData** | Inspetor de configurações do cliente 3D Enhanced (`GameData.uop`). Decodifica todas as **79 tabelas de configuração** com modos **📊 Tabela Estruturada** e **📝 Texto Raw**, busca interna por linhas e exportador em lote para CSV. |

---

## 🛠️ Como Compilar e Executar

### Pré-requisitos
- [Rust toolchain](https://rustup.rs/) (versão 1.75 ou superior recomendada)
- Bibliotecas gráficas padrão do sistema (`libx11`, `libxcursor`, `libxrandr`, `libxi`, `libasound2` no Linux)

### Compilando e Executando
```bash
# Clone o repositório
git clone https://github.com/mbnunes/UOFiddlerPy.git
cd UOFiddlerPy

# Execute diretamente através do script inicializador
./run.sh

# Ou compile e execute com cargo
cargo run --release -p uo_gui
```

### Executando Testes e Benchmarks
```bash
cargo test --workspace
```

---

## 🏗️ Estrutura do Projeto

```
UOFiddlerPy/
├── Cargo.toml                  # Manifesto do workspace Rust
├── crates/
│   ├── uo_core/                # Decodificadores de alta performance (100% Rust Seguro)
│   │   ├── src/
│   │   │   ├── art.rs          # Decodificação de Art clássica e DDS do EC
│   │   │   ├── animations.rs   # Decodificador de sequências de animação MUL e UOP
│   │   │   ├── gamedata.rs     # Parser de GameData.uop (79 tabelas e contêineres)
│   │   │   ├── multis.rs       # Renderizador isométrico de multis e coleções uop
│   │   │   ├── maps.rs         # Leitor de blocos de mapa e minimapas
│   │   │   ├── tiledata.rs     # Parser de TileData 32-bit e 64-bit High Seas
│   │   │   ├── gumps.rs        # Decodificador de Gumps clássicos e UOP
│   │   │   └── ...
│   └── uo_gui/                 # Interface gráfica nativa acelerada (egui / eframe)
│       └── src/main.rs         # 18 abas, virtualização, navegação por teclado, tema escuro
├── Animationlist.xml           # Definições de nomes de criaturas e categorias
├── Multilist.xml               # Metadados e nomes das estruturas multis
├── run.sh                      # Inicializador universal
└── README.md
```

---

## 📜 Licença
Código aberto sob a licença MIT. Contribuições e sugestões são bem-vindas!
