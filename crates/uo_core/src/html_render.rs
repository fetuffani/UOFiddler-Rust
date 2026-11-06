//! Simulation of the HTML subset the UO client renders in htmlgump text, drawn with the client's
//! Unicode fonts. Supported tags (the ones used by the Sphere scripts' `def_html`): `b`, `i`, `em`,
//! `u`, `big`, `small`, `h1`..`h6`, `a href`, `br`, `center`, `div align=left|right`,
//! `basefont size=1..7 color=#rrggbb`. Other tags are ignored.
//!
//! The client's exact font mapping for sizes and for bold/italic is not specified anywhere we
//! can cite, so those are approximations: sizes scale the bitmap, bold draws the glyphs twice,
//! italic shears them.

use crate::fonts::UnicodeFont;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Style {
    bold: bool,
    italic: bool,
    underline: bool,
    size: u8,
    color: [u8; 4],
    align: Align,
}

#[derive(Debug, PartialEq)]
enum Tok {
    Text(String),
    Open(String, Vec<(String, String)>),
    Close(String),
}

fn parse_attrs(s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let b: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < b.len() {
        while i < b.len() && b[i].is_whitespace() {
            i += 1;
        }
        let ks = i;
        while i < b.len() && b[i] != '=' && !b[i].is_whitespace() {
            i += 1;
        }
        let key: String = b[ks..i].iter().collect::<String>().to_lowercase();
        if key.is_empty() {
            break;
        }
        while i < b.len() && b[i].is_whitespace() {
            i += 1;
        }
        let mut val = String::new();
        if i < b.len() && b[i] == '=' {
            i += 1;
            while i < b.len() && b[i].is_whitespace() {
                i += 1;
            }
            if i < b.len() && (b[i] == '"' || b[i] == '\'') {
                let q = b[i];
                i += 1;
                while i < b.len() && b[i] != q {
                    val.push(b[i]);
                    i += 1;
                }
                i += 1;
            } else {
                while i < b.len() && !b[i].is_whitespace() {
                    val.push(b[i]);
                    i += 1;
                }
            }
        }
        out.push((key, val));
    }
    out
}

fn tokenize(html: &str) -> Vec<Tok> {
    let mut toks = Vec::new();
    let mut text = String::new();
    let chars: Vec<char> = html.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(rel) = chars[i + 1..].iter().position(|&c| c == '>') {
                let raw: String = chars[i + 1..i + 1 + rel].iter().collect();
                let inner = raw.trim();
                // a tag name follows '<' directly, and no other '<' may open before the '>'
                let looks_like_tag = raw.chars().next().map_or(false, |c| c.is_ascii_alphabetic() || c == '/') && !raw.contains('<');
                if looks_like_tag {
                    if !text.is_empty() {
                        toks.push(Tok::Text(std::mem::take(&mut text)));
                    }
                    if let Some(name) = inner.strip_prefix('/') {
                        toks.push(Tok::Close(name.trim().to_lowercase()));
                    } else {
                        let (name, rest) = inner.split_once(char::is_whitespace).unwrap_or((inner, ""));
                        let rest = rest.trim_end_matches('/');
                        toks.push(Tok::Open(name.to_lowercase(), parse_attrs(rest)));
                    }
                    i += rel + 2;
                    continue;
                }
            }
        }
        text.push(chars[i]);
        i += 1;
    }
    if !text.is_empty() {
        toks.push(Tok::Text(text));
    }
    toks
}

fn decode_entities(s: &str) -> String {
    s.replace("&nbsp;", " ").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
}

fn parse_color(v: &str) -> Option<[u8; 4]> {
    let h = v.trim().strip_prefix('#')?;
    if h.len() != 6 {
        return None;
    }
    let n = u32::from_str_radix(h, 16).ok()?;
    Some([(n >> 16) as u8, (n >> 8) as u8, n as u8, 255])
}

/// Scale of the bitmap for the size levels 1..=7 (3 is the normal size).
fn size_scale(size: u8) -> f32 {
    [0.7, 0.85, 1.0, 1.25, 1.5, 1.75, 2.0][(size.clamp(1, 7) - 1) as usize]
}

struct Piece {
    w: usize,
    h: usize,
    rgba: Vec<u8>,
}

struct Line {
    pieces: Vec<(i32, Piece)>,
    width: i32,
    height: i32,
    align: Align,
}

fn scale_nearest(src: &Piece, f: f32) -> Piece {
    if (f - 1.0).abs() < 0.01 {
        return Piece { w: src.w, h: src.h, rgba: src.rgba.clone() };
    }
    let (nw, nh) = (((src.w as f32 * f).round() as usize).max(1), ((src.h as f32 * f).round() as usize).max(1));
    let mut rgba = vec![0u8; nw * nh * 4];
    for y in 0..nh {
        let sy = ((y as f32 / f) as usize).min(src.h - 1);
        for x in 0..nw {
            let sx = ((x as f32 / f) as usize).min(src.w - 1);
            let (s, d) = ((sy * src.w + sx) * 4, (y * nw + x) * 4);
            rgba[d..d + 4].copy_from_slice(&src.rgba[s..s + 4]);
        }
    }
    Piece { w: nw, h: nh, rgba }
}

fn styled_word(font: &UnicodeFont, word: &str, st: &Style) -> Option<Piece> {
    let (w, h, rgba) = font.render(word, st.color, None)?;
    let mut p = scale_nearest(&Piece { w, h, rgba }, size_scale(st.size));
    if st.bold {
        let mut b = vec![0u8; (p.w + 1) * p.h * 4];
        for y in 0..p.h {
            for x in 0..p.w {
                let s = (y * p.w + x) * 4;
                if p.rgba[s + 3] > 0 {
                    for dx in [0usize, 1] {
                        let d = (y * (p.w + 1) + x + dx) * 4;
                        b[d..d + 4].copy_from_slice(&p.rgba[s..s + 4]);
                    }
                }
            }
        }
        p = Piece { w: p.w + 1, h: p.h, rgba: b };
    }
    if st.italic {
        let shear = p.h / 4 + 1;
        let mut b = vec![0u8; (p.w + shear) * p.h * 4];
        for y in 0..p.h {
            let off = (p.h - 1 - y) / 4;
            for x in 0..p.w {
                let s = (y * p.w + x) * 4;
                if p.rgba[s + 3] > 0 {
                    let d = (y * (p.w + shear) + x + off) * 4;
                    b[d..d + 4].copy_from_slice(&p.rgba[s..s + 4]);
                }
            }
        }
        p = Piece { w: p.w + shear, h: p.h, rgba: b };
    }
    if st.underline {
        let y = ((15.0 * size_scale(st.size)) as usize).min(p.h.saturating_sub(1));
        for x in 0..p.w {
            let d = (y * p.w + x) * 4;
            p.rgba[d..d + 4].copy_from_slice(&st.color);
        }
    }
    Some(p)
}

/// Renders `html` at `width` pixels (word wrapping). Returns (width, height, RGBA).
pub fn render_html(font: &UnicodeFont, html: &str, default_color: [u8; 4], width: i32) -> Option<(usize, usize, Vec<u8>)> {
    let base = Style { bold: false, italic: false, underline: false, size: 3, color: default_color, align: Align::Left };
    let mut st = base;
    let mut stack: Vec<(String, Style)> = Vec::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut cur = Line { pieces: Vec::new(), width: 0, height: 0, align: Align::Left };
    let line_h = font.line_height;
    let width = width.max(8);

    macro_rules! new_line {
        ($align:expr) => {{
            let l = std::mem::replace(&mut cur, Line { pieces: Vec::new(), width: 0, height: 0, align: $align });
            lines.push(Line { height: l.height.max(line_h), ..l });
        }};
    }
    let break_block = |cur: &mut Line, lines: &mut Vec<Line>, align: Align| {
        if !cur.pieces.is_empty() {
            let l = std::mem::replace(cur, Line { pieces: Vec::new(), width: 0, height: 0, align });
            lines.push(Line { height: l.height.max(line_h), ..l });
        } else {
            cur.align = align;
        }
    };

    for tok in tokenize(html) {
        match tok {
            Tok::Text(t) => {
                let t = decode_entities(&t);
                for word in t.split_whitespace() {
                    let Some(p) = styled_word(font, word, &st) else { continue };
                    let space = (8.0 * size_scale(st.size)).round() as i32;
                    let need = p.w as i32 + if cur.pieces.is_empty() { 0 } else { space };
                    if !cur.pieces.is_empty() && cur.width + need > width {
                        new_line!(st.align);
                    }
                    let x = if cur.pieces.is_empty() { 0 } else { cur.width + space };
                    cur.width = x + p.w as i32;
                    cur.height = cur.height.max(p.h as i32 + 2);
                    cur.align = st.align;
                    cur.pieces.push((x, p));
                }
            }
            Tok::Open(name, attrs) => {
                let get = |k: &str| attrs.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str());
                let prev = st;
                match name.as_str() {
                    "b" => st.bold = true,
                    "i" | "em" => st.italic = true,
                    "u" => st.underline = true,
                    "big" => st.size = (st.size + 1).min(7),
                    "small" => st.size = st.size.saturating_sub(1).max(1),
                    "a" => {
                        st.underline = true;
                        st.color = [100, 150, 255, 255];
                    }
                    "br" => {
                        new_line!(st.align);
                        continue;
                    }
                    "center" => {
                        st.align = Align::Center;
                        break_block(&mut cur, &mut lines, Align::Center);
                    }
                    "div" => {
                        st.align = match get("align").map(|v| v.to_lowercase()).as_deref() {
                            Some("center") => Align::Center,
                            Some("right") => Align::Right,
                            _ => Align::Left,
                        };
                        break_block(&mut cur, &mut lines, st.align);
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        let n = name.as_bytes()[1] - b'0';
                        st.bold = true;
                        st.size = [7, 5, 4, 3, 2, 1][(n - 1) as usize];
                        break_block(&mut cur, &mut lines, st.align);
                    }
                    "basefont" => {
                        if let Some(s) = get("size").and_then(|v| v.trim().parse::<u8>().ok()) {
                            st.size = s.clamp(1, 7);
                        }
                        if let Some(c) = get("color").and_then(parse_color) {
                            st.color = c;
                        }
                    }
                    _ => continue, // unknown tag: ignored
                }
                stack.push((name, prev));
            }
            Tok::Close(name) => {
                if let Some(pos) = stack.iter().rposition(|(n, _)| *n == name) {
                    let (_, prev) = stack.remove(pos);
                    let is_block = matches!(name.as_str(), "center" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6");
                    if is_block {
                        break_block(&mut cur, &mut lines, prev.align);
                    }
                    st = prev;
                }
            }
        }
    }
    if !cur.pieces.is_empty() {
        let l = std::mem::replace(&mut cur, Line { pieces: Vec::new(), width: 0, height: 0, align: Align::Left });
        lines.push(Line { height: l.height.max(line_h), ..l });
    }
    if lines.is_empty() {
        return None;
    }
    let total_h: i32 = lines.iter().map(|l| l.height).sum();
    let (ow, oh) = (width as usize, total_h.max(1) as usize);
    let mut out = vec![0u8; ow * oh * 4];
    let mut y0 = 0i32;
    for l in &lines {
        let base_x = match l.align {
            Align::Left => 0,
            Align::Center => ((width - l.width) / 2).max(0),
            Align::Right => (width - l.width).max(0),
        };
        for (x, p) in &l.pieces {
            let oy = y0 + (l.height - p.h as i32 - 2).max(0);
            for py in 0..p.h {
                for px in 0..p.w {
                    let s = (py * p.w + px) * 4;
                    if p.rgba[s + 3] == 0 {
                        continue;
                    }
                    let (dx, dy) = (base_x + x + px as i32, oy + py as i32);
                    if dx >= 0 && (dx as usize) < ow && dy >= 0 && (dy as usize) < oh {
                        let d = (dy as usize * ow + dx as usize) * 4;
                        out[d..d + 4].copy_from_slice(&p.rgba[s..s + 4]);
                    }
                }
            }
        }
        y0 += l.height;
    }
    Some((ow, oh, out))
}

/// The text of `html` without any tag (entities decoded).
pub fn strip_html_tags(html: &str) -> String {
    tokenize(html)
        .into_iter()
        .filter_map(|t| if let Tok::Text(s) = t { Some(decode_entities(&s)) } else { None })
        .collect()
}

/// Wraps the characters `start..end` of `text` with `open` and `close` and returns the new text and
/// the character range of the wrapped content. With an empty selection the tags are inserted
/// around the cursor.
pub fn wrap_selection(text: &str, start: usize, end: usize, open: &str, close: &str) -> (String, (usize, usize)) {
    let (a, b) = (start.min(end), start.max(end));
    let chars: Vec<char> = text.chars().collect();
    let (a, b) = (a.min(chars.len()), b.min(chars.len()));
    let mut out: String = chars[..a].iter().collect();
    out.push_str(open);
    out.extend(chars[a..b].iter());
    out.push_str(close);
    out.extend(chars[b..].iter());
    let o = open.chars().count();
    (out, (a + o, b + o))
}

/// Converts HTML to the form Sphere scripts need: inside a script line the `<` and `>` characters
/// belong to Sphere's own `<expression>` syntax, so the tags are written with the `def_html`
/// defnames (`<b>` -> `<DEF.B>`, `</b>` -> `<DEF.BE>`) and literal angle brackets with
/// `<DEF.L1>` / `<DEF.R1>`.
pub fn html_to_sphere_defs(html: &str) -> String {
    let mut out = String::new();
    for tok in tokenize(html) {
        match tok {
            Tok::Text(t) => {
                for ch in decode_entities(&t).chars() {
                    match ch {
                        '<' => out.push_str("<DEF.L1>"),
                        '>' => out.push_str("<DEF.R1>"),
                        c => out.push(c),
                    }
                }
            }
            Tok::Open(name, attrs) => {
                let get = |k: &str| attrs.iter().find(|(a, _)| a == k).map(|(_, v)| v.clone());
                let simple = |n: &str| format!("<DEF.{}>", n);
                let s = match name.as_str() {
                    "b" => simple("B"),
                    "big" => simple("BIG"),
                    "small" => simple("SMALL"),
                    "em" => simple("EM"),
                    "u" => simple("U"),
                    "i" => simple("I"),
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => simple(&name.to_uppercase()),
                    "br" => simple("BR"),
                    "center" => simple("CENTER"),
                    "div" => match get("align").map(|v| v.to_lowercase()).as_deref() {
                        Some("right") => simple("DIV_RIGHT"),
                        _ => simple("DIV_LEFT"),
                    },
                    "a" => format!("<DEF.A>{}>", attrs.iter().map(|(k, v)| format!(" {}=\"{}\"", k, v)).collect::<String>()),
                    "basefont" => {
                        let size = get("size").filter(|v| matches!(v.as_str(), "1" | "2" | "3" | "4" | "5" | "7"));
                        match (size, get("color")) {
                            (Some(sz), None) => simple(&format!("BFONT_SIZE{}", sz)),
                            _ => format!("<DEF.BFONT>{}>", attrs.iter().map(|(k, v)| format!(" {}=\"{}\"", k, v)).collect::<String>()),
                        }
                    }
                    other => format!("<DEF.L1>{}{}<DEF.R1>", other, attrs.iter().map(|(k, v)| format!(" {}=\"{}\"", k, v)).collect::<String>()),
                };
                out.push_str(&s);
            }
            Tok::Close(name) => {
                let s = match name.as_str() {
                    "b" => "<DEF.BE>".to_string(),
                    "big" => "<DEF.BIGE>".to_string(),
                    "small" => "<DEF.SMALLE>".to_string(),
                    "em" => "<DEF.EME>".to_string(),
                    "u" => "<DEF.UE>".to_string(),
                    "i" => "<DEF.IE>".to_string(),
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => format!("<DEF.{}E>", name.to_uppercase()),
                    "center" => "<DEF.CENTERE>".to_string(),
                    "div" => "<DEF.DIV_END>".to_string(),
                    "a" => "<DEF.AE>".to_string(),
                    "basefont" => "<DEF.BFONTE>".to_string(),
                    other => format!("<DEF.L1>/{}<DEF.R1>", other),
                };
                out.push_str(&s);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// font with solid 3x6 glyphs for a, b, c (y offset 4)
    fn font() -> UnicodeFont {
        let mut d = vec![0u8; 0x40000];
        let mut put = |ch: char| {
            let off = d.len() as u32;
            d.extend_from_slice(&[0, 4, 3, 6]);
            d.extend(std::iter::repeat(0b1110_0000u8).take(6));
            let i = ch as usize * 4;
            d[i..i + 4].copy_from_slice(&off.to_le_bytes());
        };
        for c in ['a', 'b', 'c'] {
            put(c);
        }
        UnicodeFont::from_bytes(d).unwrap()
    }

    const W: [u8; 4] = [255, 255, 255, 255];
    fn first_col(img: &(usize, usize, Vec<u8>)) -> usize {
        (0..img.0).find(|&x| (0..img.1).any(|y| img.2[(y * img.0 + x) * 4 + 3] > 0)).unwrap()
    }
    fn count(img: &(usize, usize, Vec<u8>)) -> usize {
        img.2.chunks_exact(4).filter(|p| p[3] > 0).count()
    }

    #[test]
    fn tokenizer_keeps_non_tags_as_text() {
        let t = tokenize("a < b <b x=\"1\">c</b>");
        assert_eq!(t[0], Tok::Text("a < b ".into()));
        assert!(matches!(&t[1], Tok::Open(n, a) if n == "b" && a == &vec![("x".to_string(), "1".to_string())]));
        assert_eq!(t[3], Tok::Close("b".into()));
    }

    #[test]
    fn center_moves_text_to_the_middle() {
        let f = font();
        let left = render_html(&f, "abc", W, 100).unwrap();
        let centered = render_html(&f, "<center>abc</center>", W, 100).unwrap();
        let right = render_html(&f, "<div align=\"right\">abc</div>", W, 100).unwrap();
        assert_eq!(first_col(&left), 0);
        assert!(first_col(&centered) > 20 && first_col(&centered) < first_col(&right));
    }

    #[test]
    fn bold_and_br_change_the_result() {
        let f = font();
        let plain = render_html(&f, "abc", W, 100).unwrap();
        let bold = render_html(&f, "<b>abc</b>", W, 100).unwrap();
        assert!(count(&bold) > count(&plain));
        let two = render_html(&f, "ab<br>cb", W, 100).unwrap();
        assert!(two.1 > plain.1);
    }

    #[test]
    fn unknown_tags_are_ignored_and_colour_applies() {
        let f = font();
        let a = render_html(&f, "<foo>abc</foo>", W, 100).unwrap();
        let b = render_html(&f, "abc", W, 100).unwrap();
        assert_eq!(a, b);
        let red = render_html(&f, "<basefont color=\"#ff0000\">abc</basefont>", W, 100).unwrap();
        let px = red.2.chunks_exact(4).find(|p| p[3] > 0).unwrap();
        assert_eq!(&px[..3], &[255, 0, 0]);
    }

    #[test]
    fn size_tags_scale_the_text() {
        let f = font();
        let normal = render_html(&f, "abc", W, 200).unwrap();
        let big = render_html(&f, "<basefont size=\"7\">abc</basefont>", W, 200).unwrap();
        assert!(count(&big) > count(&normal) * 2);
    }

    #[test]
    fn wrap_selection_inserts_tags_around_characters() {
        let (t, sel) = wrap_selection("héllo world", 0, 5, "<b>", "</b>");
        assert_eq!(t, "<b>héllo</b> world");
        assert_eq!(sel, (3, 8));
        let (t2, _) = wrap_selection("ab", 1, 1, "<i>", "</i>");
        assert_eq!(t2, "a<i></i>b");
    }

    #[test]
    fn strips_tags() {
        assert_eq!(strip_html_tags("<b>a</b> &lt;x&gt; <br>c"), "a <x> c");
    }

    #[test]
    fn sphere_defs_conversion() {
        assert_eq!(html_to_sphere_defs("<b>Hi</b> <center>x</center>"), "<DEF.B>Hi<DEF.BE> <DEF.CENTER>x<DEF.CENTERE>");
        assert_eq!(html_to_sphere_defs("<a href=\"u\">l</a>"), "<DEF.A> href=\"u\">l<DEF.AE>");
        assert_eq!(html_to_sphere_defs("<basefont size=\"4\">t</basefont>"), "<DEF.BFONT_SIZE4>t<DEF.BFONTE>");
        assert_eq!(html_to_sphere_defs("<basefont size=\"6\">"), "<DEF.BFONT> size=\"6\">");
        assert_eq!(html_to_sphere_defs("1 < 2"), "1 <DEF.L1> 2");
        assert_eq!(html_to_sphere_defs("<div align=\"right\">r</div>"), "<DEF.DIV_RIGHT>r<DEF.DIV_END>");
    }
}
