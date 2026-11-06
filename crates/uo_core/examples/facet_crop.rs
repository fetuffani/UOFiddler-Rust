use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let d = std::fs::read(std::path::Path::new(&dir).join("facet00.mul")).unwrap();
    let m = decode_multimap(&d, false).unwrap();
    let (cx, cy, cw, ch) = (1300u32, 1400u32, 500u32, 400u32);
    let mut out = image::RgbaImage::new(cw, ch);
    for y in 0..ch { for x in 0..cw {
        let i = (((cy + y) * m.width + cx + x) * 4) as usize;
        out.put_pixel(x, y, image::Rgba([m.rgba[i], m.rgba[i+1], m.rgba[i+2], 255]));
    }}
    out.save("/out/facet00_crop.png").unwrap();
}
