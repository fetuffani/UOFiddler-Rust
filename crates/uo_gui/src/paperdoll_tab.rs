//! Paperdoll builder: dress a body with wearable items and see the animation and the paperdoll
//! gump side by side. Both views use the same equipment data (tiledata layer + animation id,
//! Equipconv.def / gump.def for the gump, hues.mul for colours).

use eframe::egui;
use std::collections::{BTreeMap, HashMap};
use std::time::{Duration, Instant};
use uo_core::*;

/// Actions of the people animation (classic numbering of anim.mul, as in ClassicUO's
/// `PeopleAnimationGroup`); the text is the locale key `paperdoll.act.<n>`.
const ACTIONS: [u32; 35] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34];

fn action_label(a: u32) -> String {
    crate::i18n::tr(&format!("paperdoll.act.{a}"))
}

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
    /// body ids that mobtypes.txt classifies as equipment
    pub equipment_bodies: &'a std::collections::HashSet<u32>,
    /// creature class of every animation body (0 monster, 1 sea, 2 animal, 3 human), for mounts
    pub anim_types: &'a HashMap<u32, u8>,
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
    /// no animation frames / no paperdoll gump for this item in the loaded files (set by `rebuild`);
    /// such an item is still worn, it just draws nothing
    no_anim: bool,
    no_gump: bool,
}

#[derive(Clone)]
struct CatalogItem {
    item_id: u16,
    anim_id: u16,
    name: String,
    /// lowercase name, for the search
    name_lc: String,
    /// layer the emulator scripts give the item, else the one the tiledata declares (`quality`); 0 = none
    layer: u8,
    /// known to be wearable: the tiledata flags it (with an animation and a layer) or an emulator lists it
    known: bool,
}

/// Small note for an item listed under a layer it was not declared for, "" when it fits.
fn layer_note(item: &CatalogItem, sel_layer: u8) -> String {
    if !item.known {
        t!("paperdoll.note.not_wearable")
    } else if item.layer != 0 && item.layer != sel_layer {
        t!("paperdoll.note.other_layer", layer = item.layer)
    } else {
        String::new()
    }
}

/// "no anim / no gump" for an equipped item, "" when it has both.
fn missing_note(e: &Equipped) -> String {
    let mut parts = Vec::new();
    if e.no_anim {
        parts.push(t!("paperdoll.row.no_anim"));
    }
    if e.no_gump {
        parts.push(t!("paperdoll.row.no_gump"));
    }
    parts.join(" / ")
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
    /// every item of the tiledata that has a name or an animation id (or that an emulator lists)
    all_items: Option<Vec<CatalogItem>>,
    /// list every item under every layer, not only those known for the selected one
    show_all: bool,
    /// keep only the items known to be wearable (tiledata flag or emulator list)
    known_only: bool,
    /// the filtered list shown under the layers (indices into `all_items`) and what it was built for
    list: Vec<usize>,
    list_key: Option<(String, bool, bool, u8)>,
    /// rows of the list drawn last frame (to scroll only when the arrows leave them) and the row to bring into view
    list_view: std::ops::Range<usize>,
    scroll_to: Option<usize>,
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
            all_items: None,
            show_all: true,
            known_only: false,
            list: Vec::new(),
            list_key: None,
            list_view: 0..0,
            scroll_to: None,
            doll_tex: None,
            anim_tex: Vec::new(),
            dirty: true,
            info: String::new(),
            scale: 1.0,
            slot_tex: HashMap::new(),
        }
    }
}

fn slot_label(layer: u8) -> String {
    match layer {
        1 => t!("paperdoll.slot.1"),
        2 => t!("paperdoll.slot.2"),
        3 => t!("paperdoll.slot.3"),
        4 => t!("paperdoll.slot.4"),
        5 => t!("paperdoll.slot.5"),
        6 => t!("paperdoll.slot.6"),
        7 => t!("paperdoll.slot.7"),
        8 => t!("paperdoll.slot.8"),
        9 => t!("paperdoll.slot.9"),
        10 => t!("paperdoll.slot.10"),
        11 => t!("paperdoll.slot.11"),
        12 => t!("paperdoll.slot.12"),
        13 => t!("paperdoll.slot.13"),
        14 => t!("paperdoll.slot.14"),
        16 => t!("paperdoll.slot.16"),
        17 => t!("paperdoll.slot.17"),
        18 => t!("paperdoll.slot.18"),
        19 => t!("paperdoll.slot.19"),
        20 => t!("paperdoll.slot.20"),
        21 => t!("paperdoll.slot.21"),
        22 => t!("paperdoll.slot.22"),
        24 => t!("paperdoll.slot.24"),
        25 => t!("paperdoll.slot.25"),
        _ => "?".to_string(),
    }
}

/// Localized long name of a layer (layer list, catalog title, empty-slot tooltip). The core
/// `LayerInfo::name` stays as the Portuguese fallback.
fn layer_label(layer: u8) -> String {
    match layer {
        1 => t!("paperdoll.layer.1"),
        2 => t!("paperdoll.layer.2"),
        3 => t!("paperdoll.layer.3"),
        4 => t!("paperdoll.layer.4"),
        5 => t!("paperdoll.layer.5"),
        6 => t!("paperdoll.layer.6"),
        7 => t!("paperdoll.layer.7"),
        8 => t!("paperdoll.layer.8"),
        9 => t!("paperdoll.layer.9"),
        10 => t!("paperdoll.layer.10"),
        11 => t!("paperdoll.layer.11"),
        12 => t!("paperdoll.layer.12"),
        13 => t!("paperdoll.layer.13"),
        14 => t!("paperdoll.layer.14"),
        16 => t!("paperdoll.layer.16"),
        17 => t!("paperdoll.layer.17"),
        18 => t!("paperdoll.layer.18"),
        19 => t!("paperdoll.layer.19"),
        20 => t!("paperdoll.layer.20"),
        21 => t!("paperdoll.layer.21"),
        22 => t!("paperdoll.layer.22"),
        24 => t!("paperdoll.layer.24"),
        25 => t!("paperdoll.layer.25"),
        _ => layer_info(layer).map_or_else(|| "?".to_string(), |l| l.name.to_string()),
    }
}

/// Localized label of a body kind (the core `label()` stays the Portuguese fallback).
fn body_label(b: BodyKind) -> String {
    match b {
        BodyKind::HumanMale => t!("paperdoll.body.human_m"),
        BodyKind::HumanFemale => t!("paperdoll.body.human_f"),
        BodyKind::ElfMale => t!("paperdoll.body.elf_m"),
        BodyKind::ElfFemale => t!("paperdoll.body.elf_f"),
        BodyKind::GargoyleMale => t!("paperdoll.body.gargoyle_m"),
        BodyKind::GargoyleFemale => t!("paperdoll.body.gargoyle_f"),
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

pub(crate) fn decode_gump_id(gi: &FileIndex, id: u32) -> Option<GumpBitmap> {
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
    /// Every item that has a name or an animation id, or that an emulator lists. The tiledata's
    /// wearable flag and layer are not always right (see issue #7), so the user may put any of them
    /// on any layer; `known` only says which ones are *believed* wearable.
    fn build_all_items(td: &TileDataManager) -> Vec<CatalogItem> {
        let emu = emulator_wearables();
        td.item_tiles
            .iter()
            .enumerate()
            .filter_map(|(id, t)| {
                let emu_layer = emu.get(&(id as u16));
                let mount = mount_info(id as u16).is_some(); // ClassicUO's mount table
                if t.anim_id == 0 && t.name.trim().is_empty() && emu_layer.is_none() && !mount {
                    return None;
                }
                let flagged = t.flags & WEARABLE != 0 && t.anim_id != 0 && layer_info(t.quality).is_some();
                let layer = if mount { MOUNT_LAYER } else { emu_layer.copied().flatten().unwrap_or(if flagged { t.quality } else { 0 }) };
                Some(CatalogItem { item_id: id as u16, anim_id: t.anim_id, name: t.name.clone(), name_lc: t.name.to_lowercase(), layer, known: flagged || emu_layer.is_some() || mount })
            })
            .collect()
    }

    fn equip(&mut self, td: &TileDataManager, layer: u8, item_id: u16) {
        if let Some(t) = td.item_tiles.get(item_id as usize) {
            self.equipped.insert(
                layer,
                Equipped {
                    item_id,
                    anim_id: t.anim_id,
                    hue: 0,
                    partial: t.flags & PARTIAL_HUE != 0,
                    name: t.name.clone(),
                    no_anim: false,
                    no_gump: false,
                },
            );
            self.dirty = true;
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
        for (l, e) in order {
            if e.anim_id == 0 || *l == MOUNT_LAYER {
                continue; // nothing to draw (the row says "no anim / no gump"); the mount is not on the paperdoll
            }
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

    /// The worn mount as (creature body, rider offset Y): the mount table first, the tiledata
    /// animation id for mount items that are not in it. (ClassicUO lets the tiledata win, but in the
    /// reference client the ids of the table are boat parts whose animation field is not a body.)
    /// Gargoyles do not ride.
    fn mount_target(&self) -> Option<(u32, i32)> {
        if matches!(self.body, BodyKind::GargoyleMale | BodyKind::GargoyleFemale) {
            return None;
        }
        let e = self.equipped.get(&MOUNT_LAYER)?;
        match mount_info(e.item_id) {
            Some(i) => Some((i.body, i.offset_y)),
            None if e.anim_id != 0 => Some((e.anim_id as u32, 0)),
            None => None,
        }
    }

    /// Frames of the worn mount for the selected action and direction, with the rider's offset Y
    /// and the people group the rider must use.
    fn mount_frames(&self, d: &PaperdollData) -> Option<(Vec<AnimationFrame>, i32, u32)> {
        let (body, offset) = self.mount_target()?;
        let rider = rider_group(self.action);
        let group = mount_group(rider, d.anim_types.get(&body).copied().unwrap_or(0));
        let mut frames = load_animation_frames(d.resolver, d.mul_anim, d.uop_anim, body, group, self.direction)?;
        if let Some(e) = self.equipped.get(&MOUNT_LAYER) {
            if let Some(p) = hue_palette(d.hues, e.hue) {
                for f in &mut frames {
                    apply_hue_bytes(&mut f.rgba_data, &p, false);
                }
            }
        }
        Some((frames, offset, rider))
    }

    fn compose_anim(&self, d: &PaperdollData) -> Vec<Vec<u8>> {
        // on a mount the rider uses the mounted groups; without usable mount frames he stands on the ground
        let mount = self.mount_frames(d);
        let action = mount.as_ref().map_or(self.action, |m| m.2);
        let load = |body: u32| load_animation_frames(d.resolver, d.mul_anim, d.uop_anim, body, action, self.direction);
        let Some(mut body_frames) = load(self.body.anim_body()) else { return Vec::new() };
        if let Some(p) = hue_palette(d.hues, self.skin_hue) {
            for f in &mut body_frames {
                apply_hue_bytes(&mut f.rgba_data, &p, false);
            }
        }
        let mut layers: Vec<(i32, Vec<AnimationFrame>)> = Vec::new();
        for (l, e) in &self.equipped {
            if e.anim_id == 0 || *l == MOUNT_LAYER {
                continue;
            }
            let id = d.equip_gump.convert_anim(self.body.anim_body(), e.anim_id as u32);
            let frames = load_equipment_frames(d.mul_anim, d.uop_anim, d.equipment_bodies.contains(&id), id, action, self.direction);
            if let Some(mut frames) = frames {
                if let Some(p) = hue_palette(d.hues, e.hue) {
                    for f in &mut frames {
                        apply_hue_bytes(&mut f.rgba_data, &p, e.partial);
                    }
                }
                layers.push((layer_info(*l).map_or(0, |i| i.anim_order), frames));
            }
        }
        layers.sort_by_key(|(o, _)| *o);

        let place = |canvas: &mut Vec<u8>, f: &AnimationFrame, dy: i32| {
            blit_alpha(
                canvas,
                CANVAS.0,
                CANVAS.1,
                &f.rgba_data,
                f.width as usize,
                f.height as usize,
                ANCHOR.0 - f.center_x as i32,
                ANCHOR.1 - f.center_y as i32 - f.height as i32 + dy,
            );
        };
        let rider_dy = mount.as_ref().map_or(0, |m| m.1);
        (0..body_frames.len())
            .map(|k| {
                let mut canvas = vec![0u8; CANVAS.0 * CANVAS.1 * 4];
                // the mount first (behind), then the rider and what he wears, moved down by the mount's offset
                if let Some((frames, _, _)) = &mount {
                    place(&mut canvas, &frames[k % frames.len()], 0);
                }
                place(&mut canvas, &body_frames[k], rider_dy);
                for (_, frames) in &layers {
                    place(&mut canvas, &frames[k % frames.len()], rider_dy);
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
        // Items whose animation or gump does not exist in the loaded files (new items on an old client,
        // wrong tiledata): they stay equipped, draw nothing and the layer row says so.
        let mount = self.mount_frames(d);
        let action = mount.as_ref().map_or(self.action, |m| m.2);
        let (anim_body, female, direction) = (self.body.anim_body(), self.body.female(), self.direction);
        for (layer, e) in self.equipped.iter_mut() {
            if *layer == MOUNT_LAYER {
                // drawn under the rider, never on the paperdoll gump
                e.no_anim = mount.is_none();
                e.no_gump = false;
                continue;
            }
            if e.anim_id == 0 {
                e.no_anim = true;
                e.no_gump = true;
                continue;
            }
            let id = d.equip_gump.convert_anim(anim_body, e.anim_id as u32);
            e.no_anim = load_equipment_frames(d.mul_anim, d.uop_anim, d.equipment_bodies.contains(&id), id, action, direction).is_none();
            let exists = |g: u32| d.gump_index.map_or(false, |gi| gi.is_valid(g as usize));
            let (g, _) = d.equip_gump.gump_for_anim(anim_body, e.anim_id as u32, female, exists);
            e.no_gump = !exists(g);
        }
        self.frame = 0;
        self.info = t!(
            "paperdoll.info",
            body = body_label(self.body),
            anim = self.body.anim_body(),
            action = self.action,
            dir = self.direction,
            frames = self.anim_tex.len(),
            items = self.equipped.len()
        );
        if let Some((_, _, rider)) = &mount {
            self.info.push_str(&format!(" {}", t!("paperdoll.info.mounted", group = rider)));
        }
        self.dirty = false;
    }

    // ---------------------------------------------------------------- UI

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, d: Option<&PaperdollData>) {
        let Some(d) = d else {
            ui.heading(t!("paperdoll.title"));
            ui.label(t!("paperdoll.need_data"));
            return;
        };
        if self.all_items.is_none() {
            self.all_items = Some(Self::build_all_items(d.tiledata));
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
            // UOFIDDLER_PAPERDOLL_ITEMS=0x2662,0x1539 equips exact items (automated screenshots)
            if let Ok(v) = std::env::var("UOFIDDLER_PAPERDOLL_ITEMS") {
                let ids: Vec<u16> = v.split(',').filter_map(|x| u16::from_str_radix(x.trim().trim_start_matches("0x"), 16).ok()).collect();
                self.preset(d.tiledata, &ids);
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
            ui.add(egui::Slider::new(&mut self.scale, 0.5..=2.0).text(t!("paperdoll.scale")));
            ui.separator();
            ui.columns(2, |cols| {
                cols[0].heading(t!("paperdoll.anim.heading"));
                if let Some(t) = self.anim_tex.get(self.frame) {
                    let avail = cols[0].available_size();
                    let k = (self.scale * ANIM_ZOOM).min(avail.y / CANVAS.1 as f32).min(avail.x / CANVAS.0 as f32).max(0.5);
                    let size = egui::vec2(CANVAS.0 as f32, CANVAS.1 as f32) * k;
                    cols[0].image((t.id(), size));
                } else {
                    cols[0].label(t!("paperdoll.anim.none"));
                }
                cols[1].heading(t!("paperdoll.doll.heading"));
                if let Some(t) = &self.doll_tex {
                    let size = t.size_vec2() * self.scale;
                    egui::ScrollArea::both().id_salt("pd_doll").show(&mut cols[1], |ui| {
                        ui.image((t.id(), size));
                    });
                } else {
                    cols[1].label(t!("paperdoll.doll.none"));
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
        ui.strong(t!("paperdoll.slots.heading"));
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
                        resp.on_hover_text(t!(
                            "paperdoll.slot.tip",
                            name = e.name,
                            item = format!("{:04X}", e.item_id),
                            anim = e.anim_id,
                            gump = g,
                            hue = e.hue
                        ))
                    }
                    None => resp.on_hover_text(t!("paperdoll.slot.empty", name = layer_label(info.id))),
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
        ui.heading(t!("paperdoll.title"));
        ui.horizontal_wrapped(|ui| {
            for b in BodyKind::ALL {
                if ui.selectable_label(self.body == b, body_label(b)).clicked() {
                    self.body = b;
                    self.dirty = true;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label(t!("paperdoll.skin"));
            if ui.add(egui::DragValue::new(&mut self.skin_hue).range(0..=2999)).changed() {
                self.dirty = true;
            }
            ui.label(t!("paperdoll.dir"));
            if ui.add(egui::Slider::new(&mut self.direction, 0..=7)).changed() {
                self.dirty = true;
            }
        });
        ui.horizontal(|ui| {
            egui::ComboBox::from_label(t!("paperdoll.action")).selected_text(format!("{} {}", self.action, action_label(self.action))).show_ui(ui, |ui| {
                for a in ACTIONS {
                    if ui.selectable_value(&mut self.action, a, format!("{} {}", a, action_label(a))).clicked() {
                        self.dirty = true;
                    }
                }
            });
            ui.checkbox(&mut self.playing, "▶");
        });
        ui.horizontal_wrapped(|ui| {
            if ui.button(t!("paperdoll.preset.nude")).clicked() {
                self.equipped.clear();
                self.dirty = true;
            }
            if ui.button(t!("paperdoll.preset.common")).clicked() {
                self.preset(d.tiledata, &[0x1517, 0x1539, 0x170B, 0x203B]);
            }
            if ui.button(t!("paperdoll.preset.plate")).clicked() {
                self.preset(d.tiledata, &[0x1412, 0x1413, 0x1410, 0x1414, 0x1415, 0x1411, 0x13B9, 0x1B72]);
            }
            if ui.button(t!("paperdoll.preset.robe")).clicked() {
                self.preset(d.tiledata, &[0x1F03, 0x1515, 0x170B, 0x203C]);
            }
        });
        ui.separator();
        ui.strong(t!("paperdoll.layers"));
        egui::ScrollArea::vertical().id_salt("pd_layers").max_height(240.0).show(ui, |ui| {
            for info in LAYERS.iter() {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.sel_layer == info.id, format!("{:>2} {}", info.id, layer_label(info.id))).clicked() {
                        self.sel_layer = info.id;
                    }
                    let mut remove = false;
                    if let Some(e) = self.equipped.get_mut(&info.id) {
                        let mut note = missing_note(e);
                        if info.id == MOUNT_LAYER && matches!(self.body, BodyKind::GargoyleMale | BodyKind::GargoyleFemale) {
                            note = t!("paperdoll.row.no_ride");
                        }
                        let text = format!("0x{:04X} {}", e.item_id, e.name);
                        if note.is_empty() {
                            ui.label(text);
                        } else {
                            let red = egui::Color32::from_rgb(235, 90, 90);
                            ui.colored_label(red, text);
                            ui.label(egui::RichText::new(format!("({note})")).small().color(red));
                        }
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
        let name = layer_label(self.sel_layer);
        ui.strong(t!("paperdoll.layer_items", name = name));
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search);
        });
        ui.checkbox(&mut self.show_all, t!("paperdoll.show_all")).on_hover_text(t!("paperdoll.show_all.tip"));
        ui.checkbox(&mut self.known_only, t!("paperdoll.known_only")).on_hover_text(t!("paperdoll.known_only.tip"));

        // arrows: left / right change the layer, up / down walk the list and wear the item (not while typing)
        let (up, down, left, right) = if ui.ctx().wants_keyboard_input() {
            (false, false, false, false)
        } else {
            ui.ctx().input(|i| (i.key_pressed(egui::Key::ArrowUp), i.key_pressed(egui::Key::ArrowDown), i.key_pressed(egui::Key::ArrowLeft), i.key_pressed(egui::Key::ArrowRight)))
        };
        if left || right {
            let at = LAYERS.iter().position(|l| l.id == self.sel_layer).unwrap_or(0);
            let to = if left { at.saturating_sub(1) } else { (at + 1).min(LAYERS.len() - 1) };
            self.sel_layer = LAYERS[to].id;
        }

        // the filtered list is rebuilt only when something it depends on changes
        let key = (self.search.to_lowercase(), self.show_all, self.known_only, self.sel_layer);
        if self.list_key.as_ref() != Some(&key) {
            let (q, show_all, known_only, layer) = (&key.0, key.1, key.2, key.3);
            self.list = self
                .all_items
                .as_ref()
                .map(|v| {
                    v.iter()
                        .enumerate()
                        .filter(|(_, i)| (show_all || (i.known && i.layer == layer)) && (!known_only || i.known))
                        .filter(|(_, i)| q.is_empty() || i.name_lc.contains(q.as_str()) || format!("{:04x}", i.item_id).contains(q.as_str()))
                        .map(|(n, _)| n)
                        .collect()
                })
                .unwrap_or_default();
            self.list_key = Some(key);
        }
        ui.label(t!("paperdoll.items_count", n = self.list.len()));
        let mut pick = None;
        if (up || down) && !self.list.is_empty() {
            if let Some(all) = &self.all_items {
                let worn = self.equipped.get(&self.sel_layer).map(|e| e.item_id);
                let at = worn.and_then(|id| self.list.iter().position(|&n| all[n].item_id == id));
                let to = match (at, up) {
                    (None, true) => self.list.len() - 1,
                    (None, false) => 0,
                    (Some(a), true) => a.saturating_sub(1),
                    (Some(a), false) => (a + 1).min(self.list.len() - 1),
                };
                pick = Some(all[self.list[to]].item_id);
                self.scroll_to = Some(to);
            }
        }
        let mut shown = self.list_view.clone();
        if let Some(all) = &self.all_items {
            let list = &self.list;
            let step = 20.0 + ui.spacing().item_spacing.y;
            let mut area = egui::ScrollArea::vertical().id_salt("pd_items").auto_shrink([false, false]);
            if let Some(n) = self.scroll_to.take() {
                // move only when the row is outside the rows on screen, keeping a row of context
                let rows = shown.len().max(3);
                if n < shown.start {
                    area = area.vertical_scroll_offset(n.saturating_sub(1) as f32 * step);
                } else if n + 1 >= shown.end {
                    area = area.vertical_scroll_offset((n + 2).saturating_sub(rows) as f32 * step);
                }
            }
            area.show_rows(ui, 20.0, list.len(), |ui, range| {
                shown = range.clone();
                for n in range {
                    let it = &all[list[n]];
                    let sel = self.equipped.get(&self.sel_layer).map_or(false, |e| e.item_id == it.item_id);
                    let mut text = t!("paperdoll.item_row", item = format!("{:04X}", it.item_id), name = it.name, anim = it.anim_id);
                    let note = layer_note(it, self.sel_layer);
                    if !note.is_empty() {
                        text.push_str(&format!("  · {note}"));
                    }
                    if ui.selectable_label(sel, text).clicked() {
                        pick = Some(it.item_id);
                    }
                }
            });
        }
        self.list_view = shown;
        if let Some(id) = pick {
            let layer = self.sel_layer;
            self.equip(d.tiledata, layer, id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(layer: u8, known: bool) -> CatalogItem {
        CatalogItem { item_id: 0x2662, anim_id: 734, name: "shirt".into(), name_lc: "shirt".into(), layer, known }
    }

    fn worn(no_anim: bool, no_gump: bool) -> Equipped {
        Equipped { item_id: 0x2662, anim_id: 734, hue: 0, partial: false, name: "shirt".into(), no_anim, no_gump }
    }

    #[test]
    fn items_under_another_layer_or_not_wearable_get_a_note() {
        assert_eq!(layer_note(&item(5, true), 5), "");
        assert!(layer_note(&item(5, true), 3).contains('5'));
        assert!(!layer_note(&item(5, false), 5).is_empty());
        // a known wearable whose layer nobody names (armor, weapons) is not flagged
        assert_eq!(layer_note(&item(0, true), 5), "");
    }

    #[test]
    fn missing_note_names_what_is_missing() {
        assert_eq!(missing_note(&worn(false, false)), "");
        assert_eq!(missing_note(&worn(true, false)), "no anim");
        assert_eq!(missing_note(&worn(false, true)), "no gump");
        assert_eq!(missing_note(&worn(true, true)), "no anim / no gump");
    }
}
