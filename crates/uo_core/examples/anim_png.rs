//! Renders the first frames of a body/action/direction (UOP-first creature loader) to /out/anim_<body>_<action>.png
use std::collections::HashMap;
use uo_core::*;
fn main() {
    let a: Vec<u32> = std::env::args().skip(1).filter_map(|x| x.parse().ok()).collect();
    let (body, action, dir) = (a[0], a[1], a.get(2).copied().unwrap_or(3) as u8);
    let d = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&d);
    let cfg = [(1, "anim.idx", "anim.mul"), (2, "anim2.idx", "anim2.mul"), (3, "anim3.idx", "anim3.mul"), (4, "anim4.idx", "anim4.mul"), (5, "anim5.idx", "anim5.mul")];
    let mut idx: HashMap<u32, FileIndex> = HashMap::new();
    for (id, i, m) in cfg { if let (Some(ip), Some(mp)) = (fm.get_file_path(i), fm.get_file_path(m)) { if let Ok(fi) = FileIndex::from_mul_idx(ip, mp, 0x40000) { idx.insert(id, fi); } } }
    let rd = |n: &str| fm.get_file_path(n).and_then(|p| std::fs::read_to_string(p).ok());
    let res = ClassicAnimResolver::parse(rd("body.def").as_deref(), rd("bodyconv.def").as_deref());
    let mut uop = UopAnimationIndex::new();
    uop.load_directory(&d);
    let Some(frames) = load_animation_frames(&res, &idx, &uop, body, action, dir) else { println!("no frames"); return };
    println!("body {} action {} dir {}: {} frames, first {}x{} center ({}, {})", body, action, dir, frames.len(), frames[0].width, frames[0].height, frames[0].center_x, frames[0].center_y);
    let n = frames.len().min(4);
    let (cw, ch) = (frames.iter().take(n).map(|f| f.width as u32).max().unwrap(), frames.iter().take(n).map(|f| f.height as u32).max().unwrap());
    let mut img = image::RgbaImage::from_pixel(cw * n as u32 + 4, ch + 4, image::Rgba([40, 40, 60, 255]));
    for (k, f) in frames.iter().take(n).enumerate() {
        for y in 0..f.height as u32 { for x in 0..f.width as u32 {
            let s = ((y * f.width as u32 + x) * 4) as usize;
            if f.rgba_data[s + 3] > 0 { img.put_pixel(k as u32 * cw + x + 2, y + 2, image::Rgba([f.rgba_data[s], f.rgba_data[s+1], f.rgba_data[s+2], 255])); }
        }}
    }
    img.save(format!("/out/anim_{}_{}.png", body, action)).unwrap();
}
