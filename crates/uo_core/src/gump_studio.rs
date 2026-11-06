//! GumpStudio model: a visual gump (dialog) made of elements, saved as JSON and exported to
//! RunUO (C#), SphereScript dialogs and POL (gumps.inc) scripts.
//!
//! The element set follows https://github.com/mbnunes/GumpStudio (background, image, tiled image,
//! item, label, HTML / localized HTML, button, checkbox, radio, text entry, alpha region).

use serde::{Deserialize, Serialize};

use crate::gumps::GumpBitmap;
use crate::html_render::html_to_sphere_defs;

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
    /// `font`: Unicode font number (unifont*.mul) or, with `ascii`, an ASCII font (fonts.mul).
    /// The font is a client-side choice and does not change the exported code.
    Label {
        text: String,
        hue: u32,
        #[serde(default = "default_font")]
        font: u8,
        #[serde(default)]
        ascii: bool,
    },
    Html { text: String, cliloc: u32, background: bool, scrollbar: bool },
    Button { normal: u32, pressed: u32, action: ButtonAction, page: u32, id: i32 },
    Checkbox { off: u32, on: u32, checked: bool, id: i32 },
    Radio { off: u32, on: u32, checked: bool, id: i32 },
    TextEntry { hue: u32, text: String, id: i32 },
    Alpha,
}

fn default_font() -> u8 {
    1
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
            "label" => (Self::Label { text: "Texto".into(), hue: 0, font: 1, ascii: false }, 0, 0),
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

/// Repeats an element `count` times, moving it by (`step_x`, `step_y`) each time. Exported as a
/// loop (`for` in RunUO / POL, `FOR` with `<eval>` in Sphere); button, switch and entry ids grow by
/// one per repetition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repeat {
    pub count: u32,
    pub step_x: i32,
    pub step_y: i32,
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
    #[serde(default)]
    pub repeat: Option<Repeat>,
    /// elements with the same non-zero group id move together in the editor (0 = no group)
    #[serde(default)]
    pub group: u32,
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
    /// Indexes that move together with element `i`: its whole group when it has one.
    pub fn group_members(&self, i: usize) -> Vec<usize> {
        match self.elements.get(i).map(|e| e.group).filter(|&g| g != 0) {
            Some(g) => self.elements.iter().enumerate().filter(|(_, e)| e.group == g).map(|(k, _)| k).collect(),
            None => vec![i],
        }
    }

    /// Puts the given elements in a new group (the next free id); returns the id.
    pub fn group_elements(&mut self, idx: &[usize]) -> u32 {
        let id = self.elements.iter().map(|e| e.group).max().unwrap_or(0) + 1;
        for &i in idx {
            if let Some(e) = self.elements.get_mut(i) {
                e.group = id;
            }
        }
        id
    }

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
            .map(|e| {
                // a repeated element uses `count` consecutive ids starting at its own
                let extra = e.repeat.map_or(0, |r| r.count.saturating_sub(1) as i32);
                match &e.kind {
                    ElementKind::Button { id, action: ButtonAction::Reply, .. } => {
                        if *id > 0 { *id } else { let v = next(&btn_used, &mut nb); nb += extra; v }
                    }
                    ElementKind::Checkbox { id, .. } | ElementKind::Radio { id, .. } => {
                        if *id > 0 { *id } else { let v = next(&sw_used, &mut ns); ns += extra; v }
                    }
                    ElementKind::TextEntry { id, .. } => {
                        if *id > 0 { *id } else { let v = next(&te_used, &mut nt); nt += extra; v }
                    }
                    _ => 0,
                }
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum LoopStyle {
    /// C# / POL: loop variable `i`
    Code,
    /// Sphere: `<LOCAL._FOR>` inside `<eval ...>`
    Sphere,
}

/// Position / id expressions of an element: plain numbers, or expressions in the loop variable for
/// a repeated element.
fn loop_exprs(e: &GumpElement, id: i32, style: LoopStyle) -> (String, String, String) {
    let Some(r) = e.repeat.filter(|r| r.count > 1) else {
        return (e.x.to_string(), e.y.to_string(), id.to_string());
    };
    let expr = |base: i32, step: i32| -> String {
        match style {
            LoopStyle::Code => if step == 0 { base.to_string() } else { format!("{} + i * {}", base, step) },
            LoopStyle::Sphere => if step == 0 { base.to_string() } else { format!("<eval {}+(<LOCAL._FOR>*{})>", base, step) },
        }
    };
    let idv = match style {
        LoopStyle::Code => format!("{} + i", id),
        LoopStyle::Sphere => format!("<eval {}+<LOCAL._FOR>>", id),
    };
    (expr(e.x, r.step_x), expr(e.y, r.step_y), idv)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    Cs,
    Pol,
    Sphere,
}

/// One generated statement, wrapped in a loop when the element repeats.
fn emit_loop(line: &str, rep: Option<Repeat>, lang: Lang, indent: &str) -> String {
    match (rep.filter(|r| r.count > 1), lang) {
        (None, _) => format!("{}{}\n", indent, line),
        (Some(r), Lang::Cs) => format!("{i}for ( int i = 0; i < {n}; i++ )\n{i}\t{l}\n", i = indent, n = r.count, l = line),
        (Some(r), Lang::Pol) => format!("{i}for ( i := 0; i < {n}; i := i + 1 )\n{i}\t{l}\n{i}endfor\n", i = indent, n = r.count, l = line),
        (Some(r), Lang::Sphere) => format!("FOR 0 {}\n {}\nENDFOR\n", r.count - 1, line),
    }
}

/// Plain text for the inline `dtext` family: one line, with `<`/`>` as Sphere defnames.
fn sphere_plain(text: &str) -> String {
    text.replace('\n', " ").replace('<', "<DEF.L1>").replace('>', "<DEF.R1>")
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
            let rep = e.repeat;
            let (ex, ey, idv) = loop_exprs(e, ids[i], LoopStyle::Code);
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("AddBackground( {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, gump_id),
                ElementKind::Image { gump_id, hue } => {
                    if *hue != 0 { format!("AddImage( {}, {}, {}, {} );", ex, ey, gump_id, hue) } else { format!("AddImage( {}, {}, {} );", ex, ey, gump_id) }
                }
                ElementKind::Tiled { gump_id } => format!("AddImageTiled( {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => {
                    if *hue != 0 { format!("AddItem( {}, {}, 0x{:X}, {} );", ex, ey, item_id, hue) } else { format!("AddItem( {}, {}, 0x{:X} );", ex, ey, item_id) }
                }
                ElementKind::Label { text, hue, .. } => format!("AddLabel( {}, {}, {}, \"{}\" );", ex, ey, hue, cs_escape(text)),
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    if *cliloc != 0 {
                        format!("AddHtmlLocalized( {}, {}, {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, cliloc, b(*background), b(*scrollbar))
                    } else {
                        format!("AddHtml( {}, {}, {}, {}, \"{}\", {}, {} );", ex, ey, e.w, e.h, cs_escape(text), b(*background), b(*scrollbar))
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => {
                        for k in 0..e.repeat.map_or(1, |r| r.count.max(1)) {
                            reply_ids.push((ids[i] + k as i32, e.name.clone()));
                        }
                        format!("AddButton( {}, {}, {}, {}, {}, GumpButtonType.Reply, 0 );", ex, ey, normal, pressed, idv)
                    }
                    ButtonAction::Page => format!("AddButton( {}, {}, {}, {}, 0, GumpButtonType.Page, {} );", ex, ey, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("AddCheck( {}, {}, {}, {}, {}, {} );", ex, ey, off, on, b(*checked), idv),
                ElementKind::Radio { off, on, checked, .. } => format!("AddRadio( {}, {}, {}, {}, {}, {} );", ex, ey, off, on, b(*checked), idv),
                ElementKind::TextEntry { hue, text, .. } => format!("AddTextEntry( {}, {}, {}, {}, {}, {}, \"{}\" );", ex, ey, e.w, e.h, hue, idv, cs_escape(text)),
                ElementKind::Alpha => format!("AddAlphaRegion( {}, {}, {}, {} );", ex, ey, e.w, e.h),
            };
            s.push_str(&emit_loop(&line, rep, Lang::Cs, "\t\t\t"));
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
            let rep = e.repeat;
            let (ex, ey, idv) = loop_exprs(e, ids[i], LoopStyle::Sphere);
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("resizepic {} {} {} {} {}", ex, ey, gump_id, e.w, e.h),
                ElementKind::Image { gump_id, hue } => {
                    if *hue != 0 { format!("gumppic {} {} {} hue={}", ex, ey, gump_id, hue) } else { format!("gumppic {} {} {}", ex, ey, gump_id) }
                }
                ElementKind::Tiled { gump_id } => format!("gumppictiled {} {} {} {} {}", ex, ey, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => {
                    if *hue != 0 { format!("tilepichue {} {} {} {}", ex, ey, item_id, hue) } else { format!("tilepic {} {} {}", ex, ey, item_id) }
                }
                ElementKind::Label { text, hue, .. } => format!("dtext {} {} {} {}{}", ex, ey, hue, if text.starts_with('.') { "." } else { "" }, sphere_plain(text)),
                ElementKind::Html { text, cliloc, background, scrollbar } => {
                    if *cliloc != 0 {
                        format!("xmfhtmlgump {} {} {} {} {} {} {}", ex, ey, e.w, e.h, cliloc, *background as u8, *scrollbar as u8)
                    } else {
                        format!("dhtmlgump {} {} {} {} {} {} {}", ex, ey, e.w, e.h, *background as u8, *scrollbar as u8, html_to_sphere_defs(&text.replace('\n', " ")))
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => {
                        for k in 0..e.repeat.map_or(1, |r| r.count.max(1)) {
                            reply.push((ids[i] + k as i32, e.name.clone()));
                        }
                        format!("button {} {} {} {} 1 0 {}", ex, ey, normal, pressed, idv)
                    }
                    ButtonAction::Page => format!("button {} {} {} {} 0 {} 0", ex, ey, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("checkbox {} {} {} {} {} {}", ex, ey, off, on, *checked as u8, idv),
                ElementKind::Radio { off, on, checked, .. } => format!("radio {} {} {} {} {} {}", ex, ey, off, on, *checked as u8, idv),
                ElementKind::TextEntry { hue, text, .. } => format!("dtextentry {} {} {} {} {} {} {}", ex, ey, e.w, e.h, hue, idv, sphere_plain(text)),
                ElementKind::Alpha => format!("checkertrans {} {} {} {}", ex, ey, e.w, e.h),
            };
            s.push_str(&emit_loop(&line, rep, Lang::Sphere, ""));
        }
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
    if doc.elements.iter().any(|e| e.repeat.map_or(false, |r| r.count > 1)) {
        s.push_str("\tvar i;\n");
    }
    if !doc.movable || !doc.closable || !doc.disposable {
        s.push_str(&format!("\t// movable={} closable={} disposable={} (set through the gump flags of your include)\n", doc.movable, doc.closable, doc.disposable));
    }
    for (page, idxs) in doc.by_page() {
        s.push_str(&format!("\tGFPage( gump, {} );\n", page));
        for i in idxs {
            let e = &doc.elements[i];
            let rep = e.repeat;
            let (ex, ey, idv) = loop_exprs(e, ids[i], LoopStyle::Code);
            let line = match &e.kind {
                ElementKind::Background { gump_id } => format!("GFResizePic( gump, {}, {}, {}, {}, {} );", ex, ey, gump_id, e.w, e.h),
                ElementKind::Image { gump_id, .. } => format!("GFGumpPic( gump, {}, {}, {} );", ex, ey, gump_id),
                ElementKind::Tiled { gump_id } => format!("GFGumpPicTiled( gump, {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, gump_id),
                ElementKind::Item { item_id, hue } => format!("GFTilePic( gump, {}, {}, 0x{:X}, {} );", ex, ey, item_id, hue),
                ElementKind::Label { text, hue, .. } => format!("GFTextLine( gump, {}, {}, {}, {} );", ex, ey, hue, q(text)),
                ElementKind::Html { text, background, scrollbar, cliloc } => {
                    if *cliloc != 0 {
                        format!("GFHTMLArea( gump, {}, {}, {}, {}, {}, {}, {} ); // cliloc {} is not supported by gumps.inc: use a plain text", ex, ey, e.w, e.h, q(""), *background as u8, *scrollbar as u8, cliloc)
                    } else {
                        format!("GFHTMLArea( gump, {}, {}, {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, q(text), *background as u8, *scrollbar as u8)
                    }
                }
                ElementKind::Button { normal, pressed, action, page, .. } => match action {
                    ButtonAction::Reply => format!("GFAddButton( gump, {}, {}, {}, {}, GF_CLOSE_BTN, {} );", ex, ey, normal, pressed, idv),
                    ButtonAction::Page => format!("GFAddButton( gump, {}, {}, {}, {}, GF_PAGE_BTN, {} );", ex, ey, normal, pressed, page),
                },
                ElementKind::Checkbox { off, on, checked, .. } => format!("GFCheckBox( gump, {}, {}, {}, {}, {}, {} );", ex, ey, off, on, *checked as u8, idv),
                ElementKind::Radio { off, on, checked, .. } => format!("GFRadioButton( gump, {}, {}, {}, {}, {}, {} );", ex, ey, off, on, *checked as u8, idv),
                ElementKind::TextEntry { hue, text, .. } => format!("GFTextEntry( gump, {}, {}, {}, {}, {}, {}, {} );", ex, ey, e.w, e.h, hue, q(text), idv),
                ElementKind::Alpha => format!("GFCheckerTrans( gump, {}, {}, {}, {} );", ex, ey, e.w, e.h),
            };
            s.push_str(&emit_loop(&line, rep, Lang::Pol, "\t"));
        }
    }
    s.push_str("\n\tvar input := GFSendGump( who, gump );\n\tif ( !input || input[0] == 0 )\n\t\treturn; // closed\n\tendif\n\n\tcase ( input[0] )\n");
    for (i, e) in doc.elements.iter().enumerate() {
        if let ElementKind::Button { action: ButtonAction::Reply, .. } = e.kind {
            for k in 0..e.repeat.map_or(1, |r| r.count.max(1)) {
                s.push_str(&format!("\t\t{}: // {}\n\t\t\tbreak;\n", ids[i] + k as i32, e.name));
            }
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

/// A saved combination of elements (backgrounds, images, buttons...) that can be inserted into any gump.
/// Positions are stored relative to the top-left corner of the group.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GumpFavorite {
    pub name: String,
    pub elements: Vec<GumpElement>,
}

impl GumpFavorite {
    /// Builds a favorite from `elements`, moving them so the top-left corner of the group is (0, 0).
    pub fn from_elements(name: &str, elements: &[GumpElement]) -> Self {
        let (min_x, min_y) = (elements.iter().map(|e| e.x).min().unwrap_or(0), elements.iter().map(|e| e.y).min().unwrap_or(0));
        let elements = elements.iter().cloned().map(|mut e| {
            e.x -= min_x;
            e.y -= min_y;
            e
        }).collect();
        Self { name: name.to_string(), elements }
    }

    /// Copies of the elements placed with the group's top-left corner at (x, y) on `page`.
    pub fn instantiate(&self, x: i32, y: i32, page: u32) -> Vec<GumpElement> {
        self.elements.iter().cloned().map(|mut e| {
            e.x += x;
            e.y += y;
            e.page = page;
            e
        }).collect()
    }
}

/// The user's list of favorites, saved as JSON.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Favorites {
    pub items: Vec<GumpFavorite>,
}

impl Favorites {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

/// Manual lists of resizepic start ids: `valid` ids are always listed (when the gumps exist), `invalid`
/// ids and ranges are never listed. Format of the text files: `valid 83`, `invalid 2240-2302`, `#` comments.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResizepicLists {
    pub valid: Vec<std::ops::RangeInclusive<u32>>,
    pub invalid: Vec<std::ops::RangeInclusive<u32>>,
}

impl ResizepicLists {
    /// The list shipped with the program (`data/resizepics.txt`).
    pub fn builtin() -> Self {
        Self::parse(include_str!("../data/resizepics.txt"))
    }

    pub fn parse(text: &str) -> Self {
        let mut l = Self::default();
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            let mut it = line.split_whitespace();
            let (Some(kind), Some(range)) = (it.next(), it.next()) else { continue };
            let (a, b) = range.split_once('-').unwrap_or((range, range));
            let (Ok(a), Ok(b)) = (a.trim().parse::<u32>(), b.trim().parse::<u32>()) else { continue };
            let r = a.min(b)..=a.max(b);
            match kind {
                "valid" => l.valid.push(r),
                "invalid" => l.invalid.push(r),
                _ => {}
            }
        }
        l
    }

    /// Adds the entries of `other` (for example the user's own file).
    pub fn merged(mut self, other: Self) -> Self {
        self.valid.extend(other.valid);
        self.invalid.extend(other.invalid);
        self
    }

    pub fn is_invalid(&self, id: u32) -> bool {
        self.invalid.iter().any(|r| r.contains(&id))
    }

    /// The invalid range that contains `id`, to skip it whole.
    pub fn invalid_range(&self, id: u32) -> Option<&std::ops::RangeInclusive<u32>> {
        self.invalid.iter().find(|r| r.contains(&id))
    }

    pub fn is_valid(&self, id: u32) -> bool {
        !self.is_invalid(id) && self.valid.iter().any(|r| r.contains(&id))
    }

    /// Every individual valid id (ranges expanded, invalid ones removed).
    pub fn valid_ids(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.valid.iter().flat_map(|r| r.clone()).filter(|&i| !self.is_invalid(i)).collect();
        v.sort_unstable();
        v.dedup();
        v
    }
}

/// Ids that start a resizepic: nine consecutive gumps `id..id + 8` in the order
/// corner, edge, corner / edge, centre, edge / corner, edge, corner where
/// - the four corners have the same size,
/// - opposite edges have the same size (top = bottom, left = right),
/// - the top and bottom edges are as tall as the corners, the side edges as wide as the corners,
/// - no piece is larger than 512 pixels (a frame is made of small tiles),
/// - and `seamless(id)` accepts the pixels (see `frame_tiles_seamlessly`).
/// `size_of` returns the (width, height) of a gump, `None` when it does not exist.
pub fn find_resizepics(max_id: u32, size_of: impl Fn(u32) -> Option<(u32, u32)>, seamless: impl Fn(u32) -> bool, lists: &ResizepicLists) -> Vec<u32> {
    let mut out = Vec::new();
    let mut id = 0;
    while id + 8 <= max_id {
        if let Some(r) = lists.invalid_range(id) {
            id = *r.end() + 1; // skip the whole curated range
            continue;
        }
        if is_resizepic_start(id, &size_of, &seamless) {
            out.push(id);
            id += 9; // the pieces of one resizepic never start another
        } else {
            id += 1;
        }
    }
    // the manual `valid` list is always included when the nine gumps exist
    for v in lists.valid_ids() {
        if (0..9).all(|k| size_of(v + k).is_some()) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The rules of `find_resizepics` for one id (the curated exclusions are applied by the caller).
pub fn is_resizepic_start(id: u32, size_of: &impl Fn(u32) -> Option<(u32, u32)>, seamless: &impl Fn(u32) -> bool) -> bool {
    let Some(s) = (0..9).map(|k| size_of(id + k)).collect::<Option<Vec<(u32, u32)>>>() else {
        return false;
    };
    let (tl, t, tr, l, c, r, bl, b, br) = (s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8]);
    let corners_equal = tl == tr && tl == bl && tl == br;
    let opposite_edges_equal = t == b && l == r;
    let edges_fit = t.1 == tl.1 && l.0 == tl.0;
    let small = s.iter().all(|&(w, h)| w > 0 && h > 0 && w <= 512 && h <= 512);
    // proportions of a frame: corners and centre are squarish, the borders are slender strips
    let ratio = |(w, h): (u32, u32)| w.max(h) as f32 / w.min(h).max(1) as f32;
    let proportions = ratio(tl) <= MAX_CORNER_RATIO && ratio(c) <= MAX_CENTRE_RATIO && ratio(t) >= MIN_EDGE_RATIO && ratio(l) >= MIN_EDGE_RATIO;
    corners_equal && opposite_edges_equal && edges_fit && small && proportions && c.0 > 0 && seamless(id)
}

/// Proportion limits (longest side / shortest side) taken from the known frames: corners are square
/// (1.0), centres at most 1.4, border strips at least 9.3. Sets whose nine pieces are all the same
/// square size (picture pages, icons) therefore fail; such a frame can still be added to the manual list.
pub const MAX_CORNER_RATIO: f32 = 1.6;
pub const MAX_CENTRE_RATIO: f32 = 2.0;
pub const MIN_EDGE_RATIO: f32 = 2.5;

/// True when the nine pieces of a frame repeat without a visible seam: the first and last column
/// (rows for the side edges) of the top/bottom edges, the side edges and the centre must match.
/// Pieces cut out of a single picture do not have that property, and fully transparent pieces are not frames.
pub fn frame_tiles_seamlessly(parts: &[GumpBitmap]) -> bool {
    if parts.len() < 9 {
        return false;
    }
    // mean absolute difference between the first and the last column (`horizontal`) or row
    let seam = |b: &GumpBitmap, horizontal: bool| -> f32 {
        let (w, h) = (b.width as usize, b.height as usize);
        if w == 0 || h == 0 {
            return f32::MAX;
        }
        let px = |x: usize, y: usize| &b.rgba[(y * w + x) * 4..(y * w + x) * 4 + 4];
        let n = if horizontal { h } else { w };
        let total: f32 = (0..n)
            .map(|i| {
                let (a, c) = if horizontal { (px(0, i), px(w - 1, i)) } else { (px(i, 0), px(i, h - 1)) };
                (0..4).map(|k| (a[k] as f32 - c[k] as f32).abs()).sum::<f32>() / 4.0
            })
            .sum();
        total / n as f32
    };
    // a frame has visible content: opaque centre and a corner that is not empty
    let opaque = |b: &GumpBitmap| b.rgba.chunks_exact(4).filter(|p| p[3] > 0).count() as f32 / (b.rgba.len() / 4).max(1) as f32;
    if opaque(&parts[4]) < 0.5 || opaque(&parts[0]) < 0.1 {
        return false;
    }
    let edges = seam(&parts[1], true).max(seam(&parts[7], true)).max(seam(&parts[3], false)).max(seam(&parts[5], false));
    let centre = seam(&parts[4], true).max(seam(&parts[4], false));
    edges <= 35.0 && centre <= 10.0
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
        GumpElement { name: "el".into(), x, y, w, h, page, kind, repeat: None, group: 0 }
    }

    fn sample() -> GumpDoc {
        let mut d = GumpDoc::default();
        d.name = "My Gump".into();
        d.elements = vec![
            el(ElementKind::Background { gump_id: 9200 }, 0, 0, 300, 200, 0),
            el(ElementKind::Label { text: "He said \"hi\"".into(), hue: 52, font: 1, ascii: false }, 20, 20, 0, 0, 0),
            el(ElementKind::Button { normal: 247, pressed: 248, action: ButtonAction::Reply, page: 0, id: 0 }, 20, 160, 0, 0, 0),
            el(ElementKind::Button { normal: 4005, pressed: 4007, action: ButtonAction::Page, page: 2, id: 0 }, 60, 160, 0, 0, 1),
            el(ElementKind::Checkbox { off: 210, on: 211, checked: true, id: 0 }, 20, 60, 0, 0, 1),
            el(ElementKind::TextEntry { hue: 0, text: "abc".into(), id: 0 }, 20, 90, 150, 20, 2),
            el(ElementKind::Item { item_id: 0xEED, hue: 0 }, 200, 20, 0, 0, 0),
        ];
        d
    }

    #[test]
    fn groups_move_together() {
        let mut d = GumpDoc::default();
        d.elements = vec![
            el(ElementKind::Alpha, 0, 0, 10, 10, 0),
            el(ElementKind::Alpha, 5, 5, 10, 10, 0),
            el(ElementKind::Alpha, 9, 9, 10, 10, 0),
        ];
        assert_eq!(d.group_members(0), vec![0], "no group: only itself");
        let g = d.group_elements(&[0, 2]);
        assert_eq!(g, 1);
        assert_eq!(d.group_members(2), vec![0, 2]);
        assert_eq!(d.group_elements(&[1]), 2, "next free group id");
        // old files without the field still load (group = 0)
        let old = r#"{"name":"x","x":0,"y":0,"closable":true,"movable":true,"disposable":true,"resizable":false,"elements":[{"name":"a","x":1,"y":2,"w":3,"h":4,"page":0,"kind":{"type":"Alpha"}}]}"#;
        assert_eq!(GumpDoc::from_json(old).unwrap().elements[0].group, 0);
    }

    #[test]
    fn favorites_keep_relative_positions_and_roundtrip() {
        let els = vec![
            el(ElementKind::Background { gump_id: 9200 }, 50, 40, 200, 100, 0),
            el(ElementKind::Label { text: "t".into(), hue: 0, font: 1, ascii: false }, 60, 55, 0, 0, 2),
        ];
        let fav = GumpFavorite::from_elements("frame", &els);
        assert_eq!((fav.elements[0].x, fav.elements[0].y), (0, 0));
        assert_eq!((fav.elements[1].x, fav.elements[1].y), (10, 15));
        let placed = fav.instantiate(100, 20, 3);
        assert_eq!((placed[1].x, placed[1].y), (110, 35));
        assert!(placed.iter().all(|e| e.page == 3));
        let list = Favorites { items: vec![fav] };
        assert_eq!(Favorites::from_json(&list.to_json()).unwrap(), list);
        assert!(Favorites::from_json("not json").is_err());
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
        assert!(c.contains("dtext 20 20 52 He said \"hi\""));
        assert!(c.contains("button 20 160 247 248 1 0 1"));
        assert!(c.contains("button 60 160 4005 4007 0 2 0"));
        assert!(c.contains("dtextentry 20 90 150 20 0 1 abc"));
        assert!(!c.contains("TEXT]"), "no separate text section");
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
    fn sphere_html_uses_inline_defnames() {
        let mut d = GumpDoc::default();
        d.elements = vec![el(ElementKind::Html { text: "<b>Hi</b> 1 < 2".into(), cliloc: 0, background: true, scrollbar: false }, 5, 6, 100, 50, 0)];
        let c = export_sphere(&d);
        assert!(c.contains("dhtmlgump 5 6 100 50 1 0 <DEF.B>Hi<DEF.BE> 1 <DEF.L1> 2"), "{c}");
    }

    #[test]
    fn repeated_elements_export_as_loops() {
        let mut d = GumpDoc::default();
        let mut b = el(ElementKind::Button { normal: 4005, pressed: 4007, action: ButtonAction::Reply, page: 0, id: 0 }, 10, 20, 0, 0, 0);
        b.repeat = Some(Repeat { count: 3, step_x: 0, step_y: 25 });
        let other = el(ElementKind::Button { normal: 1, pressed: 2, action: ButtonAction::Reply, page: 0, id: 0 }, 0, 0, 0, 0, 0);
        d.elements = vec![b, other];
        assert_eq!(d.resolved_ids(), vec![1, 4], "the loop reserves ids 1..=3");
        let cs = export_runuo(&d);
        assert!(cs.contains("for ( int i = 0; i < 3; i++ )\n\t\t\t\tAddButton( 10, 20 + i * 25, 4005, 4007, 1 + i, GumpButtonType.Reply, 0 );"), "{cs}");
        assert!(cs.contains("case 3:") && cs.contains("case 4:"));
        let sp = export_sphere(&d);
        assert!(sp.contains("FOR 0 2\n button 10 <eval 20+(<LOCAL._FOR>*25)> 4005 4007 1 0 <eval 1+<LOCAL._FOR>>\nENDFOR"), "{sp}");
        let pol = export_pol(&d);
        assert!(pol.contains("var i;") && pol.contains("for ( i := 0; i < 3; i := i + 1 )"), "{pol}");
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
    fn finds_nine_piece_frames_by_size_order_and_seams() {
        // gumps 100..=108 form a frame (square 6x6 corners, slender 30x6 / 6x30 borders, 30x30 centre)
        let frame = [(6, 6), (30, 6), (6, 6), (6, 30), (30, 30), (6, 30), (6, 6), (30, 6), (6, 6)];
        // 200..=208: same size pattern but the pieces are cut from one picture (no seams)
        let size_of = |id: u32| -> Option<(u32, u32)> {
            if (100..109).contains(&id) { Some(frame[(id - 100) as usize]) } else if (200..209).contains(&id) { Some(frame[(id - 200) as usize]) } else if id == 50 || id == 51 { Some((8, 8)) } else { None }
        };
        let seamless = |id: u32| id == 100;
        let none = ResizepicLists::default();
        assert_eq!(find_resizepics(300, size_of, seamless, &none), vec![100]);
        // a curated non-frame range is skipped even when the structure fits, including the phase-shifted starts after it
        let in_range = |id: u32| -> Option<(u32, u32)> { (2240..2262).contains(&id).then(|| frame[((id - 2240) % 9) as usize]) };
        let builtin = ResizepicLists::builtin();
        assert!(find_resizepics(3000, in_range, |_| true, &builtin).is_empty());
        let single = |id: u32| -> Option<(u32, u32)> { (1423..1432).contains(&id).then(|| frame[(id - 1423) as usize]) };
        assert!(find_resizepics(2000, single, |_| true, &builtin).is_empty(), "1423 is curated out");
        // corners of different sizes are never a frame
        let bad = |id: u32| -> Option<(u32, u32)> { (300..309).contains(&id).then(|| if id == 302 { (7, 6) } else { frame[(id - 300) as usize] }) };
        assert!(find_resizepics(400, bad, |_| true, &none).is_empty());
        // opposite edges must be equal
        let bad2 = |id: u32| -> Option<(u32, u32)> { (300..309).contains(&id).then(|| if id == 307 { (31, 6) } else { frame[(id - 300) as usize] }) };
        assert!(find_resizepics(400, bad2, |_| true, &none).is_empty());
    }

    #[test]
    fn proportions_reject_square_pieces_and_the_manual_list_overrides() {
        // pieces all 70x70 (icon pages): the borders are not slender
        let square = |id: u32| -> Option<(u32, u32)> { (7000..7009).contains(&id).then_some((70, 70)) };
        let none = ResizepicLists::default();
        assert!(find_resizepics(8000, square, |_| true, &none).is_empty());
        // a valid manual entry is listed even though the proportions fail
        let manual = ResizepicLists::parse("valid 7000 # forced\ninvalid 1-2");
        assert_eq!(find_resizepics(8000, square, |_| true, &manual), vec![7000]);
        // invalid wins over valid
        let both = ResizepicLists::parse("valid 7000\ninvalid 7000");
        assert!(find_resizepics(8000, square, |_| true, &both).is_empty());
        // real frame proportions (9200: corners 7x7, top 427x7, side 7x316, centre 128x128) pass
        let real = [(7, 7), (427, 7), (7, 7), (7, 316), (128, 128), (7, 316), (7, 7), (427, 7), (7, 7)];
        let size_of = |id: u32| (9200..9209).contains(&id).then(|| real[(id - 9200) as usize]);
        assert_eq!(find_resizepics(9300, size_of, |_| true, &none), vec![9200]);
    }

    #[test]
    fn resizepic_list_parsing() {
        let l = ResizepicLists::parse("# c\nvalid 83 # x\ninvalid 10-12\nbogus 1\nvalid x\n");
        assert_eq!(l.valid_ids(), vec![83]);
        assert!(l.is_invalid(11) && !l.is_invalid(13));
        assert!(ResizepicLists::builtin().is_valid(83) && ResizepicLists::builtin().is_invalid(7500));
    }

    #[test]
    fn seams_tell_tiles_from_picture_pieces() {
        let flat = |v: u8| GumpBitmap { width: 6, height: 6, rgba: vec![v; 6 * 6 * 4] }; // opaque grey (alpha = v)
        let gradient = GumpBitmap { width: 6, height: 6, rgba: (0..6 * 6 * 4).map(|i| if (i / 4) % 6 < 3 { 0 } else { 255 }).collect() };
        let tiles: Vec<GumpBitmap> = (0..9).map(|_| flat(100)).collect();
        assert!(frame_tiles_seamlessly(&tiles));
        let mut picture = tiles.clone();
        picture[1] = gradient; // top edge: left half black, right half white
        assert!(!frame_tiles_seamlessly(&picture));
        let empty: Vec<GumpBitmap> = (0..9).map(|_| GumpBitmap { width: 6, height: 6, rgba: vec![0; 6 * 6 * 4] }).collect();
        assert!(!frame_tiles_seamlessly(&empty), "transparent pieces are not a frame");
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
