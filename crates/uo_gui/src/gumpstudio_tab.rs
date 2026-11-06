//! GumpStudio tab: visual gump designer (after https://github.com/mbnunes/GumpStudio) with a
//! live preview drawn from the real client art, and exporters for RunUO, Sphere and POL.

use eframe::egui;
use std::collections::HashMap;
use uo_core::*;

use crate::paperdoll_tab::decode_gump_id;

pub struct GumpStudioData<'a> {
    pub file_manager: &'a FileManager,
    pub ascii_fonts: Option<&'a FontsManager>,
    pub gump_index: Option<&'a FileIndex>,
    pub art: Option<&'a FileIndex>,
    pub hues: Option<&'a HuesManager>,
    pub cliloc: Option<&'a ClilocManager>,
    /// language of the loaded cliloc (cache key)
    pub cliloc_lang: &'a str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    RunUO,
    Sphere,
    Pol,
}

impl Lang {
    fn label(&self) -> &'static str {
        match self {
            Self::RunUO => "RunUO (C#)",
            Self::Sphere => "Sphere (dialog)",
            Self::Pol => "POL (gumps.inc)",
        }
    }
    fn ext(&self) -> &'static str {
        match self {
            Self::RunUO => "cs",
            Self::Sphere => "scp",
            Self::Pol => "src",
        }
    }
}

type TexKey = (u8, u32, i32, i32);

pub struct GumpStudioTab {
    doc: GumpDoc,
    selected: Option<usize>,
    view_page: u32,
    zoom: f32,
    snap: i32,
    undo: Vec<GumpDoc>,
    lang: Lang,
    export_text: String,
    status: String,
    tex: HashMap<TexKey, Option<egui::TextureHandle>>,
    cliloc_cache: HashMap<u32, String>,
    cliloc_cache_lang: String,
    drag_started: bool,
    uni_fonts: HashMap<u8, Option<UnicodeFont>>,
    html_tex: HashMap<(i32, String), Option<egui::TextureHandle>>,
    html_color: [u8; 3],
    resizepic_open: bool,
    multi: Vec<usize>,
    favorites: Favorites,
    fav_name: String,
    resizepic_search: String,
    resizepic_list: Vec<u32>,
    resizepic_scan_pos: u32,
    text_tex: HashMap<(bool, u8, u32, i32, String), Option<egui::TextureHandle>>,
}

impl Default for GumpStudioTab {
    fn default() -> Self {
        let mut t = Self {
            doc: GumpDoc::default(),
            selected: None,
            view_page: 0,
            zoom: 1.0,
            snap: 1,
            undo: Vec::new(),
            lang: Lang::RunUO,
            export_text: String::new(),
            status: String::new(),
            tex: HashMap::new(),
            cliloc_cache: HashMap::new(),
            cliloc_cache_lang: String::new(),
            drag_started: false,
            uni_fonts: HashMap::new(),
            html_tex: HashMap::new(),
            html_color: [255, 80, 80],
            resizepic_open: false,
            multi: Vec::new(),
            favorites: Self::load_favorites(),
            fav_name: String::new(),
            resizepic_search: String::new(),
            resizepic_list: Vec::new(),
            resizepic_scan_pos: 0,
            text_tex: HashMap::new(),
        };
        t.doc = Self::sample_doc();
        // UOFIDDLER_GS_SELECT=<element index> selects an element on startup (automated screenshots)
        t.selected = std::env::var("UOFIDDLER_GS_SELECT").ok().and_then(|v| v.parse().ok());
        t
    }
}

impl GumpStudioTab {
    fn sample_doc() -> GumpDoc {
        let mut d = GumpDoc::default();
        d.name = "ExampleGump".into();
        let mk = |name: &str, kind, x, y, w, h| GumpElement { name: name.into(), x, y, w, h, page: 0, kind, repeat: None, group: 0 };
        d.elements = vec![
            mk("background", ElementKind::Background { gump_id: 9200 }, 0, 0, 300, 240),
            mk("title", ElementKind::Label { text: "Example gump".into(), hue: 52, font: 1, ascii: false }, 20, 15, 0, 0),
            mk("name_entry", ElementKind::TextEntry { hue: 0, text: "type here".into(), id: 0 }, 20, 50, 200, 20),
            mk("opt", ElementKind::Checkbox { off: 210, on: 211, checked: true, id: 0 }, 20, 85, 0, 0),
            mk("opt_text", ElementKind::Label { text: "Enable".into(), hue: 0, font: 1, ascii: false }, 50, 87, 0, 0),
            mk("ok", ElementKind::Button { normal: 247, pressed: 248, action: ButtonAction::Reply, page: 0, id: 0 }, 20, 130, 0, 0),
            mk("info", ElementKind::Html { text: "<center><b>HTML</b> no gump</center><basefont color=\"#ffd24a\">Texto <i>colorido</i> e <u>sublinhado</u></basefont><br><big>grande</big> <small>pequeno</small>".into(), cliloc: 0, background: true, scrollbar: false }, 20, 165, 260, 60),
            mk("ok_text", ElementKind::Label { text: "OK".into(), hue: 0, font: 1, ascii: false }, 55, 132, 0, 0),
        ];
        d
    }

    fn favorites_path() -> std::path::PathBuf {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default();
        std::path::PathBuf::from(home).join(".config/uofiddler/gumpstudio_favorites.json")
    }

    fn load_favorites() -> Favorites {
        std::fs::read_to_string(Self::favorites_path()).ok().and_then(|t| Favorites::from_json(&t).ok()).unwrap_or_default()
    }

    fn save_favorites(&self) {
        let p = Self::favorites_path();
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(p, self.favorites.to_json());
    }

    fn push_undo(&mut self) {
        if self.undo.len() >= 60 {
            self.undo.remove(0);
        }
        self.undo.push(self.doc.clone());
    }

    // ------------------------------------------------------------- textures

    fn hue_pal(d: &GumpStudioData, hue: u32) -> Option<[u32; 32]> {
        if hue == 0 {
            return None;
        }
        d.hues?.hues.get(hue as usize - 1).map(|h| h.get_rgba_palette())
    }

    fn upload(&mut self, ctx: &egui::Context, key: TexKey, bmp: Option<GumpBitmap>) -> Option<egui::TextureHandle> {
        let t = bmp.map(|b| {
            ctx.load_texture(
                format!("gs_{}_{}_{}_{}", key.0, key.1, key.2, key.3),
                egui::ColorImage::from_rgba_unmultiplied([b.width as usize, b.height as usize], &b.rgba),
                egui::TextureOptions::NEAREST,
            )
        });
        self.tex.insert(key, t.clone());
        t
    }

    fn gump_tex(&mut self, ctx: &egui::Context, d: &GumpStudioData, id: u32, hue: u32) -> Option<egui::TextureHandle> {
        let key = (0, id, hue as i32, 0);
        if let Some(t) = self.tex.get(&key) {
            return t.clone();
        }
        let bmp = d.gump_index.and_then(|gi| decode_gump_id(gi, id)).map(|mut b| {
            if let Some(p) = Self::hue_pal(d, hue) {
                apply_hue_bytes(&mut b.rgba, &p, true);
            }
            b
        });
        self.upload(ctx, key, bmp)
    }

    fn resize_tex(&mut self, ctx: &egui::Context, d: &GumpStudioData, id: u32, w: i32, h: i32) -> Option<egui::TextureHandle> {
        let key = (1, id, w, h);
        if let Some(t) = self.tex.get(&key) {
            return t.clone();
        }
        let bmp = d.gump_index.and_then(|gi| {
            let parts: Vec<GumpBitmap> = (0..9).filter_map(|i| decode_gump_id(gi, id + i)).collect();
            compose_resizepic(&parts, w, h)
        });
        self.upload(ctx, key, bmp)
    }

    fn tiled_tex(&mut self, ctx: &egui::Context, d: &GumpStudioData, id: u32, w: i32, h: i32) -> Option<egui::TextureHandle> {
        let key = (2, id, w, h);
        if let Some(t) = self.tex.get(&key) {
            return t.clone();
        }
        let bmp = d.gump_index.and_then(|gi| decode_gump_id(gi, id)).and_then(|b| compose_tiled(&b, w, h));
        self.upload(ctx, key, bmp)
    }

    fn item_tex(&mut self, ctx: &egui::Context, d: &GumpStudioData, id: u32, hue: u32) -> Option<egui::TextureHandle> {
        let key = (3, id, hue as i32, 0);
        if let Some(t) = self.tex.get(&key) {
            return t.clone();
        }
        let bmp = d.art.and_then(|a| a.get_data(0x4000 + id as usize)).and_then(|raw| decode_static(&raw)).map(|mut b| {
            if let Some(p) = Self::hue_pal(d, hue) {
                apply_hue_bytes(&mut b.rgba, &p, true);
            }
            GumpBitmap { width: b.width as u32, height: b.height as u32, rgba: b.rgba }
        });
        self.upload(ctx, key, bmp)
    }

    /// Text drawn with the client fonts: Unicode font `font` (unifont*.mul) or, when `ascii`, the
    /// ASCII font `font` of fonts.mul. The hue colours the glyphs like the client does.
    fn text_texture(&mut self, ctx: &egui::Context, d: &GumpStudioData, text: &str, font: u8, ascii: bool, hue: u32, wrap: Option<i32>) -> Option<egui::TextureHandle> {
        let key = (ascii, font, hue, wrap.unwrap_or(-1), text.to_string());
        if let Some(t) = self.text_tex.get(&key) {
            return t.clone();
        }
        let rendered = if ascii {
            d.ascii_fonts
                .and_then(|f| f.ascii_fonts.get(font as usize))
                .and_then(|f| f.as_ref())
                .and_then(|f| f.render_text(text))
                .map(|(w, h, mut rgba)| {
                    if let Some(p) = Self::hue_pal(d, hue) {
                        apply_hue_bytes(&mut rgba, &p, false);
                    }
                    (w, h, rgba)
                })
        } else {
            let f = self.uni_fonts.entry(font).or_insert_with(|| {
                let name = if font == 0 { "unifont.mul".to_string() } else { format!("unifont{}.mul", font) };
                d.file_manager.get_file_path(&name).and_then(|p| UnicodeFont::from_file(p).ok())
            });
            let c = Self::text_color(d, hue);
            f.as_ref().and_then(|f| f.render(text, [c.r(), c.g(), c.b(), 255], wrap))
        };
        let tex = rendered.map(|(w, h, rgba)| {
            ctx.load_texture(
                format!("gs_text_{}", self.text_tex.len()),
                egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba),
                egui::TextureOptions::NEAREST,
            )
        });
        self.text_tex.insert(key, tex.clone());
        tex
    }

    /// HTML text drawn like the client does (UO Unicode font 1, tags simulated), wrapped at `wrap` px.
    fn html_texture(&mut self, ctx: &egui::Context, d: &GumpStudioData, html: &str, wrap: i32) -> Option<egui::TextureHandle> {
        let key = (wrap, html.to_string());
        if let Some(t) = self.html_tex.get(&key) {
            return t.clone();
        }
        let f = self.uni_fonts.entry(1).or_insert_with(|| d.file_manager.get_file_path("unifont1.mul").and_then(|p| UnicodeFont::from_file(p).ok()));
        let rendered = f.as_ref().and_then(|f| render_html(f, html, [235, 235, 235, 255], wrap));
        let tex = rendered.map(|(w, h, rgba)| {
            ctx.load_texture(format!("gs_html_{}", self.html_tex.len()), egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba), egui::TextureOptions::NEAREST)
        });
        self.html_tex.insert(key, tex.clone());
        tex
    }

    fn cliloc_text(&mut self, d: &GumpStudioData, n: u32) -> String {
        if self.cliloc_cache_lang != d.cliloc_lang {
            self.cliloc_cache.clear();
            self.cliloc_cache_lang = d.cliloc_lang.to_string();
        }
        if let Some(t) = self.cliloc_cache.get(&n) {
            return t.clone();
        }
        let t = d
            .cliloc
            .and_then(|c| c.entries.iter().find(|e| e.number == n as i32))
            .map_or_else(|| format!("cliloc {}", n), |e| e.text.clone());
        self.cliloc_cache.insert(n, t.clone());
        t
    }

    fn text_color(d: &GumpStudioData, hue: u32) -> egui::Color32 {
        match Self::hue_pal(d, hue) {
            Some(p) => {
                let c = p[31];
                egui::Color32::from_rgb((c & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, ((c >> 16) & 0xFF) as u8)
            }
            None => egui::Color32::from_rgb(235, 235, 235),
        }
    }

    // ------------------------------------------------------------- UI

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: &GumpStudioData) {
        egui::TopBottomPanel::top("gs_top").min_height(64.0).show_inside(ui, |ui| self.toolbar(ui));

        if !self.export_text.is_empty() {
            egui::TopBottomPanel::bottom("gs_export").resizable(true).default_height(230.0).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong(format!("Código exportado - {}", self.lang.label()));
                    if ui.button("📋 Copiar").clicked() {
                        ctx.copy_text(self.export_text.clone());
                        self.status = "Código copiado".into();
                    }
                    if ui.button("💾 Salvar...").clicked() {
                        let fname = format!("{}.{}", self.doc.ident(), self.lang.ext());
                        if let Some(p) = rfd::FileDialog::new().set_file_name(&fname).save_file() {
                            self.status = match std::fs::write(&p, &self.export_text) {
                                Ok(_) => format!("Salvo em {}", p.display()),
                                Err(e) => format!("Erro ao salvar: {e}"),
                            };
                        }
                    }
                    if ui.button("✖ Fechar").clicked() {
                        self.export_text.clear();
                    }
                });
                egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                    ui.add(egui::Label::new(egui::RichText::new(&self.export_text).monospace()).extend());
                });
            });
        }

        egui::SidePanel::left("gs_left").resizable(true).default_width(250.0).show_inside(ui, |ui| self.element_list(ui));
        egui::SidePanel::right("gs_right").resizable(true).default_width(360.0).show_inside(ui, |ui| self.properties(ui, ctx, d));
        egui::CentralPanel::default().show_inside(ui, |ui| self.canvas(ui, ctx, d));
        if self.resizepic_open {
            self.resizepic_window(ctx, d);
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.strong("🧩 GumpStudio");
            ui.label("Nome:");
            ui.add(egui::TextEdit::singleline(&mut self.doc.name).desired_width(120.0));
            ui.label("X:");
            ui.add(egui::DragValue::new(&mut self.doc.x));
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut self.doc.y));
            ui.checkbox(&mut self.doc.movable, "movível");
            ui.checkbox(&mut self.doc.closable, "fechável");
            ui.checkbox(&mut self.doc.disposable, "descartável");
            ui.separator();
            if ui.button("🆕 Novo").clicked() {
                self.push_undo();
                self.doc = GumpDoc::default();
                self.selected = None;
            }
            if ui.button("📂 Abrir...").clicked() {
                if let Some(p) = rfd::FileDialog::new().add_filter("GumpStudio JSON", &["json"]).pick_file() {
                    match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|s| GumpDoc::from_json(&s)) {
                        Ok(doc) => {
                            self.push_undo();
                            self.doc = doc;
                            self.selected = None;
                            self.status = format!("Aberto {}", p.display());
                        }
                        Err(e) => self.status = format!("Erro ao abrir: {e}"),
                    }
                }
            }
            if ui.button("💾 Salvar...").clicked() {
                if let Some(p) = rfd::FileDialog::new().set_file_name(format!("{}.json", self.doc.ident())).save_file() {
                    self.status = match std::fs::write(&p, self.doc.to_json()) {
                        Ok(_) => format!("Salvo em {}", p.display()),
                        Err(e) => format!("Erro ao salvar: {e}"),
                    };
                }
            }
            if ui.add_enabled(!self.undo.is_empty(), egui::Button::new("↶ Desfazer")).clicked() {
                if let Some(prev) = self.undo.pop() {
                    self.doc = prev;
                    self.selected = None;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Exportar:");
            for l in [Lang::RunUO, Lang::Sphere, Lang::Pol] {
                if ui.button(l.label()).clicked() {
                    self.lang = l;
                    self.export_text = match l {
                        Lang::RunUO => export_runuo(&self.doc),
                        Lang::Sphere => export_sphere(&self.doc),
                        Lang::Pol => export_pol(&self.doc),
                    };
                }
            }
            ui.separator();
            ui.label("Página:");
            let pages = self.doc.pages();
            for p in &pages {
                let name = if *p == 0 { "0 (sempre)".to_string() } else { p.to_string() };
                if ui.selectable_label(self.view_page == *p, name).clicked() {
                    self.view_page = *p;
                }
            }
            if ui.small_button("+ página").clicked() {
                self.view_page = pages.last().copied().unwrap_or(0) + 1;
            }
            ui.separator();
            ui.label("Zoom:");
            ui.add(egui::Slider::new(&mut self.zoom, 0.5..=3.0));
            ui.label("Grade:");
            ui.add(egui::DragValue::new(&mut self.snap).range(1..=32));
            if !self.status.is_empty() {
                ui.separator();
                ui.label(&self.status);
            }
        });
        ui.add_space(2.0);
    }

    fn element_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Elementos");
        ui.label("Adicionar:");
        let mut add: Option<&str> = None;
        ui.horizontal_wrapped(|ui| {
            for (key, label) in [
                ("background", "Fundo"),
                ("image", "Imagem"),
                ("tiled", "Repetida"),
                ("item", "Item"),
                ("label", "Texto"),
                ("html", "HTML"),
                ("button", "Botão"),
                ("checkbox", "Check"),
                ("radio", "Radio"),
                ("entry", "Entrada"),
                ("alpha", "Alfa"),
            ] {
                if ui.small_button(label).clicked() {
                    add = Some(key);
                }
            }
        });
        if let Some(k) = add {
            if let Some((kind, w, h)) = ElementKind::default_for(k) {
                self.push_undo();
                let n = self.doc.elements.len();
                self.doc.elements.push(GumpElement {
                    name: format!("{}{}", k, n + 1),
                    x: 20,
                    y: 20,
                    w,
                    h,
                    page: self.view_page,
                    kind,
                    repeat: None,
                    group: 0,
                });
                self.selected = Some(n);
            }
        }
        // groups: Ctrl+click adds elements to the selection; grouped elements move together
        ui.label(t!("gs.group.hint"));
        ui.horizontal(|ui| {
            let mut sel: Vec<usize> = self.multi.clone();
            if let Some(s) = self.selected {
                if !sel.contains(&s) {
                    sel.push(s);
                }
            }
            if ui.add_enabled(sel.len() > 1, egui::Button::new(t!("gs.group.make"))).clicked() {
                self.push_undo();
                self.doc.group_elements(&sel);
            }
            let has_group = self.selected.and_then(|i| self.doc.elements.get(i)).map_or(false, |e| e.group != 0);
            if ui.add_enabled(has_group, egui::Button::new(t!("gs.group.dissolve"))).clicked() {
                if let Some(i) = self.selected {
                    self.push_undo();
                    for k in self.doc.group_members(i) {
                        self.doc.elements[k].group = 0;
                    }
                }
            }
        });
        // favorites: saved combinations of elements, inserted into any gump
        ui.collapsing(t!("gs.fav.title"), |ui| {
            ui.horizontal(|ui| {
                ui.label(t!("gs.fav.name"));
                ui.text_edit_singleline(&mut self.fav_name);
            });
            let name_or = |this: &Self, fallback: &str| if this.fav_name.trim().is_empty() { fallback.to_string() } else { this.fav_name.trim().to_string() };
            let mut changed = false;
            if ui.add_enabled(self.selected.map_or(false, |i| i < self.doc.elements.len()), egui::Button::new(t!("gs.fav.save_selected"))).clicked() {
                if let Some(e) = self.selected.and_then(|i| self.doc.elements.get(i)).cloned() {
                    let name = name_or(self, &e.name);
                    self.favorites.items.push(GumpFavorite::from_elements(&name, &[e]));
                    changed = true;
                }
            }
            if ui.add_enabled(!self.doc.elements.is_empty(), egui::Button::new(t!("gs.fav.save_all"))).clicked() {
                let name = name_or(self, &self.doc.name.clone());
                self.favorites.items.push(GumpFavorite::from_elements(&name, &self.doc.elements));
                changed = true;
            }
            if changed {
                self.fav_name.clear();
            }
            if self.favorites.items.is_empty() {
                ui.weak(t!("gs.fav.empty"));
            }
            let (mut insert, mut delete): (Option<usize>, Option<usize>) = (None, None);
            for (i, f) in self.favorites.items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} ({})", f.name, t!("gs.fav.count", n = f.elements.len())));
                    if ui.small_button(t!("gs.fav.insert")).clicked() {
                        insert = Some(i);
                    }
                    if ui.small_button(t!("gs.fav.delete")).clicked() {
                        delete = Some(i);
                    }
                });
            }
            if let Some(i) = insert {
                let mut placed = self.favorites.items[i].instantiate(20, 20, self.view_page);
                let base = self.doc.elements.iter().map(|e| e.group).max().unwrap_or(0);
                for e in placed.iter_mut().filter(|e| e.group != 0) {
                    e.group += base;
                }
                self.push_undo();
                let first = self.doc.elements.len();
                self.doc.elements.extend(placed);
                self.selected = (first < self.doc.elements.len()).then_some(first);
            }
            if let Some(i) = delete {
                self.favorites.items.remove(i);
                changed = true;
            }
            if changed {
                self.save_favorites();
            }
        });
        ui.separator();
        let mut action: Option<(usize, i8)> = None; // -1 up, 1 down, 2 dup, 3 delete
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // topmost element first
            for i in (0..self.doc.elements.len()).rev() {
                let e = &self.doc.elements[i];
                ui.horizontal(|ui| {
                    let text = format!("{}{} · {}", if e.page > 0 { format!("p{} ", e.page) } else { String::new() }, e.name, e.kind.label());
                    if ui.selectable_label(self.selected == Some(i), text).clicked() {
                        self.selected = Some(i);
                    }
                    if self.selected == Some(i) {
                        if ui.small_button("⬆").on_hover_text("Trazer para frente").clicked() {
                            action = Some((i, 1));
                        }
                        if ui.small_button("⬇").on_hover_text("Enviar para trás").clicked() {
                            action = Some((i, -1));
                        }
                        if ui.small_button("⎘").on_hover_text("Duplicar").clicked() {
                            action = Some((i, 2));
                        }
                        if ui.small_button("🗑").on_hover_text("Excluir").clicked() {
                            action = Some((i, 3));
                        }
                    }
                });
            }
        });
        if let Some((i, a)) = action {
            self.push_undo();
            match a {
                1 if i + 1 < self.doc.elements.len() => {
                    self.doc.elements.swap(i, i + 1);
                    self.selected = Some(i + 1);
                }
                -1 if i > 0 => {
                    self.doc.elements.swap(i, i - 1);
                    self.selected = Some(i - 1);
                }
                2 => {
                    let mut c = self.doc.elements[i].clone();
                    c.x += 10;
                    c.y += 10;
                    c.name = format!("{}_copy", c.name);
                    self.doc.elements.insert(i + 1, c);
                    self.selected = Some(i + 1);
                }
                3 => {
                    self.doc.elements.remove(i);
                    self.selected = None;
                }
                _ => {}
            }
        }
    }

    fn gump_field(ui: &mut egui::Ui, label: &str, v: &mut u32) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.add(egui::DragValue::new(v).range(0..=65535));
        });
    }

    fn properties(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: &GumpStudioData) {
        ui.heading("Propriedades");
        let Some(i) = self.selected.filter(|&i| i < self.doc.elements.len()) else {
            ui.label("Selecione um elemento no canvas ou na lista.");
            return;
        };
        let before = self.doc.elements[i].clone();
        let mut preview: Option<(u32, bool)> = None; // (gump id, resizepic?)
        {
            let e = &mut self.doc.elements[i];
            ui.label(e.kind.label());
            ui.horizontal(|ui| {
                ui.label("Nome:");
                ui.text_edit_singleline(&mut e.name);
            });
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut e.x));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut e.y));
            });
            if e.kind.is_sized() {
                ui.horizontal(|ui| {
                    ui.label("Largura:");
                    ui.add(egui::DragValue::new(&mut e.w).range(1..=2000));
                    ui.label("Altura:");
                    ui.add(egui::DragValue::new(&mut e.h).range(1..=2000));
                });
            }
            ui.horizontal(|ui| {
                ui.label("Página:");
                ui.add(egui::DragValue::new(&mut e.page).range(0..=50));
            });
            ui.horizontal(|ui| {
                ui.label(t!("gs.group.label"));
                ui.add(egui::DragValue::new(&mut e.group).range(0..=999));
            });
            // repeated element: exported as a loop in RunUO / POL / Sphere
            let mut repeating = e.repeat.is_some();
            if ui
                .checkbox(&mut repeating, t!("gs.repeat"))
                .on_hover_text(t!("gs.repeat.tip"))
                .changed()
            {
                e.repeat = repeating.then_some(Repeat { count: 3, step_x: 0, step_y: 25 });
            }
            if let Some(r) = &mut e.repeat {
                ui.horizontal(|ui| {
                    ui.label(t!("gs.repeat.times"));
                    ui.add(egui::DragValue::new(&mut r.count).range(2..=200));
                    ui.label(t!("gs.repeat.step_x"));
                    ui.add(egui::DragValue::new(&mut r.step_x));
                    ui.label(t!("gs.repeat.step_y"));
                    ui.add(egui::DragValue::new(&mut r.step_y));
                });
            }
            ui.separator();
            match &mut e.kind {
                ElementKind::Background { gump_id } => {
                    Self::gump_field(ui, "Gump (resizepic):", gump_id);
                    if ui.button(t!("gs.resizepic.browse")).clicked() {
                        self.resizepic_open = true;
                    }
                    preview = Some((*gump_id, true));
                }
                ElementKind::Image { gump_id, hue } => {
                    Self::gump_field(ui, "Gump:", gump_id);
                    Self::gump_field(ui, "Hue:", hue);
                    preview = Some((*gump_id, false));
                }
                ElementKind::Tiled { gump_id } => {
                    Self::gump_field(ui, "Gump:", gump_id);
                    preview = Some((*gump_id, false));
                }
                ElementKind::Item { item_id, hue } => {
                    Self::gump_field(ui, "Item id:", item_id);
                    Self::gump_field(ui, "Hue:", hue);
                }
                ElementKind::Label { text, hue, font, ascii } => {
                    ui.label("Texto:");
                    ui.text_edit_multiline(text);
                    Self::gump_field(ui, "Hue:", hue);
                    ui.horizontal(|ui| {
                        ui.label("Fonte:");
                        ui.selectable_value(ascii, false, "Unicode");
                        ui.selectable_value(ascii, true, "ASCII");
                        let max = if *ascii { 9 } else { 12 };
                        let mut f = (*font).min(max) as u32;
                        ui.add(egui::DragValue::new(&mut f).range(0..=max as u32));
                        *font = f as u8;
                    })
                    .response
                    .on_hover_text("Fonte do cliente usada na pré-visualização; não altera o código exportado");
                }
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    Self::gump_field(ui, "Cliloc (0 = texto):", cliloc);
                    if *cliloc == 0 {
                        ui.label("HTML:");
                        let id = egui::Id::new(("gs_html_text", i));
                        let n = text.chars().count();
                        let sel = egui::TextEdit::load_state(ctx, id)
                            .and_then(|st| st.cursor.char_range())
                            .map(|r| (r.primary.index, r.secondary.index))
                            .unwrap_or((n, n));
                        let mut apply: Option<(String, String)> = None;
                        let mut strip = false;
                        let tag = |o: &str, c: &str| Some((o.to_string(), c.to_string()));
                        ui.horizontal_wrapped(|ui| {
                            if ui.button(egui::RichText::new("B").strong()).on_hover_text("Negrito <b>").clicked() {
                                apply = tag("<b>", "</b>");
                            }
                            if ui.button(egui::RichText::new("I").italics()).on_hover_text("Itálico <i>").clicked() {
                                apply = tag("<i>", "</i>");
                            }
                            if ui.button(egui::RichText::new("U").underline()).on_hover_text("Sublinhado <u>").clicked() {
                                apply = tag("<u>", "</u>");
                            }
                            if ui.button("A-").on_hover_text("Menor <small>").clicked() {
                                apply = tag("<small>", "</small>");
                            }
                            if ui.button("A+").on_hover_text("Maior <big>").clicked() {
                                apply = tag("<big>", "</big>");
                            }
                            egui::ComboBox::from_id_salt(("gs_html_size", i)).selected_text("Tamanho").width(70.0).show_ui(ui, |ui| {
                                for sz in [1, 2, 3, 4, 5, 7] {
                                    if ui.selectable_label(false, sz.to_string()).clicked() {
                                        apply = tag(&format!("<basefont size=\"{}\">", sz), "</basefont>");
                                    }
                                }
                            });
                            egui::ComboBox::from_id_salt(("gs_html_head", i)).selected_text("Título").width(60.0).show_ui(ui, |ui| {
                                for h in 1..=6 {
                                    if ui.selectable_label(false, format!("H{}", h)).clicked() {
                                        apply = tag(&format!("<h{}>", h), &format!("</h{}>", h));
                                    }
                                }
                            });
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.color_edit_button_srgb(&mut self.html_color);
                            if ui.button("Cor").on_hover_text("Aplica a cor à seleção <basefont color>").clicked() {
                                let c = self.html_color;
                                apply = tag(&format!("<basefont color=\"#{:02x}{:02x}{:02x}\">", c[0], c[1], c[2]), "</basefont>");
                            }
                            if ui.button("⬅").on_hover_text("Alinhar à esquerda").clicked() {
                                apply = tag("<div align=\"left\">", "</div>");
                            }
                            if ui.button("↔").on_hover_text("Centralizar <center>").clicked() {
                                apply = tag("<center>", "</center>");
                            }
                            if ui.button("➡").on_hover_text("Alinhar à direita").clicked() {
                                apply = tag("<div align=\"right\">", "</div>");
                            }
                            if ui.button("Link").on_hover_text("Link <a href>").clicked() {
                                apply = tag("<a href=\"\">", "</a>");
                            }
                            if ui.button("↵").on_hover_text("Quebra de linha <br>").clicked() {
                                apply = Some(("<br>".into(), String::new()));
                            }
                            if ui.button("Limpar").on_hover_text("Remover todas as tags").clicked() {
                                strip = true;
                            }
                        });
                        ui.add(egui::TextEdit::multiline(text).id(id).desired_rows(6).desired_width(f32::INFINITY));
                        if strip {
                            *text = strip_html_tags(text);
                        }
                        if let Some((o, c)) = apply {
                            let (a, b) = if c.is_empty() { (sel.0.max(sel.1), sel.0.max(sel.1)) } else { sel };
                            let (new, (s0, s1)) = wrap_selection(text, a, b, &o, &c);
                            *text = new;
                            if let Some(mut st) = egui::TextEdit::load_state(ctx, id) {
                                st.cursor.set_char_range(Some(egui::text::CCursorRange::two(egui::text::CCursor::new(s0), egui::text::CCursor::new(s1))));
                                st.store(ctx, id);
                            }
                        }
                    }
                    ui.checkbox(background, "fundo");
                    ui.checkbox(scrollbar, "barra de rolagem");
                }
                ElementKind::Button { normal, pressed, action, page, id } => {
                    Self::gump_field(ui, "Gump normal:", normal);
                    Self::gump_field(ui, "Gump pressionado:", pressed);
                    ui.horizontal(|ui| {
                        ui.label("Ação:");
                        ui.selectable_value(action, ButtonAction::Reply, "Resposta");
                        ui.selectable_value(action, ButtonAction::Page, "Mudar página");
                    });
                    match action {
                        ButtonAction::Page => Self::gump_field(ui, "Ir para a página:", page),
                        ButtonAction::Reply => {
                            let mut v = *id as u32;
                            Self::gump_field(ui, "Id (0 = automático):", &mut v);
                            *id = v as i32;
                        }
                    }
                    preview = Some((*normal, false));
                }
                ElementKind::Checkbox { off, on, checked, id } | ElementKind::Radio { off, on, checked, id } => {
                    Self::gump_field(ui, "Gump desmarcado:", off);
                    Self::gump_field(ui, "Gump marcado:", on);
                    ui.checkbox(checked, "marcado inicialmente");
                    let mut v = *id as u32;
                    Self::gump_field(ui, "Id (0 = automático):", &mut v);
                    *id = v as i32;
                    preview = Some((if *checked { *on } else { *off }, false));
                }
                ElementKind::TextEntry { hue, text, id } => {
                    Self::gump_field(ui, "Hue:", hue);
                    ui.horizontal(|ui| {
                        ui.label("Texto inicial:");
                        ui.text_edit_singleline(text);
                    });
                    let mut v = *id as u32;
                    Self::gump_field(ui, "Id (0 = automático):", &mut v);
                    *id = v as i32;
                }
                ElementKind::Alpha => {
                    ui.label("Região translúcida sobre o fundo.");
                }
            }
        }
        if self.doc.elements[i] != before {
            // one undo point per edit burst: store the state before the change
            let mut snapshot = self.doc.clone();
            snapshot.elements[i] = before;
            if self.undo.len() >= 60 {
                self.undo.remove(0);
            }
            if self.undo.last().map_or(true, |u| u.elements.get(i) != snapshot.elements.get(i)) {
                self.undo.push(snapshot);
            }
        }
        if let Some((id, resize)) = preview {
            ui.separator();
            ui.label("Pré-visualização:");
            let t = if resize { self.resize_tex(ctx, d, id, 96, 72) } else { self.gump_tex(ctx, d, id, 0) };
            match t {
                Some(t) => {
                    let sz = t.size_vec2();
                    let k = (200.0 / sz.x).min(120.0 / sz.y).min(1.0);
                    ui.image((t.id(), sz * k));
                }
                None => {
                    ui.weak("gump não encontrado");
                }
            }
        }
    }

    /// Browser of the resizepic frames found in the client gumps; clicking one sets it on the selected background.
    fn resizepic_window(&mut self, ctx: &egui::Context, d: &GumpStudioData) {
        const MAX_GUMP_ID: u32 = 0xFFFF;
        if let Some(gi) = d.gump_index {
            let start = std::time::Instant::now();
            let mut steps = 0u32;
            while self.resizepic_scan_pos + 8 <= MAX_GUMP_ID {
                let id = self.resizepic_scan_pos;
                let found = is_resizepic_at(
                    id,
                    |k| gi.gump_size(k as usize),
                    |k| {
                        let parts: Vec<GumpBitmap> = (0..9).filter_map(|p| decode_gump_id(gi, k + p)).collect();
                        frame_tiles_seamlessly(&parts)
                    },
                );
                if found {
                    self.resizepic_list.push(id);
                    self.resizepic_scan_pos += 9;
                } else {
                    self.resizepic_scan_pos += 1;
                }
                steps += 1;
                if steps >= 512 && start.elapsed() >= std::time::Duration::from_millis(8) {
                    break;
                }
            }
            if self.resizepic_scan_pos + 8 <= MAX_GUMP_ID {
                ctx.request_repaint();
            }
        } else {
            self.resizepic_scan_pos = MAX_GUMP_ID;
        }
        let scanning = self.resizepic_scan_pos + 8 <= MAX_GUMP_ID;
        let count = self.resizepic_list.len();
        let mut open = true;
        let mut pick: Option<u32> = None;
        egui::Window::new(t!("gs.resizepic.title")).open(&mut open).fixed_size([560.0, 420.0]).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(t!("gs.resizepic.search"));
                ui.text_edit_singleline(&mut self.resizepic_search);
            });
            if scanning {
                let frac = (self.resizepic_scan_pos as f32 / (MAX_GUMP_ID - 8) as f32).clamp(0.0, 1.0);
                let pct = (frac * 100.0).round() as u32;
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.add(egui::ProgressBar::new(frac).text(t!("gs.resizepic.loading", pct = pct, n = count)));
                });
            } else {
                ui.label(t!("gs.resizepic.count", n = count));
            }
            ui.label(t!("gs.resizepic.tip"));
            ui.separator();
            let q = self.resizepic_search.trim().to_string();
            let shown: Vec<u32> = self.resizepic_list.iter().copied().filter(|id| q.is_empty() || id.to_string().contains(&q)).collect();
            let cols = 5usize;
            let num_rows = (shown.len() + cols - 1) / cols;
            egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(ui, 96.0, num_rows, |ui, row_range| {
                egui::Grid::new("gs_resizepic_grid").spacing([8.0, 8.0]).show(ui, |ui| {
                    for row in row_range {
                        let start_idx = row * cols;
                        let end_idx = (start_idx + cols).min(shown.len());
                        for &id in &shown[start_idx..end_idx] {
                            ui.vertical(|ui| {
                                if let Some(t) = self.resize_tex(ctx, d, id, 96, 72) {
                                    if ui.add(egui::ImageButton::new((t.id(), t.size_vec2()))).clicked() {
                                        pick = Some(id);
                                    }
                                }
                                ui.small(id.to_string());
                            });
                        }
                        ui.end_row();
                    }
                });
            });
        });
        if !open {
            self.resizepic_open = false;
        }
        if let (Some(id), Some(i)) = (pick, self.selected) {
            if let Some(ElementKind::Background { gump_id }) = self.doc.elements.get(i).map(|e| e.kind.clone()) {
                if gump_id != id {
                    self.push_undo();
                    if let Some(ElementKind::Background { gump_id }) = self.doc.elements.get_mut(i).map(|e| &mut e.kind) {
                        *gump_id = id;
                    }
                }
            }
        }
    }

    /// Size in gump pixels of an element (for hit testing and selection boxes).
    fn natural_size(&mut self, ctx: &egui::Context, d: &GumpStudioData, e: &GumpElement) -> egui::Vec2 {
        let tex_size = |t: Option<egui::TextureHandle>| t.map_or(egui::vec2(24.0, 24.0), |t| t.size_vec2());
        match &e.kind {
            k if k.is_sized() => egui::vec2(e.w as f32, e.h as f32),
            ElementKind::Image { gump_id, hue } => tex_size(self.gump_tex(ctx, d, *gump_id, *hue)),
            ElementKind::Item { item_id, hue } => tex_size(self.item_tex(ctx, d, *item_id, *hue)),
            ElementKind::Button { normal, .. } => tex_size(self.gump_tex(ctx, d, *normal, 0)),
            ElementKind::Checkbox { off, .. } | ElementKind::Radio { off, .. } => tex_size(self.gump_tex(ctx, d, *off, 0)),
            ElementKind::Label { text, hue, font, ascii } => {
                let t = self.text_texture(ctx, d, text, *font, *ascii, *hue, None);
                t.map_or(egui::vec2((text.chars().count() as f32 * 7.0).max(14.0), 16.0), |t| t.size_vec2())
            }
            _ => egui::vec2(24.0, 24.0),
        }
    }

    fn canvas(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: &GumpStudioData) {
        let avail = ui.available_size();
        let (rect, bg) = ui.allocate_exact_size(avail, egui::Sense::click());
        let painter = ui.painter_at(rect);
        crate::draw_checkerboard(&painter, rect, 16.0);
        let origin = rect.min + egui::vec2(24.0, 24.0);
        let z = self.zoom;

        // bounding frame of the gump
        let (mut maxx, mut maxy) = (200.0f32, 120.0f32);
        let elems: Vec<GumpElement> = self.doc.elements.clone();
        let sizes: Vec<egui::Vec2> = elems.iter().map(|e| self.natural_size(ctx, d, e)).collect();
        for (e, s) in elems.iter().zip(&sizes) {
            maxx = maxx.max(e.x as f32 + s.x);
            maxy = maxy.max(e.y as f32 + s.y);
        }
        painter.rect_stroke(
            egui::Rect::from_min_size(origin, egui::vec2(maxx, maxy) * z),
            0.0,
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(80, 90, 120)),
        );

        let mut clicked_elem = false;
        for (i, e) in elems.iter().enumerate() {
            if e.page != 0 && e.page != self.view_page {
                continue;
            }
            let size = sizes[i];
            let pos = origin + egui::vec2(e.x as f32, e.y as f32) * z;
            let r = egui::Rect::from_min_size(pos, size * z);
            self.draw_element(&painter, ctx, d, e, r);
            if let Some(rep) = e.repeat.filter(|r| r.count > 1) {
                for k in 1..rep.count.min(200) as i32 {
                    let off = egui::vec2((rep.step_x * k) as f32, (rep.step_y * k) as f32) * z;
                    self.draw_element(&painter, ctx, d, e, r.translate(off));
                }
            }

            let resp = ui.interact(r, ui.id().with(("gs_el", i)), egui::Sense::click_and_drag());
            if resp.clicked() || resp.drag_started() {
                if ui.input(|inp| inp.modifiers.ctrl || inp.modifiers.command) {
                    // Ctrl+click toggles the element in the multiple selection
                    if let Some(pos) = self.multi.iter().position(|&m| m == i) {
                        self.multi.remove(pos);
                    } else {
                        self.multi.push(i);
                    }
                } else if !self.multi.contains(&i) {
                    self.multi.clear();
                }
                self.selected = Some(i);
                clicked_elem = true;
            }
            if resp.drag_started() {
                self.push_undo();
                self.drag_started = true;
            }
            if resp.dragged() {
                let delta = resp.drag_delta() / z;
                // the dragged element moves with its group or with the whole multiple selection
                let members: Vec<usize> = if self.multi.contains(&i) {
                    let mut m = self.multi.clone();
                    m.push(i);
                    m
                } else {
                    self.doc.group_members(i)
                };
                for k in members {
                    if let Some(el) = self.doc.elements.get_mut(k) {
                        el.x += delta.x.round() as i32;
                        el.y += delta.y.round() as i32;
                    }
                }
            }
            if resp.drag_stopped() && self.snap > 1 {
                if let Some(el) = self.doc.elements.get_mut(i) {
                    let s = self.snap;
                    el.x = (el.x as f32 / s as f32).round() as i32 * s;
                    el.y = (el.y as f32 / s as f32).round() as i32 * s;
                }
            }
            let in_group = self.selected.map_or(false, |sel| self.doc.group_members(sel).contains(&i));
            if self.selected == Some(i) {
                painter.rect_stroke(r.expand(1.0), 0.0, egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(255, 200, 60)));
            } else if self.multi.contains(&i) || in_group {
                painter.rect_stroke(r.expand(1.0), 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(120, 190, 255)));
            }
        }
        // resize handles of the selected element (right edge, bottom edge, corner)
        if let Some(i) = self.selected.filter(|&i| i < self.doc.elements.len()) {
            let e = self.doc.elements[i].clone();
            if e.kind.is_sized() && (e.page == 0 || e.page == self.view_page) {
                let pos = origin + egui::vec2(e.x as f32, e.y as f32) * z;
                let r = egui::Rect::from_min_size(pos, egui::vec2(e.w as f32, e.h as f32) * z);
                let hs = egui::vec2(9.0, 9.0);
                let handles = [
                    (egui::Rect::from_center_size(r.right_center(), hs), (1, 0), egui::CursorIcon::ResizeHorizontal),
                    (egui::Rect::from_center_size(r.center_bottom(), hs), (0, 1), egui::CursorIcon::ResizeVertical),
                    (egui::Rect::from_center_size(r.right_bottom(), hs), (1, 1), egui::CursorIcon::ResizeNwSe),
                ];
                for (k, (hr, dir, icon)) in handles.into_iter().enumerate() {
                    painter.rect_filled(hr, 1.0, egui::Color32::from_rgb(255, 200, 60));
                    let resp = ui.interact(hr, ui.id().with(("gs_handle", i, k)), egui::Sense::drag());
                    if resp.hovered() || resp.dragged() {
                        ui.ctx().set_cursor_icon(icon);
                    }
                    if resp.drag_started() {
                        self.push_undo();
                    }
                    if resp.dragged() {
                        let d = resp.drag_delta() / z;
                        if let Some(el) = self.doc.elements.get_mut(i) {
                            el.w = (el.w + (d.x * dir.0 as f32).round() as i32).clamp(8, 2000);
                            el.h = (el.h + (d.y * dir.1 as f32).round() as i32).clamp(8, 2000);
                        }
                    }
                    if resp.drag_stopped() && self.snap > 1 {
                        let sn = self.snap;
                        if let Some(el) = self.doc.elements.get_mut(i) {
                            el.w = ((el.w as f32 / sn as f32).round() as i32 * sn).max(8);
                            el.h = ((el.h as f32 / sn as f32).round() as i32 * sn).max(8);
                        }
                    }
                }
            }
        }
        if bg.clicked() && !clicked_elem {
            self.selected = None;
        }
    }

    fn draw_element(&mut self, p: &egui::Painter, ctx: &egui::Context, d: &GumpStudioData, e: &GumpElement, r: egui::Rect) {
        let full = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
        let img = |p: &egui::Painter, t: Option<egui::TextureHandle>, r: egui::Rect| {
            if let Some(t) = t {
                p.image(t.id(), r, full, egui::Color32::WHITE);
            } else {
                p.rect_stroke(r, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(200, 80, 80)));
            }
        };
        match &e.kind {
            ElementKind::Background { gump_id } => {
                let t = self.resize_tex(ctx, d, *gump_id, e.w, e.h);
                img(p, t, r);
            }
            ElementKind::Image { gump_id, hue } => {
                let t = self.gump_tex(ctx, d, *gump_id, *hue);
                img(p, t, r);
            }
            ElementKind::Tiled { gump_id } => {
                let t = self.tiled_tex(ctx, d, *gump_id, e.w, e.h);
                img(p, t, r);
            }
            ElementKind::Item { item_id, hue } => {
                let t = self.item_tex(ctx, d, *item_id, *hue);
                img(p, t, r);
            }
            ElementKind::Label { text, hue, font, ascii } => {
                let t = self.text_texture(ctx, d, text, *font, *ascii, *hue, None);
                if t.is_some() {
                    img(p, t, r);
                } else {
                    p.text(r.min, egui::Align2::LEFT_TOP, text, egui::FontId::proportional(14.0), Self::text_color(d, *hue));
                }
            }
            ElementKind::Html { text, cliloc, background, .. } => {
                if *background {
                    p.rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120));
                }
                let t = if *cliloc != 0 { self.cliloc_text(d, *cliloc) } else { text.clone() };
                match self.html_texture(ctx, d, &t, e.w.max(8) - 4) {
                    Some(tex) => {
                        let sz = tex.size_vec2() * (r.width() / (e.w.max(1) as f32));
                        p.with_clip_rect(r).image(tex.id(), egui::Rect::from_min_size(r.min + egui::vec2(2.0, 2.0), sz), full, egui::Color32::WHITE);
                    }
                    None => {
                        p.text(r.min + egui::vec2(2.0, 2.0), egui::Align2::LEFT_TOP, t, egui::FontId::proportional(13.0), egui::Color32::from_rgb(230, 230, 200));
                    }
                }
                p.rect_stroke(r, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(90, 90, 70)));
            }
            ElementKind::Button { normal, .. } => {
                let t = self.gump_tex(ctx, d, *normal, 0);
                img(p, t, r);
            }
            ElementKind::Checkbox { off, on, checked, .. } | ElementKind::Radio { off, on, checked, .. } => {
                let t = self.gump_tex(ctx, d, if *checked { *on } else { *off }, 0);
                img(p, t, r);
            }
            ElementKind::TextEntry { hue, text, .. } => {
                p.rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 90));
                p.rect_stroke(r, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(120, 120, 130)));
                match self.text_texture(ctx, d, text, 1, false, *hue, None) {
                    Some(tex) => {
                        let z = r.width() / (e.w.max(1) as f32);
                        p.with_clip_rect(r).image(tex.id(), egui::Rect::from_min_size(r.min + egui::vec2(3.0, 1.0), tex.size_vec2() * z), full, egui::Color32::WHITE);
                    }
                    None => {
                        p.text(r.min + egui::vec2(3.0, 2.0), egui::Align2::LEFT_TOP, text, egui::FontId::proportional(13.0), Self::text_color(d, *hue));
                    }
                }
            }
            ElementKind::Alpha => {
                p.rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 110));
                p.rect_stroke(r, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(110, 110, 160)));
            }
        }
    }
}
