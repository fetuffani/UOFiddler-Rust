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
