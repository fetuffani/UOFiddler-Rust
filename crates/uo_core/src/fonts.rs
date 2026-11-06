//! UO Font manager (fonts.mul): reads 10 ASCII fonts and renders bitmap text.

use crate::color::color16_to_rgba;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Glyph {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct AsciiFont {
    pub header: u8,
    pub height: usize,
    pub characters: Vec<Option<Glyph>>,
}

impl AsciiFont {
    pub fn render_text(&self, text: &str) -> Option<(usize, usize, Vec<u8>)> {
        let mut glyphs = Vec::new();
        for ch in text.chars() {
            let code = (ch as usize).wrapping_sub(0x20);
            if code < self.characters.len() {
                if let Some(g) = &self.characters[code] {
                    glyphs.push(g);
                }
            }
        }

        if glyphs.is_empty() {
            return None;
        }

        let total_w: usize = glyphs.iter().map(|g| g.width).sum();
        let max_h: usize = glyphs.iter().map(|g| g.height).max().unwrap_or(0);

        if total_w == 0 || max_h == 0 {
            return None;
        }

        let mut output = vec![0u8; total_w * max_h * 4];
        let mut cur_x = 0usize;

        for g in glyphs {
            let y_offset = max_h - g.height;
            for gy in 0..g.height {
                for gx in 0..g.width {
                    let src_idx = (gy * g.width + gx) * 4;
                    let dst_x = cur_x + gx;
                    let dst_y = y_offset + gy;
                    let dst_idx = (dst_y * total_w + dst_x) * 4;

                    if g.rgba[src_idx + 3] > 0 {
                        output[dst_idx..dst_idx + 4].copy_from_slice(&g.rgba[src_idx..src_idx + 4]);
                    }
                }
            }
            cur_x += g.width;
        }

        Some((total_w, max_h, output))
    }
}

pub struct FontsManager {
    pub ascii_fonts: Vec<Option<AsciiFont>>,
}

impl FontsManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let mut fonts = Vec::with_capacity(10);
        let mut offset = 0usize;

        for _ in 0..10 {
            if offset >= data.len() {
                fonts.push(None);
                continue;
            }

            let header = data[offset];
            offset += 1;

            let mut font_h = 0usize;
            let mut characters = vec![None; 224];

            for k in 0..224 {
                if offset + 3 > data.len() {
                    break;
                }
                let w = data[offset] as usize;
                let h = data[offset + 1] as usize;
                let _unk = data[offset + 2];
                offset += 3;

                if w == 0 || h == 0 {
                    continue;
                }

                if h > font_h && k < 96 {
                    font_h = h;
                }

                let px_bytes = w * h * 2;
                if offset + px_bytes > data.len() {
                    break;
                }

                let mut rgba = vec![0u8; w * h * 4];
                for i in 0..(w * h) {
                    let word_offset = offset + i * 2;
                    let color = u16::from_le_bytes([data[word_offset], data[word_offset + 1]]);
                    if color != 0 {
                        let c = color16_to_rgba(color ^ 0x8000);
                        rgba[i * 4..i * 4 + 4].copy_from_slice(&c);
                    }
                }
                offset += px_bytes;

                characters[k] = Some(Glyph {
                    width: w,
                    height: h,
                    rgba,
                });
            }

            fonts.push(Some(AsciiFont {
                header,
                height: font_h,
                characters,
            }));
        }

        Ok(Self { ascii_fonts: fonts })
    }
}

/// Unicode font (unifont*.mul): `0x10000` u32 offsets, then per glyph
/// `i8 x_offset, i8 y_offset, u8 width, u8 height` followed by `height` rows of
/// `ceil(width / 8)` bytes (1 bit per pixel, MSB first). `y_offset` is measured from the top of
/// the line, so the baseline of the Latin letters sits at about y = 14.
pub struct UnicodeFont {
    offsets: Vec<u32>,
    data: Vec<u8>,
    /// distance between two text lines
    pub line_height: i32,
}

struct Placed {
    x: i32,
    y: i32,
    x_off: i32,
    y_off: i32,
    w: usize,
    h: usize,
    bits_at: usize,
}

impl UnicodeFont {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut data = Vec::new();
        File::open(path)?.read_to_end(&mut data)?;
        Self::from_bytes(data).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid unifont file"))
    }

    pub fn from_bytes(data: Vec<u8>) -> Option<Self> {
        if data.len() < 0x40000 {
            return None;
        }
        let offsets: Vec<u32> = data[..0x40000].chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
        let mut f = Self { offsets, data, line_height: 18 };
        // line height from the Latin range: lowest pixel row + 2
        let bottom = (0x21u32..0x7F).filter_map(|c| f.header(c)).map(|(_, yo, _, h, _)| yo + h as i32).max().unwrap_or(16);
        f.line_height = bottom + 2;
        Some(f)
    }

    /// (x_off, y_off, width, height, offset of the bitmap) of a glyph.
    fn header(&self, ch: u32) -> Option<(i32, i32, u8, u8, usize)> {
        let o = *self.offsets.get(ch as usize)? as usize;
        if o == 0 || o == 0xFFFF_FFFF || o + 4 > self.data.len() {
            return None;
        }
        let (w, h) = (self.data[o + 2], self.data[o + 3]);
        let need = h as usize * (w as usize).div_ceil(8);
        if o + 4 + need > self.data.len() {
            return None;
        }
        Some((self.data[o] as i8 as i32, self.data[o + 1] as i8 as i32, w, h, o + 4))
    }

    fn advance(&self, ch: char) -> i32 {
        if ch == ' ' {
            return 8;
        }
        match self.header(ch as u32) {
            Some((xo, _, w, _, _)) => xo.max(0) + w as i32 + 1,
            None => 0,
        }
    }

    fn word_width(&self, word: &str) -> i32 {
        word.chars().map(|c| self.advance(c)).sum()
    }

    /// Places every glyph; wraps at `max_width` on spaces when given. Returns (placements, width, height).
    fn layout(&self, text: &str, max_width: Option<i32>) -> (Vec<Placed>, i32, i32) {
        let mut out = Vec::new();
        let (mut x, mut y, mut width) = (0i32, 0i32, 0i32);
        for (li, line) in text.split('\n').enumerate() {
            if li > 0 {
                y += self.line_height;
                x = 0;
            }
            let mut first = true;
            for word in line.split(' ') {
                let ww = self.word_width(word);
                if !first {
                    let space = 8;
                    if let Some(mw) = max_width {
                        if x + space + ww > mw && x > 0 {
                            y += self.line_height;
                            x = 0;
                        } else {
                            x += space;
                        }
                    } else {
                        x += space;
                    }
                }
                first = false;
                for ch in word.chars() {
                    if let Some((xo, yo, w, h, bits_at)) = self.header(ch as u32) {
                        out.push(Placed { x, y, x_off: xo, y_off: yo, w: w as usize, h: h as usize, bits_at });
                    }
                    x += self.advance(ch);
                    width = width.max(x);
                }
                width = width.max(x);
            }
        }
        (out, width, y + self.line_height)
    }

    pub fn measure(&self, text: &str, max_width: Option<i32>) -> (i32, i32) {
        let (_, w, h) = self.layout(text, max_width);
        (w, h)
    }

    /// Renders `text` in `color` (RGBA). Returns (width, height, pixels).
    pub fn render(&self, text: &str, color: [u8; 4], max_width: Option<i32>) -> Option<(usize, usize, Vec<u8>)> {
        let (placed, w, h) = self.layout(text, max_width);
        if w <= 0 || h <= 0 || placed.is_empty() {
            return None;
        }
        let (wu, hu) = (w as usize + 2, h as usize + 2);
        let mut rgba = vec![0u8; wu * hu * 4];
        for p in &placed {
            let stride = p.w.div_ceil(8);
            for gy in 0..p.h {
                for gx in 0..p.w {
                    if self.data[p.bits_at + gy * stride + gx / 8] & (0x80 >> (gx % 8)) == 0 {
                        continue;
                    }
                    let px = p.x + p.x_off.max(0) + gx as i32;
                    let py = p.y + p.y_off + gy as i32;
                    if px < 0 || py < 0 || px as usize >= wu || py as usize >= hu {
                        continue;
                    }
                    let d = (py as usize * wu + px as usize) * 4;
                    rgba[d..d + 4].copy_from_slice(&color);
                }
            }
        }
        Some((wu, hu, rgba))
    }
}

#[cfg(test)]
mod unicode_tests {
    use super::*;

    /// Builds a tiny font with 'A' (3x2, all pixels set) and 'B' (2x1).
    fn font() -> UnicodeFont {
        let mut d = vec![0u8; 0x40000];
        let a_off = d.len() as u32;
        d.extend_from_slice(&[0, 4, 3, 2, 0b1110_0000, 0b1110_0000]);
        let b_off = d.len() as u32;
        d.extend_from_slice(&[0, 5, 2, 1, 0b1100_0000]);
        d[65 * 4..65 * 4 + 4].copy_from_slice(&a_off.to_le_bytes());
        d[66 * 4..66 * 4 + 4].copy_from_slice(&b_off.to_le_bytes());
        UnicodeFont::from_bytes(d).unwrap()
    }

    #[test]
    fn renders_and_measures() {
        let f = font();
        let (w, h) = f.measure("AB", None);
        assert_eq!(w, (3 + 1) + (2 + 1));
        assert!(h >= f.line_height);
        let (rw, rh, px) = f.render("A", [255, 0, 0, 255], None).unwrap();
        assert!(rw >= 3 && rh >= 6);
        // glyph row 0 starts at y_offset 4
        let at = |x: usize, y: usize| px[(y * rw + x) * 4 + 3];
        assert_eq!(at(0, 4), 255);
        assert_eq!(at(0, 3), 0);
    }

    #[test]
    fn wraps_on_spaces() {
        let f = font();
        let (_, h1) = f.measure("A A A", None);
        let (_, h2) = f.measure("A A A", Some(9));
        assert!(h2 > h1, "narrow width must produce more lines");
    }
}
