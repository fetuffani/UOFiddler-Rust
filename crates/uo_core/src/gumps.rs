//! Gump art decoder for UI, menus, paperdolls, and game textures.

use crate::color::color16_to_rgba;

#[derive(Clone, Debug)]
pub struct GumpBitmap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

pub fn decode_gump(payload: &[u8], width: u32, height: u32) -> Option<GumpBitmap> {
    if width == 0 || height == 0 || width > 4096 || height > 4096 {
        return None;
    }

    let word_count = payload.len() / 2;
    if word_count < (height as usize) * 2 {
        return None;
    }

    let words: &[u16] =
        unsafe { std::slice::from_raw_parts(payload.as_ptr() as *const u16, word_count) };
    let lookups: &[u32] =
        unsafe { std::slice::from_raw_parts(payload.as_ptr() as *const u32, height as usize) };

    let w = width as usize;
    let h = height as usize;
    let mut rgba = vec![0u8; w * h * 4];

    for y in 0..h {
        let mut count = (lookups[y] as usize) * 2;
        let mut x = 0usize;
        let row_offset = y * w * 4;

        while x < w && count + 1 < word_count {
            let color = words[count];
            let run = words[count + 1] as usize;
            count += 2;

            if color == 0 {
                x += run;
            } else {
                let c = color16_to_rgba(color ^ 0x8000);
                for _ in 0..run {
                    if x < w {
                        let px_idx = row_offset + x * 4;
                        rgba[px_idx..px_idx + 4].copy_from_slice(&c);
                        x += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Some(GumpBitmap {
        width,
        height,
        rgba,
    })
}
