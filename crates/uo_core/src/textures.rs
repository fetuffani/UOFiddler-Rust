//! Land texture decoder for texmaps.mul / texidx.mul (64x64 and 128x128 textures).

use crate::color::color16_to_rgba;

#[derive(Clone, Debug)]
pub struct TextureBitmap {
    pub size: u32,
    pub rgba: Vec<u8>,
}

pub fn decode_texture(data: &[u8], extra: i32) -> Option<TextureBitmap> {
    if data.is_empty() {
        return None;
    }

    let size = if extra != 0 || data.len() >= 32768 {
        128u32
    } else {
        64u32
    };

    let word_count = (size * size) as usize;
    if data.len() < word_count * 2 {
        return None;
    }

    let words: &[u16] =
        unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, word_count) };

    let mut rgba = vec![0u8; word_count * 4];
    for (i, &w) in words.iter().enumerate() {
        let c = color16_to_rgba(w | 0x8000);
        let off = i * 4;
        rgba[off..off + 4].copy_from_slice(&c);
    }

    Some(TextureBitmap { size, rgba })
}
