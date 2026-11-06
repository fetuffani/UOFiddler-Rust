//! Paperdoll builder: dress a body with wearable items and see the animation and the paperdoll
//! gump side by side. Both views use the same equipment data (tiledata layer + animation id,
//! Equipconv.def / gump.def for the gump, hues.mul for colours).

use eframe::egui;
use std::collections::{BTreeMap, HashMap};
use std::time::{Duration, Instant};
use uo_core::*;

use crate::ACTIONS_HUMAN;

const WEARABLE: u64 = 1 << 22;
const PARTIAL_HUE: u64 = 1 << 18;
const CANVAS: (usize, usize) = (200, 200);
const ANCHOR: (i32, i32) = (100, 165);
/// the animation sprites are small; show them magnified
const ANIM_ZOOM: f32 = 2.5;

/// Read-only game data the builder needs.
pub struct PaperdollData<'a> {
    pub tiledata: &'a TileDataManager,
    pub equip_gump: &'a EquipGumpMap,
    pub gump_index: Option<&'a FileIndex>,
    pub hues: Option<&'a HuesManager>,
    pub resolver: &'a ClassicAnimResolver,
    pub mul_anim: &'a HashMap<u32, FileIndex>,
    pub uop_anim: &'a UopAnimationIndex,
    /// art.mul index (item art for the slot view)
    pub art: Option<&'a FileIndex>,
}

#[derive(Clone)]
struct Equipped {
    item_id: u16,
    anim_id: u16,
    hue: u32,
    partial: bool,
    name: String,
}

#[derive(Clone)]
struct CatalogItem {
    item_id: u16,
    anim_id: u16,
    name: String,
}

pub struct PaperdollTab {
    body: BodyKind,
    skin_hue: u32,
    equipped: BTreeMap<u8, Equipped>,
    sel_layer: u8,
    search: String,
    action: u32,
    direction: u8,
    playing: bool,
    frame: usize,
    last_tick: Instant,
    catalog: Option<HashMap<u8, Vec<CatalogItem>>>,
    doll_tex: Option<egui::TextureHandle>,
    anim_tex: Vec<egui::TextureHandle>,
    dirty: bool,
    info: String,
    scale: f32,
    slot_tex: HashMap<(u16, u32, bool), Option<egui::TextureHandle>>,
}

impl Default for PaperdollTab {
    fn default() -> Self {
        Self {
            body: BodyKind::HumanMale,
            skin_hue: 1002,
            equipped: BTreeMap::new(),
            sel_layer: 5,
            search: String::new(),
            action: 0,
            direction: 3,
            playing: true,
            frame: 0,
            last_tick: Instant::now(),
            catalog: None,
            doll_tex: None,
            anim_tex: Vec::new(),
            dirty: true,
            info: String::new(),
            scale: 1.0,
            slot_tex: HashMap::new(),
        }
    }
}

fn slot_label(layer: u8) -> &'static str {
    match layer {
        1 => "Dir.",
        2 => "Esq.",
        3 => "Pés",
        4 => "Calça",
        5 => "Camisa",
        6 => "Elmo",
        7 => "Luvas",
        8 => "Anel",
        9 => "Talismã",
        10 => "Colar",
        11 => "Cabelo",
        12 => "Cinto",
        13 => "Peito",
        14 => "Pulso",
        16 => "Barba",
        17 => "Tronco",
        18 => "Brinco",
        19 => "Braços",
        20 => "Capa",
        21 => "Mochila",
        22 => "Robe",
        24 => "Pernas",
        _ => "?",
    }
}

fn hue_palette(hues: Option<&HuesManager>, hue: u32) -> Option<[u32; 32]> {
    if hue == 0 {
        return None;
    }
    hues?.hues.get(hue as usize - 1).map(|h| h.get_rgba_palette())
}

fn blit_alpha(dst: &mut [u8], dw: usize, dh: usize, src: &[u8], sw: usize, sh: usize, x0: i32, y0: i32) {
    for y in 0..sh as i32 {
        let dy = y0 + y;
        if dy < 0 || dy >= dh as i32 {
            continue;
        }
        for x in 0..sw as i32 {
            let dx = x0 + x;
            if dx < 0 || dx >= dw as i32 {
                continue;
            }
            let s = (y as usize * sw + x as usize) * 4;
            if src[s + 3] == 0 {
                continue;
            }
            let d = (dy as usize * dw + dx as usize) * 4;
            dst[d..d + 4].copy_from_slice(&src[s..s + 4]);
        }
    }
}

fn decode_gump_id(gi: &FileIndex, id: u32) -> Option<GumpBitmap> {
    let id = id as usize;
    let data = gi.get_data(id)?;
    let entry = gi.entries[id];
    let (w, h, payload) = if gi.is_uop() || entry.compression == 1 || entry.compression == 3 {
        if data.len() < 8 {
            return None;
        }
        (
            u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
            data[8..].to_vec(),
        )
    } else {
        (((entry.extra >> 16) & 0xFFFF) as u32, (entry.extra & 0xFFFF) as u32, data)
    };
    decode_gump(&payload, w, h)
}

impl PaperdollTab {
    fn build_catalog(td: &TileDataManager) -> HashMap<u8, Vec<CatalogItem>> {
        let mut m: HashMap<u8, Vec<CatalogItem>> = HashMap::new();
        for (id, t) in td.item_tiles.iter().enumerate() {
            if t.flags & WEARABLE != 0 && t.anim_id != 0 && layer_info(t.quality).is_some() {
                m.entry(t.quality).or_default().push(CatalogItem {
                    item_id: id as u16,
                    anim_id: t.anim_id,
                    name: t.name.clone(),
                });
            }
        }
        m
    }

    fn equip(&mut self, td: &TileDataManager, layer: u8, item_id: u16) {
        if let Some(t) = td.item_tiles.get(item_id as usize) {
            if t.anim_id != 0 {
                self.equipped.insert(
                    layer,
                    Equipped {
                        item_id,
                        anim_id: t.anim_id,
                        hue: 0,
                        partial: t.flags & PARTIAL_HUE != 0,
                        name: t.name.clone(),
                    },
                );
                self.dirty = true;
            }
        }
    }

    fn preset(&mut self, td: &TileDataManager, items: &[u16]) {
        self.equipped.clear();
        for &i in items {
            if let Some(t) = td.item_tiles.get(i as usize) {
                if layer_info(t.quality).is_some() {
                    self.equip(td, t.quality, i);
                }
            }
        }
    }

    // ---------------------------------------------------------------- composition

    fn compose_doll(&self, d: &PaperdollData) -> Option<(usize, usize, Vec<u8>)> {
        let gi = d.gump_index?;
        let mut body = decode_gump_id(gi, self.body.body_gump())?;
        if let Some(p) = hue_palette(d.hues, self.skin_hue) {
            apply_hue_bytes(&mut body.rgba, &p, false);
        }
        let (w, h) = (body.width as usize, body.height as usize);
        let mut canvas = body.rgba.clone();

        let mut order: Vec<(&u8, &Equipped)> = self.equipped.iter().collect();
        order.sort_by_key(|(l, _)| layer_info(**l).map_or(0, |i| i.doll_order));
        let exists = |g: u32| gi.is_valid(g as usize);
        for (_, e) in order {
            let (gump, ehue) = d.equip_gump.gump_for_anim(self.body.anim_body(), e.anim_id as u32, self.body.female(), exists);
            let Some(mut bmp) = decode_gump_id(gi, gump) else { continue };
            let hue = if e.hue != 0 { e.hue } else { ehue.max(0) as u32 };
            if let Some(p) = hue_palette(d.hues, hue) {
                apply_hue_bytes(&mut bmp.rgba, &p, e.partial);
            }
            blit_alpha(&mut canvas, w, h, &bmp.rgba, bmp.width as usize, bmp.height as usize, 0, 0);
        }
        Some((w, h, canvas))
    }

    fn compose_anim(&self, d: &PaperdollData) -> Vec<Vec<u8>> {
        let load = |body: u32| load_animation_frames(d.resolver, d.mul_anim, d.uop_anim, body, self.action, self.direction);
        let Some(mut body_frames) = load(self.body.anim_body()) else { return Vec::new() };
        if let Some(p) = hue_palette(d.hues, self.skin_hue) {
            for f in &mut body_frames {
                apply_hue_bytes(&mut f.rgba_data, &p, false);
            }
        }
        let mut layers: Vec<(i32, Vec<AnimationFrame>)> = Vec::new();
        for (l, e) in &self.equipped {
            if let Some(mut frames) = load(e.anim_id as u32) {
                if let Some(p) = hue_palette(d.hues, e.hue) {
                    for f in &mut frames {
                        apply_hue_bytes(&mut f.rgba_data, &p, e.partial);
                    }
                }
                layers.push((layer_info(*l).map_or(0, |i| i.anim_order), frames));
            }
        }
        layers.sort_by_key(|(o, _)| *o);

        let place = |canvas: &mut Vec<u8>, f: &AnimationFrame| {
            blit_alpha(
                canvas,
                CANVAS.0,
                CANVAS.1,
                &f.rgba_data,
                f.width as usize,
                f.height as usize,
                ANCHOR.0 - f.center_x as i32,
                ANCHOR.1 - f.center_y as i32 - f.height as i32,
            );
        };
        (0..body_frames.len())
            .map(|k| {
                let mut canvas = vec![0u8; CANVAS.0 * CANVAS.1 * 4];
                place(&mut canvas, &body_frames[k]);
                for (_, frames) in &layers {
                    place(&mut canvas, &frames[k % frames.len()]);
                }
                canvas
            })
            .collect()
    }

    fn rebuild(&mut self, ctx: &egui::Context, d: &PaperdollData) {
        self.doll_tex = self.compose_doll(d).map(|(w, h, rgba)| {
            ctx.load_texture("paperdoll_doll", egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba), egui::TextureOptions::NEAREST)
        });
        let frames = self.compose_anim(d);
        self.anim_tex = frames
            .iter()
            .enumerate()
            .map(|(i, rgba)| {
                ctx.load_texture(
                    format!("paperdoll_anim_{i}"),
                    egui::ColorImage::from_rgba_unmultiplied([CANVAS.0, CANVAS.1], rgba),
                    egui::TextureOptions::NEAREST,
                )
            })
            .collect();
        self.frame = 0;
        self.info = format!(
            "{} · corpo {} · ação {} · direção {} · {} quadros · {} itens",
            self.body.label(),
            self.body.anim_body(),
            self.action,
            self.direction,
            self.anim_tex.len(),
            self.equipped.len()
        );
        self.dirty = false;
    }

    // ---------------------------------------------------------------- UI

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: Option<&PaperdollData>) {
        let Some(d) = d else {
            ui.heading("🧍 Paperdoll Builder");
            ui.label("Carregue tiledata.mul e os arquivos de gump/animação para usar o construtor.");
            return;
        };
        if self.catalog.is_none() {
            self.catalog = Some(Self::build_catalog(d.tiledata));
            // UOFIDDLER_PAPERDOLL=<plate|common|robe>[,<body kind index>] (automated screenshots)
            if let Ok(v) = std::env::var("UOFIDDLER_PAPERDOLL") {
                let mut p = v.split(',');
                match p.next() {
                    Some("plate") => self.preset(d.tiledata, &[0x1412, 0x1413, 0x1410, 0x1414, 0x1415, 0x1411, 0x13B9, 0x1B72]),
                    Some("robe") => self.preset(d.tiledata, &[0x1F03, 0x1515, 0x170B, 0x203C]),
                    _ => self.preset(d.tiledata, &[0x1517, 0x1539, 0x170B, 0x203B]),
                }
                if let Some(k) = p.next().and_then(|x| x.parse::<usize>().ok()) {
                    self.body = BodyKind::ALL[k.min(5)];
                }
            }
        }
        if self.dirty {
            self.rebuild(ctx, d);
        }
        if self.playing && self.anim_tex.len() > 1 {
            if self.last_tick.elapsed() >= Duration::from_millis(110) {
                self.frame = (self.frame + 1) % self.anim_tex.len();
                self.last_tick = Instant::now();
            }
            ctx.request_repaint_after(Duration::from_millis(60));
        }

        egui::SidePanel::left("paperdoll_left").resizable(true).default_width(380.0).show_inside(ui, |ui| {
            self.show_controls(ui, d);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::TopBottomPanel::bottom("paperdoll_slots").resizable(false).show_inside(ui, |ui| {
                self.show_slots(ui, ctx, d);
            });
            ui.label(&self.info);
            ui.add(egui::Slider::new(&mut self.scale, 0.5..=2.0).text("escala"));
            ui.separator();
            ui.columns(2, |cols| {
                cols[0].heading("🐉 Animação");
                if let Some(t) = self.anim_tex.get(self.frame) {
                    let avail = cols[0].available_size();
                    let k = (self.scale * ANIM_ZOOM).min(avail.y / CANVAS.1 as f32).min(avail.x / CANVAS.0 as f32).max(0.5);
                    let size = egui::vec2(CANVAS.0 as f32, CANVAS.1 as f32) * k;
                    cols[0].image((t.id(), size));
                } else {
                    cols[0].label("Sem animação para esta combinação de corpo/ação/direção.");
                }
                cols[1].heading("🖼 Paperdoll (gump)");
                if let Some(t) = &self.doll_tex {
                    let size = t.size_vec2() * self.scale;
                    egui::ScrollArea::both().id_salt("pd_doll").show(&mut cols[1], |ui| {
                        ui.image((t.id(), size));
                    });
                } else {
                    cols[1].label("Gump do corpo não encontrado.");
                }
            });
        });
    }

    /// Item texture for the slot view: the item's art, hued like it would be in a backpack.
    fn slot_texture(&mut self, ctx: &egui::Context, d: &PaperdollData, item_id: u16, hue: u32, partial: bool) -> Option<egui::TextureHandle> {
        let key = (item_id, hue, partial);
        if let Some(t) = self.slot_tex.get(&key) {
            return t.clone();
        }
        let tex = d.art.and_then(|a| a.get_data(0x4000 + item_id as usize)).and_then(|raw| decode_static(&raw)).map(|mut bmp| {
            if let Some(p) = hue_palette(d.hues, hue) {
                apply_hue_bytes(&mut bmp.rgba, &p, partial);
            }
            ctx.load_texture(
                format!("slot_{item_id}_{hue}"),
                egui::ColorImage::from_rgba_unmultiplied([bmp.width as usize, bmp.height as usize], &bmp.rgba),
                egui::TextureOptions::NEAREST,
            )
        });
        self.slot_tex.insert(key, tex.clone());
        tex
    }

    /// One slot per wearable layer, like the cells of a backpack.
    fn show_slots(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: &PaperdollData) {
        ui.add_space(4.0);
        ui.strong("🎒 Slots (itens equipados)");
        const SLOT: f32 = 62.0;
        let mut select = None;
        let mut remove = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
            for info in LAYERS.iter() {
                let eq = self.equipped.get(&info.id).cloned();
                let tex = eq.as_ref().and_then(|e| self.slot_texture(ctx, d, e.item_id, e.hue, e.partial));
                let (rect, resp) = ui.allocate_exact_size(egui::vec2(SLOT, SLOT + 14.0), egui::Sense::click());
                let cell = egui::Rect::from_min_size(rect.min, egui::vec2(SLOT, SLOT));
                let selected = self.sel_layer == info.id;
                let p = ui.painter();
                p.rect_filled(cell, 4.0, egui::Color32::from_rgb(30, 28, 24));
                p.rect_stroke(
                    cell,
                    4.0,
                    egui::Stroke::new(
                        if selected { 2.0_f32 } else { 1.0_f32 },
                        if selected { egui::Color32::from_rgb(120, 140, 255) } else { egui::Color32::from_rgb(70, 64, 54) },
                    ),
                );
                if let Some(t) = &tex {
                    let sz = t.size_vec2();
                    let k = (SLOT - 6.0) / sz.x.max(sz.y);
                    let k = if k < 1.0 { k } else { 1.0 };
                    p.image(
                        t.id(),
                        egui::Rect::from_center_size(cell.center(), sz * k),
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                let label = slot_label(info.id);
                p.text(
                    egui::pos2(rect.center().x, rect.max.y - 1.0),
                    egui::Align2::CENTER_BOTTOM,
                    label,
                    egui::FontId::proportional(10.0),
                    egui::Color32::from_rgb(170, 170, 180),
                );
                let resp = match &eq {
                    Some(e) => {
                        let (g, _) = d.equip_gump.gump_for_anim(self.body.anim_body(), e.anim_id as u32, self.body.female(), |g| {
                            d.gump_index.map_or(false, |gi| gi.is_valid(g as usize))
                        });
                        resp.on_hover_text(format!(
                            "{}\nItem 0x{:04X} · animação {} · gump {}\nHue {}   (botão direito remove)",
                            e.name, e.item_id, e.anim_id, g, e.hue
                        ))
                    }
                    None => resp.on_hover_text(format!("{} (vazio)", info.name)),
                };
                if resp.clicked() {
                    select = Some(info.id);
                }
                if resp.secondary_clicked() && eq.is_some() {
                    remove = Some(info.id);
                }
            }
        });
        if let Some(l) = select {
            self.sel_layer = l;
        }
        if let Some(l) = remove {
            self.equipped.remove(&l);
            self.dirty = true;
        }
        ui.add_space(4.0);
    }

    fn show_controls(&mut self, ui: &mut egui::Ui, d: &PaperdollData) {
        ui.heading("🧍 Paperdoll Builder");
        ui.horizontal_wrapped(|ui| {
            for b in BodyKind::ALL {
                if ui.selectable_label(self.body == b, b.label()).clicked() {
                    self.body = b;
                    self.dirty = true;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Pele (hue):");
            if ui.add(egui::DragValue::new(&mut self.skin_hue).range(0..=2999)).changed() {
                self.dirty = true;
            }
            ui.label("Dir:");
            if ui.add(egui::Slider::new(&mut self.direction, 0..=7)).changed() {
                self.dirty = true;
            }
        });
        ui.horizontal(|ui| {
            let cur = ACTIONS_HUMAN.iter().find(|(a, _)| *a == self.action).map_or("?", |(_, n)| *n);
            egui::ComboBox::from_label("Ação").selected_text(format!("{} {}", self.action, cur)).show_ui(ui, |ui| {
                for (a, n) in ACTIONS_HUMAN.iter() {
                    if ui.selectable_value(&mut self.action, *a, format!("{} {}", a, n)).clicked() {
                        self.dirty = true;
                    }
                }
            });
            ui.checkbox(&mut self.playing, "▶");
        });
        ui.horizontal_wrapped(|ui| {
            if ui.button("Nu").clicked() {
                self.equipped.clear();
                self.dirty = true;
            }
            if ui.button("Roupa comum").clicked() {
                self.preset(d.tiledata, &[0x1517, 0x1539, 0x170B, 0x203B]);
            }
            if ui.button("Armadura de placa").clicked() {
                self.preset(d.tiledata, &[0x1412, 0x1413, 0x1410, 0x1414, 0x1415, 0x1411, 0x13B9, 0x1B72]);
            }
            if ui.button("Robe + capa").clicked() {
                self.preset(d.tiledata, &[0x1F03, 0x1515, 0x170B, 0x203C]);
            }
        });
        ui.separator();
        ui.strong("Layers");
        egui::ScrollArea::vertical().id_salt("pd_layers").max_height(240.0).show(ui, |ui| {
            for info in LAYERS.iter() {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.sel_layer == info.id, format!("{:>2} {}", info.id, info.name)).clicked() {
                        self.sel_layer = info.id;
                    }
                    let mut remove = false;
                    if let Some(e) = self.equipped.get_mut(&info.id) {
                        ui.label(format!("0x{:04X} {}", e.item_id, e.name));
                        if ui.add(egui::DragValue::new(&mut e.hue).range(0..=2999).prefix("hue ")).changed() {
                            self.dirty = true;
                        }
                        if ui.small_button("✖").clicked() {
                            remove = true;
                        }
                    }
                    if remove {
                        self.equipped.remove(&info.id);
                        self.dirty = true;
                    }
                });
            }
        });
        ui.separator();
        let name = layer_info(self.sel_layer).map_or("", |l| l.name);
        ui.strong(format!("Itens da layer: {}", name));
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search);
        });
        let q = self.search.to_lowercase();
        let items: Vec<CatalogItem> = self
            .catalog
            .as_ref()
            .and_then(|c| c.get(&self.sel_layer))
            .map(|v| {
                v.iter()
                    .filter(|i| q.is_empty() || i.name.to_lowercase().contains(&q) || format!("{:04x}", i.item_id).contains(&q))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        ui.label(format!("{} itens", items.len()));
        let mut pick = None;
        egui::ScrollArea::vertical().id_salt("pd_items").auto_shrink([false, false]).show_rows(ui, 20.0, items.len(), |ui, range| {
            for i in range {
                let it = &items[i];
                let sel = self.equipped.get(&self.sel_layer).map_or(false, |e| e.item_id == it.item_id);
                if ui.selectable_label(sel, format!("0x{:04X}  {}  (anim {})", it.item_id, it.name, it.anim_id)).clicked() {
                    pick = Some(it.item_id);
                }
            }
        });
        if let Some(id) = pick {
            let layer = self.sel_layer;
            self.equip(d.tiledata, layer, id);
        }
    }
}
