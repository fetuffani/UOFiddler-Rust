//! GumpStudio model: a visual gump (dialog) made of elements, saved as JSON and exported to
//! RunUO (C#), SphereScript dialogs and POL (gumps.inc) scripts.
//!
//! The element set follows https://github.com/mbnunes/GumpStudio (background, image, tiled image,
//! item, label, HTML / localized HTML, button, checkbox, radio, text entry, alpha region).

use serde::{Deserialize, Serialize};

use crate::gumps::GumpBitmap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonAction {
    /// Closes the gump and replies to the server with the button id.
    Reply,
    /// Switches to another page of the same gump.
    Page,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ElementKind {
    /// Stretchable frame (resizepic): gump ids `gump_id .. gump_id + 8`.
    Background { gump_id: u32 },
    Image { gump_id: u32, hue: u32 },
    Tiled { gump_id: u32 },
    Item { item_id: u32, hue: u32 },
    Label { text: String, hue: u32 },
    Html { text: String, cliloc: u32, background: bool, scrollbar: bool },
    Button { normal: u32, pressed: u32, action: ButtonAction, page: u32, id: i32 },
    Checkbox { off: u32, on: u32, checked: bool, id: i32 },
    Radio { off: u32, on: u32, checked: bool, id: i32 },
    TextEntry { hue: u32, text: String, id: i32 },
    Alpha,
}

impl ElementKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Background { .. } => "Fundo (resizepic)",
            Self::Image { .. } => "Imagem",
            Self::Tiled { .. } => "Imagem repetida",
            Self::Item { .. } => "Item",
            Self::Label { .. } => "Texto",
            Self::Html { .. } => "HTML",
            Self::Button { .. } => "Botão",
            Self::Checkbox { .. } => "Checkbox",
            Self::Radio { .. } => "Radio",
            Self::TextEntry { .. } => "Entrada de texto",
            Self::Alpha => "Região alfa",
        }
    }
    /// True for elements whose size is chosen by the user (the others take the art size).
    pub fn is_sized(&self) -> bool {
        matches!(self, Self::Background { .. } | Self::Tiled { .. } | Self::Html { .. } | Self::TextEntry { .. } | Self::Alpha)
    }
    pub fn default_for(name: &str) -> Option<(Self, i32, i32)> {
        Some(match name {
            "background" => (Self::Background { gump_id: 9200 }, 200, 150),
            "image" => (Self::Image { gump_id: 10, hue: 0 }, 0, 0),
            "tiled" => (Self::Tiled { gump_id: 2624 }, 100, 60),
            "item" => (Self::Item { item_id: 0x0EED, hue: 0 }, 0, 0),
            "label" => (Self::Label { text: "Texto".into(), hue: 0 }, 0, 0),
            "html" => (Self::Html { text: "HTML".into(), cliloc: 0, background: false, scrollbar: false }, 150, 60),
            "button" => (Self::Button { normal: 247, pressed: 248, action: ButtonAction::Reply, page: 0, id: 0 }, 0, 0),
            "checkbox" => (Self::Checkbox { off: 210, on: 211, checked: false, id: 0 }, 0, 0),
            "radio" => (Self::Radio { off: 209, on: 208, checked: false, id: 0 }, 0, 0),
            "entry" => (Self::TextEntry { hue: 0, text: String::new(), id: 0 }, 150, 20),
            "alpha" => (Self::Alpha, 100, 60),
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GumpElement {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    /// 0 = always visible; N > 0 = only on page N
    pub page: u32,
    pub kind: ElementKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GumpDoc {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub closable: bool,
    pub movable: bool,
    pub disposable: bool,
    pub resizable: bool,
    pub elements: Vec<GumpElement>,
}

impl Default for GumpDoc {
    fn default() -> Self {
        Self { name: "CustomGump".into(), x: 100, y: 100, closable: true, movable: true, disposable: true, resizable: false, elements: Vec::new() }
    }
}

impl GumpDoc {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }

    /// Distinct page numbers in use (always includes 0), ascending.
    pub fn pages(&self) -> Vec<u32> {
        let mut p: Vec<u32> = self.elements.iter().map(|e| e.page).collect();
        p.push(0);
        p.sort_unstable();
        p.dedup();
        p
    }

    /// Identifier safe to use as a class / dialog / function name.
    pub fn ident(&self) -> String {
        let s: String = self.name.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
        if s.is_empty() || s.chars().next().unwrap().is_ascii_digit() {
            format!("Gump{}", s)
        } else {
            s
        }
    }

    /// Final ids for buttons / switches / text entries: explicit ids (> 0) are kept, the others
    /// are numbered from 1 skipping the ids already taken.
    fn resolved_ids(&self) -> Vec<i32> {
        let used = |f: &dyn Fn(&ElementKind) -> Option<i32>| -> Vec<i32> {
            self.elements.iter().filter_map(|e| f(&e.kind)).filter(|&i| i > 0).collect()
        };
        let btn_used = used(&|k| if let ElementKind::Button { id, action: ButtonAction::Reply, .. } = k { Some(*id) } else { None });
        let sw_used = used(&|k| match k {
            ElementKind::Checkbox { id, .. } | ElementKind::Radio { id, .. } => Some(*id),
            _ => None,
        });
        let te_used = used(&|k| if let ElementKind::TextEntry { id, .. } = k { Some(*id) } else { None });
        let (mut nb, mut ns, mut nt) = (0, 0, 0);
        let mut next = |used: &Vec<i32>, n: &mut i32| -> i32 {
            loop {
                *n += 1;
                if !used.contains(n) {
                    return *n;
                }
            }
        };
        self.elements
            .iter()
            .map(|e| match &e.kind {
                ElementKind::Button { id, action: ButtonAction::Reply, .. } => if *id > 0 { *id } else { next(&btn_used, &mut nb) },
                ElementKind::Checkbox { id, .. } | ElementKind::Radio { id, .. } => if *id > 0 { *id } else { next(&sw_used, &mut ns) },
                ElementKind::TextEntry { id, .. } => if *id > 0 { *id } else { next(&te_used, &mut nt) },
                _ => 0,
            })
            .collect()
    }

    /// Elements in export order: page 0 first, then each page ascending (z-order kept inside a page).
    fn by_page(&self) -> Vec<(u32, Vec<usize>)> {
        self.pages()
            .into_iter()
            .map(|p| (p, self.elements.iter().enumerate().filter(|(_, e)| e.page == p).map(|(i, _)| i).collect()))
            .collect()
    }
}

fn cs_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\r', "").replace('\n', "\\n")
}
fn b(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// RunUO / ServUO C# gump class.
pub fn export_runuo(doc: &GumpDoc) -> String {
    let ids = doc.resolved_ids();
    let class = doc.ident();
    let mut s = String::new();
    s.push_str("using System;\nusing Server;\nusing Server.Gumps;\nusing Server.Network;\n\nnamespace Server.Gumps\n{\n");
    s.push_str(&format!("\tpublic class {} : Gump\n\t{{\n", class));
    s.push_str(&format!("\t\tpublic {}() : base( {}, {} )\n\t\t{{\n", class, doc.x, doc.y));
    s.push_str(&format!("\t\t\tClosable = {};\n\t\t\tDisposable = {};\n\t\t\tDragable = {};\n\t\t\tResizable = {};\n\n", b(doc.closable), b(doc.disposable), b(doc.movable), b(doc.resizable)));
    let mut reply_ids: Vec<(i32, String)> = Vec::new();
    for (page, idxs) in doc.by_page() {
        if page == 0 && idxs.is_empty() {
            s.push_str("\t\t\tAddPage( 0 );\n");
            continue;
        }
        s.push_str(&format!("\t\t\tAddPage( {} );\n", page));
        for i in idxs {
            let e = &doc.elements[i];
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("AddBackground( {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, gump_id),
                ElementKind::Image { gump_id, hue } => {
                    if *hue != 0 { format!("AddImage( {}, {}, {}, {} );", e.x, e.y, gump_id, hue) } else { format!("AddImage( {}, {}, {} );", e.x, e.y, gump_id) }
                }
                ElementKind::Tiled { gump_id } => format!("AddImageTiled( {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => {
                    if *hue != 0 { format!("AddItem( {}, {}, 0x{:X}, {} );", e.x, e.y, item_id, hue) } else { format!("AddItem( {}, {}, 0x{:X} );", e.x, e.y, item_id) }
                }
                ElementKind::Label { text, hue } => format!("AddLabel( {}, {}, {}, \"{}\" );", e.x, e.y, hue, cs_escape(text)),
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    if *cliloc != 0 {
                        format!("AddHtmlLocalized( {}, {}, {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, cliloc, b(*background), b(*scrollbar))
                    } else {
                        format!("AddHtml( {}, {}, {}, {}, \"{}\", {}, {} );", e.x, e.y, e.w, e.h, cs_escape(text), b(*background), b(*scrollbar))
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => {
                        reply_ids.push((ids[i], e.name.clone()));
                        format!("AddButton( {}, {}, {}, {}, {}, GumpButtonType.Reply, 0 );", e.x, e.y, normal, pressed, ids[i])
                    }
                    ButtonAction::Page => format!("AddButton( {}, {}, {}, {}, 0, GumpButtonType.Page, {} );", e.x, e.y, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("AddCheck( {}, {}, {}, {}, {}, {} );", e.x, e.y, off, on, b(*checked), ids[i]),
                ElementKind::Radio { off, on, checked, .. } => format!("AddRadio( {}, {}, {}, {}, {}, {} );", e.x, e.y, off, on, b(*checked), ids[i]),
                ElementKind::TextEntry { hue, text, .. } => format!("AddTextEntry( {}, {}, {}, {}, {}, {}, \"{}\" );", e.x, e.y, e.w, e.h, hue, ids[i], cs_escape(text)),
                ElementKind::Alpha => format!("AddAlphaRegion( {}, {}, {}, {} );", e.x, e.y, e.w, e.h),
            };
            s.push_str(&format!("\t\t\t{}\n", line));
        }
    }
    s.push_str("\t\t}\n\n");
    s.push_str("\t\tpublic override void OnResponse( NetState sender, RelayInfo info )\n\t\t{\n\t\t\tswitch ( info.ButtonID )\n\t\t\t{\n");
    s.push_str("\t\t\t\tcase 0: // closed or right-clicked\n\t\t\t\t{\n\t\t\t\t\tbreak;\n\t\t\t\t}\n");
    for (id, name) in reply_ids {
        s.push_str(&format!("\t\t\t\tcase {}: // {}\n\t\t\t\t{{\n\t\t\t\t\tbreak;\n\t\t\t\t}}\n", id, name));
    }
    s.push_str("\t\t\t}\n\t\t}\n\t}\n}\n");
    s
}

/// SphereServer dialog script ([DIALOG], [DIALOG TEXT], [DIALOG BUTTON]).
pub fn export_sphere(doc: &GumpDoc) -> String {
    let ids = doc.resolved_ids();
    let name = format!("d_{}", doc.ident().to_lowercase());
    let mut texts: Vec<String> = Vec::new();
    let mut tidx = |t: &str| -> usize {
        texts.push(t.replace('\n', " "));
        texts.len() - 1
    };
    let mut s = format!("[DIALOG {}]\n{},{}\n", name, doc.x, doc.y);
    if !doc.movable {
        s.push_str("nomove\n");
    }
    if !doc.closable {
        s.push_str("noclose\n");
    }
    if !doc.disposable {
        s.push_str("nodispose\n");
    }
    let mut reply: Vec<(i32, String)> = Vec::new();
    for (page, idxs) in doc.by_page() {
        s.push_str(&format!("page {}\n", page));
        for i in idxs {
            let e = &doc.elements[i];
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("resizepic {} {} {} {} {}", e.x, e.y, gump_id, e.w, e.h),
                ElementKind::Image { gump_id, hue } => {
                    if *hue != 0 { format!("gumppic {} {} {} hue={}", e.x, e.y, gump_id, hue) } else { format!("gumppic {} {} {}", e.x, e.y, gump_id) }
                }
                ElementKind::Tiled { gump_id } => format!("gumppictiled {} {} {} {} {}", e.x, e.y, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => {
                    if *hue != 0 { format!("tilepichue {} {} {} {}", e.x, e.y, item_id, hue) } else { format!("tilepic {} {} {}", e.x, e.y, item_id) }
                }
                ElementKind::Label { text, hue } => format!("text {} {} {} {}", e.x, e.y, hue, tidx(text)),
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    if *cliloc != 0 {
                        format!("xmfhtmlgump {} {} {} {} {} {} {}", e.x, e.y, e.w, e.h, cliloc, *background as u8, *scrollbar as u8)
                    } else {
                        format!("htmlgump {} {} {} {} {} {} {}", e.x, e.y, e.w, e.h, tidx(text), *background as u8, *scrollbar as u8)
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => {
                        reply.push((ids[i], e.name.clone()));
                        format!("button {} {} {} {} 1 0 {}", e.x, e.y, normal, pressed, ids[i])
                    }
                    ButtonAction::Page => format!("button {} {} {} {} 0 {} 0", e.x, e.y, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("checkbox {} {} {} {} {} {}", e.x, e.y, off, on, *checked as u8, ids[i]),
                ElementKind::Radio { off, on, checked, .. } => format!("radio {} {} {} {} {} {}", e.x, e.y, off, on, *checked as u8, ids[i]),
                ElementKind::TextEntry { hue, text, .. } => format!("textentry {} {} {} {} {} {} {}", e.x, e.y, e.w, e.h, hue, ids[i], tidx(text)),
                ElementKind::Alpha => format!("checkertrans {} {} {} {}", e.x, e.y, e.w, e.h),
            };
            s.push_str(&line);
            s.push('\n');
        }
    }
    s.push_str(&format!("\n[DIALOG {} TEXT]\n", name));
    for t in &texts {
        s.push_str(t);
        s.push('\n');
    }
    s.push_str(&format!("\n[DIALOG {} BUTTON]\nON=0\n// closed\n", name));
    for (id, n) in reply {
        s.push_str(&format!("\nON={}\n// {}\n", id, n));
    }
    s
}

/// POL script using the standard `gumps.inc` helpers (GFCreateGump, GFResizePic, ...).
pub fn export_pol(doc: &GumpDoc) -> String {
    let ids = doc.resolved_ids();
    let q = |t: &str| format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n"));
    let func = doc.ident();
    let mut s = String::from("use uo;\ninclude \":gumps:gumps\"; // gumps.inc (older cores: include \"include/gumps\";)\n\n");
    s.push_str(&format!("program {}( who )\n", func));
    s.push_str(&format!("\tvar gump := GFCreateGump( {}, {} );\n", doc.x, doc.y));
    if !doc.movable || !doc.closable || !doc.disposable {
        s.push_str(&format!("\t// movable={} closable={} disposable={} (set through the gump flags of your include)\n", doc.movable, doc.closable, doc.disposable));
    }
    for (page, idxs) in doc.by_page() {
        s.push_str(&format!("\tGFPage( gump, {} );\n", page));
        for i in idxs {
            let e = &doc.elements[i];
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("GFResizePic( gump, {}, {}, {}, {}, {} );", e.x, e.y, gump_id, e.w, e.h),
                ElementKind::Image { gump_id, .. } => format!("GFGumpPic( gump, {}, {}, {} );", e.x, e.y, gump_id),
                ElementKind::Tiled { gump_id } => format!("GFGumpPicTiled( gump, {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => format!("GFTilePic( gump, {}, {}, 0x{:X}, {} );", e.x, e.y, item_id, hue),
                ElementKind::Label { text, hue } => format!("GFTextLine( gump, {}, {}, {}, {} );", e.x, e.y, hue, q(text)),
                ElementKind::Html { text, background, scrollbar, cliloc } => {
                    if *cliloc != 0 {
                        format!("GFHTMLArea( gump, {}, {}, {}, {}, {}, {}, {} ); // cliloc {} is not supported by gumps.inc: use a plain text", e.x, e.y, e.w, e.h, q(""), *background as u8, *scrollbar as u8, cliloc)
                    } else {
                        format!("GFHTMLArea( gump, {}, {}, {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, q(text), *background as u8, *scrollbar as u8)
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => format!("GFAddButton( gump, {}, {}, {}, {}, GF_CLOSE_BTN, {} );", e.x, e.y, normal, pressed, ids[i]),
                    ButtonAction::Page => format!("GFAddButton( gump, {}, {}, {}, {}, GF_PAGE_BTN, {} );", e.x, e.y, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("GFCheckBox( gump, {}, {}, {}, {}, {}, {} );", e.x, e.y, off, on, *checked as u8, ids[i]),
                ElementKind::Radio { off, on, checked, .. } => format!("GFRadioButton( gump, {}, {}, {}, {}, {}, {} );", e.x, e.y, off, on, *checked as u8, ids[i]),
                ElementKind::TextEntry { hue, text, .. } => format!("GFTextEntry( gump, {}, {}, {}, {}, {}, {}, {} );", e.x, e.y, e.w, e.h, hue, q(text), ids[i]),
                ElementKind::Alpha => format!("GFCheckerTrans( gump, {}, {}, {}, {} );", e.x, e.y, e.w, e.h),
            };
            s.push_str(&format!("\t{}\n", line));
        }
    }
    s.push_str("\n\tvar input := GFSendGump( who, gump );\n\tif ( !input || input[0] == 0 )\n\t\treturn; // closed\n\tendif\n\n\tcase ( input[0] )\n");
    for (i, e) in doc.elements.iter().enumerate() {
        if let ElementKind::Button { action: ButtonAction::Reply, .. } = e.kind {
            s.push_str(&format!("\t\t{}: // {}\n\t\t\tbreak;\n", ids[i], e.name));
        }
    }
    s.push_str("\tendcase\nendprogram\n");
    s
}

/// Tiles `src` over a `w` x `h` area starting at (x0, y0) of `dst` (RGBA).
fn tile_into(dst: &mut [u8], dw: usize, dh: usize, src: &GumpBitmap, x0: i32, y0: i32, w: i32, h: i32) {
    if src.width == 0 || src.height == 0 || w <= 0 || h <= 0 {
        return;
    }
    for y in 0..h {
        let dy = y0 + y;
        if dy < 0 || dy as usize >= dh {
            continue;
        }
        let sy = (y as u32 % src.height) as usize;
        for x in 0..w {
            let dx = x0 + x;
            if dx < 0 || dx as usize >= dw {
                continue;
            }
            let sx = (x as u32 % src.width) as usize;
            let s = (sy * src.width as usize + sx) * 4;
            if src.rgba[s + 3] == 0 {
                continue;
            }
            let d = (dy as usize * dw + dx as usize) * 4;
            dst[d..d + 4].copy_from_slice(&src.rgba[s..s + 4]);
        }
    }
}

/// Builds a `w` x `h` image out of the 9 pieces of a resizepic (`gump_id .. gump_id + 8`):
/// 0 TL, 1 top, 2 TR, 3 left, 4 centre, 5 right, 6 BL, 7 bottom, 8 BR.
pub fn compose_resizepic(parts: &[GumpBitmap], w: i32, h: i32) -> Option<GumpBitmap> {
    if parts.len() < 9 || w <= 0 || h <= 0 || w > 4096 || h > 4096 {
        return None;
    }
    let (wu, hu) = (w as usize, h as usize);
    let mut out = vec![0u8; wu * hu * 4];
    let p = |i: usize| &parts[i];
    let (lw, rw) = (p(0).width.max(p(3).width).max(p(6).width) as i32, p(2).width.max(p(5).width).max(p(8).width) as i32);
    let (th, bh) = (p(0).height.max(p(1).height).max(p(2).height) as i32, p(6).height.max(p(7).height).max(p(8).height) as i32);
    let (cw, ch) = ((w - lw - rw).max(0), (h - th - bh).max(0));
    // centre first, then edges, then corners on top
    tile_into(&mut out, wu, hu, p(4), lw, th, cw, ch);
    tile_into(&mut out, wu, hu, p(1), lw, 0, cw, th);
    tile_into(&mut out, wu, hu, p(7), lw, h - bh, cw, bh);
    tile_into(&mut out, wu, hu, p(3), 0, th, lw, ch);
    tile_into(&mut out, wu, hu, p(5), w - rw, th, rw, ch);
    for (i, x, y) in [(0, 0, 0), (2, w - rw, 0), (6, 0, h - bh), (8, w - rw, h - bh)] {
        tile_into(&mut out, wu, hu, p(i), x, y, p(i).width as i32, p(i).height as i32);
    }
    Some(GumpBitmap { width: w as u32, height: h as u32, rgba: out })
}

/// Repeats a gump over a `w` x `h` area (gumppictiled).
pub fn compose_tiled(src: &GumpBitmap, w: i32, h: i32) -> Option<GumpBitmap> {
    if w <= 0 || h <= 0 || w > 4096 || h > 4096 {
        return None;
    }
    let mut out = vec![0u8; (w * h * 4) as usize];
    tile_into(&mut out, w as usize, h as usize, src, 0, 0, w, h);
    Some(GumpBitmap { width: w as u32, height: h as u32, rgba: out })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn el(kind: ElementKind, x: i32, y: i32, w: i32, h: i32, page: u32) -> GumpElement {
        GumpElement { name: "el".into(), x, y, w, h, page, kind }
    }

    fn sample() -> GumpDoc {
        let mut d = GumpDoc::default();
        d.name = "My Gump".into();
        d.elements = vec![
            el(ElementKind::Background { gump_id: 9200 }, 0, 0, 300, 200, 0),
            el(ElementKind::Label { text: "He said \"hi\"".into(), hue: 52 }, 20, 20, 0, 0, 0),
            el(ElementKind::Button { normal: 247, pressed: 248, action: ButtonAction::Reply, page: 0, id: 0 }, 20, 160, 0, 0, 0),
            el(ElementKind::Button { normal: 4005, pressed: 4007, action: ButtonAction::Page, page: 2, id: 0 }, 60, 160, 0, 0, 1),
            el(ElementKind::Checkbox { off: 210, on: 211, checked: true, id: 0 }, 20, 60, 0, 0, 1),
            el(ElementKind::TextEntry { hue: 0, text: "abc".into(), id: 0 }, 20, 90, 150, 20, 2),
            el(ElementKind::Item { item_id: 0xEED, hue: 0 }, 200, 20, 0, 0, 0),
        ];
        d
    }

    #[test]
    fn json_roundtrip() {
        let d = sample();
        assert_eq!(GumpDoc::from_json(&d.to_json()).unwrap(), d);
    }

    #[test]
    fn runuo_export() {
        let c = export_runuo(&sample());
        assert!(c.contains("public class MyGump : Gump"));
        assert!(c.contains("AddBackground( 0, 0, 300, 200, 9200 );"));
        assert!(c.contains("AddLabel( 20, 20, 52, \"He said \\\"hi\\\"\" );"));
        assert!(c.contains("AddButton( 20, 160, 247, 248, 1, GumpButtonType.Reply, 0 );"));
        assert!(c.contains("AddButton( 60, 160, 4005, 4007, 0, GumpButtonType.Page, 2 );"));
        assert!(c.contains("AddCheck( 20, 60, 210, 211, true, 1 );"));
        assert!(c.contains("AddTextEntry( 20, 90, 150, 20, 0, 1, \"abc\" );"));
        assert!(c.contains("AddItem( 200, 20, 0xEED );"));
        assert!(c.contains("AddPage( 1 );") && c.contains("AddPage( 2 );"));
        assert!(c.contains("case 1:"));
    }

    #[test]
    fn sphere_export() {
        let c = export_sphere(&sample());
        assert!(c.starts_with("[DIALOG d_mygump]\n100,100\n"));
        assert!(c.contains("resizepic 0 0 9200 300 200"));
        assert!(c.contains("text 20 20 52 0"));
        assert!(c.contains("button 20 160 247 248 1 0 1"));
        assert!(c.contains("button 60 160 4005 4007 0 2 0"));
        assert!(c.contains("textentry 20 90 150 20 0 1 1"));
        assert!(c.contains("[DIALOG d_mygump TEXT]\nHe said \"hi\"\nabc\n"));
        assert!(c.contains("[DIALOG d_mygump BUTTON]\nON=0"));
    }

    #[test]
    fn pol_export() {
        let c = export_pol(&sample());
        assert!(c.contains("program MyGump( who )"));
        assert!(c.contains("var gump := GFCreateGump( 100, 100 );"));
        assert!(c.contains("GFResizePic( gump, 0, 0, 9200, 300, 200 );"));
        assert!(c.contains("GFAddButton( gump, 20, 160, 247, 248, GF_CLOSE_BTN, 1 );"));
        assert!(c.contains("GFAddButton( gump, 60, 160, 4005, 4007, GF_PAGE_BTN, 2 );"));
        assert!(c.contains("GFTextEntry( gump, 20, 90, 150, 20, 0, \"abc\", 1 );"));
    }

    #[test]
    fn explicit_ids_are_kept_and_not_reused() {
        let mut d = GumpDoc::default();
        d.elements = vec![
            el(ElementKind::Button { normal: 1, pressed: 2, action: ButtonAction::Reply, page: 0, id: 1 }, 0, 0, 0, 0, 0),
            el(ElementKind::Button { normal: 1, pressed: 2, action: ButtonAction::Reply, page: 0, id: 0 }, 0, 0, 0, 0, 0),
            el(ElementKind::Button { normal: 1, pressed: 2, action: ButtonAction::Reply, page: 0, id: 7 }, 0, 0, 0, 0, 0),
        ];
        assert_eq!(d.resolved_ids(), vec![1, 2, 7]);
    }

    #[test]
    fn resizepic_composes_to_requested_size() {
        let mk = |w: u32, h: u32, v: u8| GumpBitmap { width: w, height: h, rgba: vec![v; (w * h * 4) as usize] };
        let parts: Vec<GumpBitmap> = (0..9).map(|i| mk(4, 4, 10 + i as u8)).collect();
        let r = compose_resizepic(&parts, 30, 20).unwrap();
        assert_eq!((r.width, r.height), (30, 20));
        assert_eq!(r.rgba[0], 10); // top-left corner piece
        let last = (30 * 20 - 1) * 4;
        assert_eq!(r.rgba[last], 18); // bottom-right corner piece
    }
}
