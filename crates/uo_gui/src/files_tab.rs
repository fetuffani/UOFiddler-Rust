//! "Arquivos" tab: browser for every file in the client folder with a viewer that fits each kind
//! (text / def tables, IFF localized strings, UOP container, Multimap.rle image, hex dump).

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use uo_core::*;

use crate::UoTab;

struct Entry {
    name: String,
    path: PathBuf,
    enhanced: bool,
    size: u64,
    kind: ClientFileKind,
}

enum Content {
    None,
    Text { text: String, rows: Vec<Vec<String>> },
    Iff(IffText),
    Uop { file: UopFile },
    Image { texture: egui::TextureHandle, w: u32, h: u32, note: String },
    Hex,
    Error(String),
}

pub struct FilesTab {
    entries: Vec<Entry>,
    loaded_dir: Option<PathBuf>,
    search: String,
    kind_filter: Option<ClientFileKind>,
    selected: Option<usize>,
    content: Content,
    table_mode: bool,
    content_search: String,
    // hex view
    hex_offset: u64,
    hex_bytes: Vec<u8>,
    // uop view
    uop_selected: Option<usize>,
    uop_preview: Option<(String, bool)>, // (text, is_text)
    uop_search: String,
    image_zoom: f32,
}

impl Default for FilesTab {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            loaded_dir: None,
            search: String::new(),
            kind_filter: None,
            selected: None,
            content: Content::None,
            table_mode: true,
            content_search: String::new(),
            hex_offset: 0,
            hex_bytes: Vec::new(),
            uop_selected: None,
            uop_preview: None,
            uop_search: String::new(),
            image_zoom: 0.4,
        }
    }
}

pub fn hex_dump(data: &[u8], base: u64) -> String {
    let mut s = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        s.push_str(&format!("{:08X}  ", base + (i * 16) as u64));
        for j in 0..16 {
            match chunk.get(j) {
                Some(b) => s.push_str(&format!("{:02X} ", b)),
                None => s.push_str("   "),
            }
            if j == 7 {
                s.push(' ');
            }
        }
        s.push(' ');
        for &b in chunk {
            s.push(if (32..127).contains(&b) { b as char } else { '.' });
        }
        s.push('\n');
    }
    s
}

fn read_range(path: &Path, offset: u64, len: usize) -> Vec<u8> {
    let Ok(mut f) = std::fs::File::open(path) else { return Vec::new() };
    if f.seek(SeekFrom::Start(offset)).is_err() {
        return Vec::new();
    }
    let mut buf = vec![0u8; len];
    let n = f.read(&mut buf).unwrap_or(0);
    buf.truncate(n);
    buf
}

fn human(n: u64) -> String {
    if n >= 1 << 20 {
        format!("{:.1} MB", n as f64 / (1 << 20) as f64)
    } else if n >= 1 << 10 {
        format!("{:.1} KB", n as f64 / 1024.0)
    } else {
        format!("{} B", n)
    }
}

impl FilesTab {
    fn refresh(&mut self, fm: &FileManager) {
        self.entries.clear();
        for (name, path, enhanced) in fm.list_files() {
            let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            let head = read_range(&path, 0, 8);
            let kind = classify_file(&name, Some(&head));
            self.entries.push(Entry { name, path, enhanced, size, kind });
        }
        self.loaded_dir = fm.uo_dir.clone();
        self.selected = None;
        self.content = Content::None;
    }

    fn open(&mut self, idx: usize, ctx: &egui::Context) {
        self.selected = Some(idx);
        self.uop_selected = None;
        self.uop_preview = None;
        self.hex_offset = 0;
        self.content_search.clear();
        let (path, name, kind, size) = {
            let e = &self.entries[idx];
            (e.path.clone(), e.name.clone(), e.kind, e.size)
        };
        self.hex_bytes = read_range(&path, 0, 4096);
        let lower = name.to_lowercase();
        self.content = match kind {
            ClientFileKind::Iff => match std::fs::read(&path).ok().and_then(|d| parse_iff_text(&d)) {
                Some(t) => Content::Iff(t),
                None => Content::Hex,
            },
            ClientFileKind::Uop => match read_uop(&path) {
                Some(mut u) => {
                    resolve_uop_names(&mut u, &name);
                    Content::Uop { file: u }
                }
                None => Content::Error("UOP inválido".into()),
            },
            _ if lower == "multimap.rle" => match std::fs::read(&path).ok().and_then(|d| decode_multimap(&d, true)) {
                Some(m) => {
                    let img = egui::ColorImage::from_rgba_unmultiplied([m.width as usize, m.height as usize], &m.rgba);
                    Content::Image {
                        texture: ctx.load_texture("multimap_rle", img, egui::TextureOptions::NEAREST),
                        w: m.width,
                        h: m.height,
                        note: "Mapa de overview do cliente (RLE 1 bit: bit7 = terra, bits 0-6 = comprimento do run)".into(),
                    }
                }
                None => Content::Hex,
            },
            _ if lower.starts_with("facet") && lower.ends_with(".mul") => {
                match std::fs::read(&path).ok().and_then(|d| decode_facet(&d)) {
                    Some(m) => {
                        let img = egui::ColorImage::from_rgba_unmultiplied([m.width as usize, m.height as usize], &m.rgba);
                        Content::Image {
                            texture: ctx.load_texture("facet_img", img, egui::TextureOptions::LINEAR),
                            w: m.width,
                            h: m.height,
                            note: "Mapa da facet (linhas: u32 tamanho + registros run:u8/cor:u16 em 5:5:5)".into(),
                        }
                    }
                    None => Content::Hex,
                }
            }
            ClientFileKind::Text | ClientFileKind::Def | ClientFileKind::Other | ClientFileKind::Binary
                if size <= 4 << 20 =>
            {
                match std::fs::read(&path) {
                    Ok(d) if looks_like_text(&d) => {
                        let text = decode_text(&d);
                        let rows = parse_table(&text);
                        Content::Text { text, rows }
                    }
                    Ok(_) => Content::Hex,
                    Err(e) => Content::Error(e.to_string()),
                }
            }
            _ => Content::Hex,
        };
        if let Content::Text { rows, .. } = &self.content {
            self.table_mode = rows.len() > 3 && kind != ClientFileKind::Other;
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, fm: &FileManager) -> Option<UoTab> {
        if self.loaded_dir != fm.uo_dir || self.entries.is_empty() {
            self.refresh(fm);
            // UOFIDDLER_OPEN=<file name> opens a file on startup (used for automated screenshots)
            if let Ok(n) = std::env::var("UOFIDDLER_OPEN") {
                if let Some(i) = self.entries.iter().position(|e| e.name.eq_ignore_ascii_case(n.trim())) {
                    self.open(i, ctx);
                }
            }
        }
        let mut goto = None;
        let mut to_open: Option<usize> = None;

        egui::SidePanel::left("files_left").resizable(true).default_width(330.0).show_inside(ui, |ui| {
            ui.heading("📁 Arquivos do cliente");
            ui.label(format!("{} arquivos", self.entries.len()));
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.text_edit_singleline(&mut self.search);
            });
            ui.horizontal_wrapped(|ui| {
                if ui.selectable_label(self.kind_filter.is_none(), "Todos").clicked() {
                    self.kind_filter = None;
                }
                let mut kinds: Vec<ClientFileKind> = self.entries.iter().map(|e| e.kind).collect();
                kinds.sort();
                kinds.dedup();
                for k in kinds {
                    let n = self.entries.iter().filter(|e| e.kind == k).count();
                    if ui.selectable_label(self.kind_filter == Some(k), format!("{} ({})", k.label(), n)).clicked() {
                        self.kind_filter = Some(k);
                    }
                }
            });
            ui.separator();
            let q = self.search.to_lowercase();
            let visible: Vec<usize> = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, e)| {
                    (self.kind_filter.is_none() || self.kind_filter == Some(e.kind))
                        && (q.is_empty() || e.name.to_lowercase().contains(&q))
                })
                .map(|(i, _)| i)
                .collect();
            egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(ui, 20.0, visible.len(), |ui, range| {
                for vi in range {
                    let i = visible[vi];
                    let e = &self.entries[i];
                    let label = format!("{}{}   [{}]  {}", if e.enhanced { "✨ " } else { "" }, e.name, e.kind.label(), human(e.size));
                    if ui.selectable_label(self.selected == Some(i), label).clicked() {
                        to_open = Some(i);
                    }
                }
            });
        });
        if let Some(i) = to_open {
            self.open(i, ctx);
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let Some(sel) = self.selected else {
                ui.heading("Selecione um arquivo à esquerda");
                ui.label("Texto/def (tabela ou texto), localização IFF, UOP (entradas), Multimap.rle (imagem) ou hex.");
                return;
            };
            let (name, path, kind, size) = {
                let e = &self.entries[sel];
                (e.name.clone(), e.path.clone(), e.kind, e.size)
            };
            ui.heading(&name);
            ui.label(format!("{}  |  {}  |  {}", kind.label(), human(size), path.display()));
            if kind == ClientFileKind::Cliloc && ui.button("Abrir na aba Cliloc").clicked() {
                goto = Some(UoTab::Cliloc);
            }
            ui.separator();
            self.show_content(ui, &path, kind);
        });
        goto
    }

    fn show_content(&mut self, ui: &mut egui::Ui, path: &Path, kind: ClientFileKind) {
        match &self.content {
            Content::None => {}
            Content::Error(e) => {
                ui.colored_label(egui::Color32::LIGHT_RED, e);
            }
            Content::Text { .. } => self.show_text(ui),
            Content::Iff(_) => self.show_iff(ui),
            Content::Uop { .. } => self.show_uop(ui, path),
            Content::Image { .. } => self.show_image(ui),
            Content::Hex => self.show_hex(ui, path, kind),
        }
    }

    fn show_text(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.table_mode, true, "📊 Tabela");
            ui.selectable_value(&mut self.table_mode, false, "📄 Texto");
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.content_search);
        });
        let Content::Text { text, rows, .. } = &self.content else { return };
        let q = self.content_search.to_lowercase();
        if self.table_mode {
            let cols = rows.iter().map(|r| r.len()).max().unwrap_or(1).clamp(1, 14);
            let shown: Vec<&Vec<String>> =
                rows.iter().filter(|r| q.is_empty() || r.iter().any(|c| c.to_lowercase().contains(&q))).collect();
            ui.label(format!("{} linhas", shown.len()));
            let mut tb = TableBuilder::new(ui).striped(true).max_scroll_height(f32::INFINITY);
            for _ in 0..cols - 1 {
                tb = tb.column(Column::auto().at_least(50.0).clip(true));
            }
            tb.column(Column::remainder().at_least(80.0).clip(true)).body(|body| {
                body.rows(18.0, shown.len(), |mut row| {
                    let r = shown[row.index()];
                    for c in 0..cols {
                        row.col(|ui| {
                            ui.monospace(r.get(c).map(|s| s.as_str()).unwrap_or(""));
                        });
                    }
                });
            });
        } else {
            let filtered: String = if q.is_empty() {
                text.clone()
            } else {
                text.lines().filter(|l| l.to_lowercase().contains(&q)).collect::<Vec<_>>().join("\n")
            };
            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                ui.add(egui::Label::new(egui::RichText::new(filtered).monospace()).extend());
            });
        }
    }

    fn show_iff(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.content_search);
        });
        let Content::Iff(t) = &self.content else { return };
        ui.label(format!(
            "Idioma: {}   |   Recurso: {}   |   Strings: {} (declaradas: {})",
            t.lang, t.name, t.strings.len(), t.declared_count
        ));
        let q = self.content_search.to_lowercase();
        let rows: Vec<(usize, &String)> =
            t.strings.iter().enumerate().filter(|(_, s)| q.is_empty() || s.to_lowercase().contains(&q)).collect();
        TableBuilder::new(ui)
            .striped(true)
            .max_scroll_height(f32::INFINITY)
            .column(Column::exact(60.0))
            .column(Column::remainder().clip(true))
            .header(20.0, |mut h| {
                h.col(|ui| {
                    ui.strong("#");
                });
                h.col(|ui| {
                    ui.strong("Texto");
                });
            })
            .body(|body| {
                body.rows(18.0, rows.len(), |mut row| {
                    let (i, s) = rows[row.index()];
                    row.col(|ui| {
                        ui.monospace(i.to_string());
                    });
                    row.col(|ui| {
                        ui.label(s.as_str());
                    });
                });
            });
    }

    fn show_uop(&mut self, ui: &mut egui::Ui, path: &Path) {
        let Content::Uop { file } = &self.content else { return };
        let named = file.entries.iter().filter(|e| e.name.is_some()).count();
        ui.label(format!(
            "MYP v{}  |  {} entradas ({} com nome resolvido)",
            file.version,
            file.entries.len(),
            named
        ));
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.uop_search).on_hover_text("nome, hash (hex) ou índice");
        });
        let q = self.uop_search.to_lowercase();
        let rows: Vec<usize> = file
            .entries
            .iter()
            .enumerate()
            .filter(|(i, e)| {
                q.is_empty()
                    || e.name.as_deref().map_or(false, |n| n.to_lowercase().contains(&q))
                    || format!("{:016x}", e.hash).contains(&q)
                    || i.to_string() == q
            })
            .map(|(i, _)| i)
            .collect();

        let mut clicked = None;
        let h = ui.available_height();
        egui::TopBottomPanel::bottom("uop_preview").resizable(true).default_height((h * 0.4).max(120.0)).show_inside(ui, |ui| {
            match &self.uop_preview {
                Some((t, _)) => {
                    egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                        ui.add(egui::Label::new(egui::RichText::new(t.as_str()).monospace()).extend());
                    });
                }
                None => {
                    ui.label("Selecione uma entrada para ver o conteúdo (texto ou hex).");
                }
            }
        });
        TableBuilder::new(ui)
            .striped(true)
            .sense(egui::Sense::click())
            .max_scroll_height(f32::INFINITY)
            .column(Column::exact(60.0))
            .column(Column::exact(150.0))
            .column(Column::exact(80.0))
            .column(Column::exact(80.0))
            .column(Column::remainder().clip(true))
            .header(20.0, |mut hd| {
                for t in ["#", "Hash", "Comp.", "Descomp.", "Nome"] {
                    hd.col(|ui| {
                        ui.strong(t);
                    });
                }
            })
            .body(|body| {
                body.rows(18.0, rows.len(), |mut row| {
                    let i = rows[row.index()];
                    let e = &file.entries[i];
                    row.set_selected(self.uop_selected == Some(i));
                    row.col(|ui| {
                        ui.monospace(i.to_string());
                    });
                    row.col(|ui| {
                        ui.monospace(format!("{:016X}", e.hash));
                    });
                    row.col(|ui| {
                        ui.label(format!("{}{}", e.compressed_len, if e.compressed { " z" } else { "" }));
                    });
                    row.col(|ui| {
                        ui.label(e.decompressed_len.to_string());
                    });
                    row.col(|ui| {
                        ui.label(e.name.as_deref().unwrap_or("?"));
                    });
                    if row.response().clicked() {
                        clicked = Some(i);
                    }
                });
            });
        if let Some(i) = clicked {
            self.uop_selected = Some(i);
            if let Content::Uop { file } = &self.content {
                self.uop_preview = read_uop_entry(path, &file.entries[i]).map(|d| {
                    if looks_like_text(&d) {
                        let t = decode_text(&d);
                        (t.chars().take(60_000).collect(), true)
                    } else if d.len() >= 2 && d[0] == 0xFF && d[1] == 0xFE {
                        (decode_text(&d).chars().take(60_000).collect(), true)
                    } else {
                        (hex_dump(&d[..d.len().min(8192)], 0), false)
                    }
                });
            }
        }
    }

    fn show_image(&mut self, ui: &mut egui::Ui) {
        let Content::Image { texture, w, h, note } = &self.content else { return };
        ui.label(format!("{}x{} px  -  {}", w, h, note));
        ui.add(egui::Slider::new(&mut self.image_zoom, 0.1..=3.0).text("zoom"));
        let size = egui::vec2(*w as f32, *h as f32) * self.image_zoom;
        egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            ui.image((texture.id(), size));
        });
    }

    fn show_hex(&mut self, ui: &mut egui::Ui, path: &Path, kind: ClientFileKind) {
        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        let hint = match kind {
            ClientFileKind::Mul | ClientFileKind::Idx => "Formato MUL/IDX: veja a aba específica (Items, Gumps, Maps...) para o conteúdo interpretado.",
            ClientFileKind::Executable => "Executável/biblioteca: somente hex.",
            _ => "Formato binário sem visualizador dedicado: exibindo hex.",
        };
        ui.label(hint);
        ui.horizontal(|ui| {
            if ui.button("⏮").clicked() {
                self.hex_offset = 0;
            }
            if ui.button("◀ 4 KB").clicked() {
                self.hex_offset = self.hex_offset.saturating_sub(4096);
            }
            if ui.button("4 KB ▶").clicked() && self.hex_offset + 4096 < size {
                self.hex_offset += 4096;
            }
            ui.label(format!("offset 0x{:X} / 0x{:X}", self.hex_offset, size));
        });
        let bytes = read_range(path, self.hex_offset, 4096);
        let text = hex_dump(&bytes, self.hex_offset);
        egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            ui.add(egui::Label::new(egui::RichText::new(text).monospace()).extend());
        });
    }
}
