# UOFiddler 🗡️

> 🌐 **Language / Idioma**: **English** | [Português (README_pt-br.md)](README_pt-br.md)  
> 👤 **Author**: Mauricio Nunes + DeepSeek & Gemini  
> 🔗 **GitHub**: [https://github.com/mbnunes/UOFiddler-Rust](https://github.com/mbnunes/UOFiddler-Rust)

**UOFiddler** is a complete, modern, high-performance **100% native Rust** reimplementation of the classic **UOFiddler** for Ultima Online. Built on **egui / eframe**, it runs smoothly, stably, and elegantly on **Linux, Windows, and macOS**, completely eliminating dependencies on the **.NET runtime**, **Mono**, **Wine**, or **Python**.

---

## ⚡ Highlights & Why UOFiddler?

Traditional UOFiddler on Linux via Wine or Mono is prone to graphical glitches, crashes, and path casing errors. **UOFiddler** provides:

1. **100% Native Rust**: Zero runtime dependencies. Memory-safe, instant startup (< 50 ms), and microscopic memory footprint.
2. **Dual-Client Engine (Classic & Enhanced UOP)**: Seamless live toggling between **Classic 2D** (`.mul`/`.idx`) and **Enhanced Client 3D** (`.uop` Mythic MYP v5 / DDS textures).
3. **Hardware-Accelerated UI (egui/eframe)**: Smooth 60+ FPS navigation across 65,536 static items, 16,384 land tiles, 79 GameData tables, and huge multis.
4. **Intelligent Path Detection**: Automatically discovers Ultima Online Classic and Enhanced installations across standard locations (`/opt/hd_jogos/`, `~/.wine/`, Lutris, Steam, local directories) with case-insensitive file resolution.
5. **Universal Keyboard Navigation**: Full keyboard control (Arrow Keys, PageUp, PageDown) with smooth automatic scrolling across lists and grids in all tabs.
6. **Native Audio Engine**: Direct PCM WAV playback using `rodio` over PipeWire, PulseAudio, and ALSA without external audio drivers.
7. **Vertical Zoom Slider Tool**: 1:1 true-size rendering with vertical sliders (0.5x to 5.0x) on previews for Items, Land Tiles, TileData, Textures, and Animations.

---

## 📑 Comprehensive Overview of all 18 Tabs

| Tab | Description |
|---|---|
| **⚔️ Items** | Static items viewer with **Grid** and **List** modes. Live toggle between Classic 2D Art and Enhanced Client DDS textures (`LegacyTexture.uop`). Hue tester (3,000 hues), TileData attributes, and vertical zoom tool. |
| **🌿 Land Tiles** | 16,384 terrain tiles with diamond geometry (44x44 rhombus). Dual-engine switch for Classic vs Enhanced 3D terrain textures (`Texture.uop`), terrain flags, and zoom slider. |
| **📋 TileData** | Complete editor for ItemData (65,536 entries) and LandData (16,384 entries). Support for both classic 32-bit flags and **High Seas 64-bit flags**. Grid and technical table view with true-size preview. |
| **🖼️ Gumps** | Full browser for classic `gumpart.mul` and modern `gumpartLegacyMUL.uop`. Container bounding box overlays (71 containers), search, and vertical zoom tool. |
| **🎨 Hues** | Complete palette of all 3,000 Ultima Online hues. Color bar gradient inspector with individual 32-color RGB555 swatches and table view. |
| **🧱 Textures** | Raw 64x64 and 128x128 surface textures from `texmaps.mul`. Grid and list view with 1:1 preview and vertical zoom tool. |
| **🎯 RadarCol** | All 81,920 minimap radar colors mapped to items and terrain. Real-time RGB color preview and 1:1 tile art inspection. |
| **🎭 Animations** | Complete creature animation player supporting `anim1..6.mul` and `AnimationFrame1..6.uop` (46,000+ entries). Full category filtering (Monstros, Marinhos, Animais, Humanos/Equip, UOP Modernos), dynamic action selection with `✓` indicators, directional playback (8 directions), and sprite sheet exporter. |
| **💬 Cliloc** | High-speed string localization browser (`cliloc.enu`). Search across 100,000+ entries by ID or localized text. |
| **🔊 Sounds** | PCM sound browser and player for `sound.mul`. Direct audio playback with wave information. |
| **🗺️ Map** | Interactive multi-faceted map viewer (Felucca, Trammel, Ilshenar, Malas, Tokuno, Ter Mur). Full canvas filling, mouse drag panning, and **mouse wheel scroll zoom** (0.5x to 4.0x) with contextual presets. |
| **🏰 Multis** | Multi-tile structures browser (houses, castles, ships). Supports classic `multi.mul` and Enhanced Client `MultiCollection.uop`. Real-time isometric software ray-assembler with multi-height Z slicing and mouse wheel zoom. |
| **💡 Light** | Circular and directional light patterns (`light.mul`) with high-contrast preview. |
| **🔤 Fonts** | Bitmap ASCII font browser (`fonts.mul`) with interactive text typing preview. |
| **⭐ Skills** | Skills definition table (`skills.mul`) with action button flags and text entries. |
| **🗣️ Speech** | Speech keywords and command triggers (`speech.mul`). |
| **🎬 AnimData** | Frame animation timing and delay tables (`animdata.mul`) with live static art preview. |
| **🎮 GameData** | Enhanced Client 3D configuration inspector (`GameData.uop`). Decodes all 79 configuration tables with **Structured Grid** and **Raw Monospace** views, row filtering, and batch CSV export. |

---

## 🛠️ Build and Run

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (version 1.75+ recommended)
- Standard system graphics libraries (`libx11`, `libxcursor`, `libxrandr`, `libxi`, `libasound2` on Linux)

### Building and Running
```bash
# Clone the repository
git clone https://github.com/mbnunes/UOFiddler-Rust.git
cd UOFiddler-Rust

# Run directly using the launch script
./run.sh

# Or compile and run with cargo
cargo run --release -p uo_gui
```

### Running Tests and Benchmarks
```bash
cargo test --workspace
```

---

## 🏗️ Architecture

```
UOFiddler-Rust/
├── Cargo.toml                  # Workspace manifest
├── crates/
│   ├── uo_core/                # High-performance UO format decoders (100% safe Rust)
│   │   ├── src/
│   │   │   ├── art.rs          # Classic Art & EC DDS decoding (image-rs dds)
│   │   │   ├── animations.rs   # MUL & UOP animation sequence decoder
│   │   │   ├── gamedata.rs     # GameData.uop parser (79 tables & containers)
│   │   │   ├── multis.rs       # Isometric multi renderer & uop collection
│   │   │   ├── maps.rs         # Minimap and block-level map reader
│   │   │   ├── tiledata.rs     # 32-bit & 64-bit TileData parser
│   │   │   ├── gumps.rs        # Classic & UOP Gump decoder
│   │   │   └── ...
│   └── uo_gui/                 # Native hardware-accelerated GUI (egui / eframe)
│       └── src/main.rs         # 18 tabs, virtualization, keyboard navigation, theme
├── Animationlist.xml           # Creature animation definitions
├── Multilist.xml               # Multi house names and metadata
├── run.sh                      # Universal launcher
└── README.md
```

---

## 📜 License
Open-source under the MIT License. Contributions and PRs are welcome!
