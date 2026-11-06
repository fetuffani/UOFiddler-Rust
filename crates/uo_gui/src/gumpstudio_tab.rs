//! GumpStudio tab: visual gump designer (after https://github.com/mbnunes/GumpStudio) with a
//! live preview drawn from the real client art, and exporters for RunUO, Sphere and POL.

use eframe::egui;
use std::collections::HashMap;
use uo_core::*;

use crate::paperdoll_tab::decode_gump_id;

pub struct GumpStudioData<'a> {
    pub gump_index: Option<&'a FileIndex>,
    pub art: Option<&'a FileIndex>,
    pub hues: Option<&'a HuesManager>,
    pub cliloc: Option<&'a ClilocManager>,
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
    drag_started: bool,
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
            drag_started: false,
        };
        t.doc = Self::sample_doc();
        t
    }
}

impl GumpStudioTab {
    fn sample_doc() -> GumpDoc {
        let mut d = GumpDoc::default();
        d.name = "ExampleGump".into();
        let mk = |name: &str, kind, x, y, w, h| GumpElement { name: name.into(), x, y, w, h, page: 0, kind };
        d.elements = vec![
            mk("background", ElementKind::Background { gump_id: 9200 }, 0, 0, 300, 180),
            mk("title", ElementKind::Label { text: "Example gump".into(), hue: 52 }, 20, 15, 0, 0),
            mk("name_entry", ElementKind::TextEntry { hue: 0, text: "type here".into(), id: 0 }, 20, 50, 200, 20),
            mk("opt", ElementKind::Checkbox { off: 210, on: 211, checked: true, id: 0 }, 20, 85, 0, 0),
            mk("opt_text", ElementKind::Label { text: "Enable".into(), hue: 0 }, 50, 87, 0, 0),
            mk("ok", ElementKind::Button { normal: 247, pressed: 248, action: ButtonAction::Reply, page: 0, id: 0 }, 20, 130, 0, 0),
            mk("ok_text", ElementKind::Label { text: "OK".into(), hue: 0 }, 55, 132, 0, 0),
        ];
        d
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

    fn cliloc_text(&mut self, d: &GumpStudioData, n: u32) -> String {
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
                let c = p[28];
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
        egui::SidePanel::right("gs_right").resizable(true).default_width(290.0).show_inside(ui, |ui| self.properties(ui, ctx, d));
        egui::CentralPanel::default().show_inside(ui, |ui| self.canvas(ui, ctx, d));
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
                });
                self.selected = Some(n);
            }
        }
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
            ui.separator();
            match &mut e.kind {
                ElementKind::Background { gump_id } => {
                    Self::gump_field(ui, "Gump (resizepic):", gump_id);
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
                ElementKind::Label { text, hue } => {
                    ui.label("Texto:");
                    ui.text_edit_multiline(text);
                    Self::gump_field(ui, "Hue:", hue);
                }
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    Self::gump_field(ui, "Cliloc (0 = texto):", cliloc);
                    if *cliloc == 0 {
                        ui.label("HTML:");
                        ui.text_edit_multiline(text);
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

    /// Size in gump pixels of an element (for hit testing and selection boxes).
    fn natural_size(&mut self, ctx: &egui::Context, d: &GumpStudioData, e: &GumpElement) -> egui::Vec2 {
        let tex_size = |t: Option<egui::TextureHandle>| t.map_or(egui::vec2(24.0, 24.0), |t| t.size_vec2());
        match &e.kind {
            k if k.is_sized() => egui::vec2(e.w as f32, e.h as f32),
            ElementKind::Image { gump_id, hue } => tex_size(self.gump_tex(ctx, d, *gump_id, *hue)),
            ElementKind::Item { item_id, hue } => tex_size(self.item_tex(ctx, d, *item_id, *hue)),
            ElementKind::Button { normal, .. } => tex_size(self.gump_tex(ctx, d, *normal, 0)),
            ElementKind::Checkbox { off, .. } | ElementKind::Radio { off, .. } => tex_size(self.gump_tex(ctx, d, *off, 0)),
            ElementKind::Label { text, .. } => egui::vec2((text.chars().count() as f32 * 7.0).max(14.0), 16.0),
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

            let resp = ui.interact(r, ui.id().with(("gs_el", i)), egui::Sense::click_and_drag());
            if resp.clicked() || resp.drag_started() {
                self.selected = Some(i);
                clicked_elem = true;
            }
            if resp.drag_started() {
                self.push_undo();
                self.drag_started = true;
            }
            if resp.dragged() {
                let delta = resp.drag_delta() / z;
                let snap = self.snap.max(1) as f32;
                if let Some(el) = self.doc.elements.get_mut(i) {
                    // accumulate fractional movement in the sub-pixel part of the position
                    el.x += delta.x.round() as i32;
                    el.y += delta.y.round() as i32;
                    if snap > 1.0 && resp.drag_stopped() {
                        el.x = (el.x as f32 / snap).round() as i32 * snap as i32;
                        el.y = (el.y as f32 / snap).round() as i32 * snap as i32;
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
            if self.selected == Some(i) {
                painter.rect_stroke(r.expand(1.0), 0.0, egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(255, 200, 60)));
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
            ElementKind::Label { text, hue } => {
                p.text(r.min, egui::Align2::LEFT_TOP, text, egui::FontId::proportional(14.0 * (r.height() / 16.0).max(0.5)), Self::text_color(d, *hue));
            }
            ElementKind::Html { text, cliloc, background, .. } => {
                if *background {
                    p.rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120));
                }
                let t = if *cliloc != 0 { self.cliloc_text(d, *cliloc) } else { text.clone() };
                p.text(r.min + egui::vec2(2.0, 2.0), egui::Align2::LEFT_TOP, t, egui::FontId::proportional(13.0), egui::Color32::from_rgb(230, 230, 200));
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
                p.text(r.min + egui::vec2(3.0, 2.0), egui::Align2::LEFT_TOP, text, egui::FontId::proportional(13.0), Self::text_color(d, *hue));
            }
            ElementKind::Alpha => {
                p.rect_filled(r, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 110));
                p.rect_stroke(r, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(110, 110, 160)));
            }
        }
    }
}
