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
