//! Multimap.rle / facetNN.mul: the client's 1-bit run-length encoded world overview maps.
//! Each data byte: bit 7 = pixel set (land), bits 0-6 = run length.
//! Multimap.rle has a 32-bit width/height header; facetNN.mul uses 16-bit values.

pub struct MultiMapImage {
    pub width: u32,
    pub height: u32,
    /// RGBA pixels
    pub rgba: Vec<u8>,
    /// number of pixels the stream described (may differ from width*height on odd files)
    pub decoded_pixels: usize,
}

pub fn decode_multimap(data: &[u8], header32: bool) -> Option<MultiMapImage> {
    let (w, h, start) = if header32 {
        if data.len() < 8 {
            return None;
        }
        (
            u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
            8,
        )
    } else {
        if data.len() < 4 {
            return None;
        }
        (u16::from_le_bytes([data[0], data[1]]) as u32, u16::from_le_bytes([data[2], data[3]]) as u32, 4)
    };
    if w == 0 || h == 0 || w > 16384 || h > 16384 {
        return None;
    }
    let total = (w as usize) * (h as usize);
    let mut rgba = vec![0u8; total * 4];
    for px in rgba.chunks_exact_mut(4) {
        px.copy_from_slice(&[10, 14, 30, 255]);
    }
    let mut pos = 0usize;
    for &b in &data[start..] {
        let run = (b & 0x7F) as usize;
        let on = b & 0x80 != 0;
        let end = (pos + run).min(total);
        if on {
            for px in rgba[pos * 4..end * 4].chunks_exact_mut(4) {
                px.copy_from_slice(&[210, 190, 130, 255]);
            }
        }
        pos += run;
        if pos >= total {
            break;
        }
    }
    Some(MultiMapImage { width: w, height: h, rgba, decoded_pixels: pos })
}

/// facetNN.mul: i16 width, i16 height, then `height` rows of `u32 byteLen` followed by
/// `byteLen / 3` records of (u8 run, u16 color A1R5G5B5-like). Every row must sum to `width`.
/// Returns None if the structure does not hold (strict validation).
pub fn decode_facet(data: &[u8]) -> Option<MultiMapImage> {
    if data.len() < 4 {
        return None;
    }
    let w = i16::from_le_bytes([data[0], data[1]]) as i32;
    let h = i16::from_le_bytes([data[2], data[3]]) as i32;
    if w <= 0 || h <= 0 {
        return None;
    }
    let (w, h) = (w as usize, h as usize);
    let mut rgba = vec![0u8; w * h * 4];
    let mut pos = 4usize;
    let expand = |v: u16| -> u8 { ((v << 3) | (v >> 2)) as u8 };
    for y in 0..h {
        let len = u32::from_le_bytes(data.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let row = data.get(pos..pos + len)?;
        pos += len;
        if len % 3 != 0 {
            return None;
        }
        let mut x = 0usize;
        for rec in row.chunks_exact(3) {
            let run = rec[0] as usize;
            let c = u16::from_le_bytes([rec[1], rec[2]]);
            let px = [expand((c >> 10) & 31), expand((c >> 5) & 31), expand(c & 31), 255];
            if x + run > w {
                return None;
            }
            for i in 0..run {
                let o = (y * w + x + i) * 4;
                rgba[o..o + 4].copy_from_slice(&px);
            }
            x += run;
        }
        if x != w {
            return None;
        }
    }
    if pos != data.len() {
        return None;
    }
    Some(MultiMapImage { width: w as u32, height: h as u32, rgba, decoded_pixels: w * h })
}
