//! Color conversion and Hue processing for Ultima Online 15-bit (1555) / 16-bit colors.

pub const COLOR_5_TO_8: [u8; 32] = [
    0, 8, 16, 24, 32, 41, 49, 57, 65, 74, 82, 90, 98, 106, 115, 123, 131, 139, 148, 156, 164, 172,
    180, 189, 197, 205, 213, 222, 230, 238, 246, 255,
];

pub const COLOR_8_TO_5: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3,
    3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7,
    7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 11,
    11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16,
    17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 19, 19, 19,
    19, 20, 20, 20, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22,
    22, 22, 23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 25, 25, 25, 25, 25, 25,
    25, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27, 28, 28, 28, 28, 28,
    28, 28, 28, 29, 29, 29, 29, 29, 29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30, 31,
];

#[inline(always)]
pub fn color16_to_rgba(c: u16) -> [u8; 4] {
    if c == 0 {
        return [0, 0, 0, 0];
    }
    let r = COLOR_5_TO_8[((c >> 10) & 0x1F) as usize];
    let g = COLOR_5_TO_8[((c >> 5) & 0x1F) as usize];
    let b = COLOR_5_TO_8[(c & 0x1F) as usize];
    [r, g, b, 255]
}

#[inline(always)]
pub fn color16_to_u32(c: u16) -> u32 {
    if c == 0 {
        return 0;
    }
    let r = COLOR_5_TO_8[((c >> 10) & 0x1F) as usize] as u32;
    let g = COLOR_5_TO_8[((c >> 5) & 0x1F) as usize] as u32;
    let b = COLOR_5_TO_8[(c & 0x1F) as usize] as u32;
    0xFF00_0000 | (b << 16) | (g << 8) | r
}

#[inline(always)]
pub fn rgba_to_color16(r: u8, g: u8, b: u8, a: u8) -> u16 {
    if a < 128 {
        return 0;
    }
    let r5 = COLOR_8_TO_5[r as usize] as u16;
    let g5 = COLOR_8_TO_5[g as usize] as u16;
    let b5 = COLOR_8_TO_5[b as usize] as u16;
    let mut c = (r5 << 10) | (g5 << 5) | b5;
    if c == 0 {
        c = 1;
    }
    c | 0x8000
}

/// Applies a 32-color hue palette to an RGBA pixel buffer in-place.
pub fn apply_hue(pixels: &mut [[u8; 4]], palette: &[u32; 32], only_gray: bool) {
    for px in pixels.iter_mut() {
        let a = px[3];
        if a == 0 {
            continue;
        }
        let r = px[0];
        let g = px[1];
        let b = px[2];
        if r == 0 && g == 0 && b == 0 {
            continue;
        }

        if !only_gray || (r == g && g == b) {
            let idx = COLOR_8_TO_5[r as usize] as usize;
            let h = palette[idx];
            px[0] = (h & 0xFF) as u8;
            px[1] = ((h >> 8) & 0xFF) as u8;
            px[2] = ((h >> 16) & 0xFF) as u8;
            px[3] = a;
        }
    }
}

pub fn apply_hue_bytes(rgba: &mut [u8], palette: &[u32; 32], only_gray: bool) {
    for px in rgba.chunks_exact_mut(4) {
        let a = px[3];
        if a == 0 {
            continue;
        }
        let r = px[0];
        let g = px[1];
        let b = px[2];
        if r == 0 && g == 0 && b == 0 {
            continue;
        }

        if !only_gray || (r == g && g == b) {
            let idx = COLOR_8_TO_5[r as usize] as usize;
            let h = palette[idx];
            px[0] = (h & 0xFF) as u8;
            px[1] = ((h >> 8) & 0xFF) as u8;
            px[2] = ((h >> 16) & 0xFF) as u8;
            px[3] = a;
        }
    }
}
