//! UO Light levels & patterns decoder (light.mul / lightidx.mul).

#[derive(Clone, Debug)]
pub struct LightBitmap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

pub fn decode_light(data: &[u8], extra: i32) -> Option<LightBitmap> {
    let width = (extra & 0xFFFF) as u32;
    let height = ((extra >> 16) & 0xFFFF) as u32;

    if width == 0 || height == 0 || width > 1024 || height > 1024 {
        return None;
    }

    let total_pixels = (width * height) as usize;
    if data.len() < total_pixels {
        return None;
    }

    let mut rgba = vec![0u8; total_pixels * 4];
    for i in 0..total_pixels {
        let val = data[i] as u32;
        let intensity = (val * 8).min(255) as u8;
        let idx = i * 4;
        rgba[idx] = intensity;
        rgba[idx + 1] = intensity;
        rgba[idx + 2] = intensity / 2;
        rgba[idx + 3] = 255;
    }

    Some(LightBitmap {
        width,
        height,
        rgba,
    })
}
