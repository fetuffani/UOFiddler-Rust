//! Renders gumps by id to /out/gump_<id>.png (visual checks).
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    for a in std::env::args().skip(1) {
        let id: usize = a.parse().unwrap();
        let (data, e) = (gi.get_data(id).unwrap(), gi.entries[id]);
        let b = decode_gump(&data, ((e.extra >> 16) & 0xFFFF) as u32, (e.extra & 0xFFFF) as u32).unwrap();
        let mut img = image::RgbaImage::from_raw(b.width, b.height, b.rgba).unwrap();
        for p in img.pixels_mut() { if p[3] == 0 { *p = image::Rgba([40, 40, 60, 255]); } }
        img.save(format!("/out/gump_{id}.png")).unwrap();
    }
}
