//! Renders a sprite sheet (rows = the 8 directions, columns = first frames) of an animation to a PNG.
//! usage: anim_sheet <creature|equip> <id> <action> <out.png>
use std::collections::HashMap;
use uo_core::*;
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (kind, id, action, out) = (a[0].as_str(), a[1].parse::<u32>().unwrap(), a[2].parse::<u32>().unwrap(), a[3].clone());
    let d = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&d);
    let cfg = [(1, "anim.idx", "anim.mul"), (2, "anim2.idx", "anim2.mul"), (3, "anim3.idx", "anim3.mul"), (4, "anim4.idx", "anim4.mul"), (5, "anim5.idx", "anim5.mul")];
    let mut idx: HashMap<u32, FileIndex> = HashMap::new();
    for (n, i, m) in cfg { if let (Some(ip), Some(mp)) = (fm.get_file_path(i), fm.get_file_path(m)) { if let Ok(fi) = FileIndex::from_mul_idx(ip, mp, 0x40000) { idx.insert(n, fi); } } }
    let rd = |n: &str| fm.get_file_path(n).and_then(|p| std::fs::read_to_string(p).ok());
    let res = ClassicAnimResolver::parse(rd("body.def").as_deref(), rd("bodyconv.def").as_deref());
    let mut uop = UopAnimationIndex::new();
    uop.load_directory(&d);
    let mut rows: Vec<Vec<AnimationFrame>> = Vec::new();
    for dir in 0..8u8 {
        let frames = if kind == "equip" { load_equipment_frames(&idx, &uop, false, id, action, dir) } else { load_animation_frames(&res, &idx, &uop, id, action, dir) };
        rows.push(frames.unwrap_or_default());
    }
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0).min(6);
    if cols == 0 { println!("no frames for {} {} action {}", kind, id, action); return; }
    // bounding box of every frame placed by its centre (left = -cx, top = -cy - h)
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for f in rows.iter().flatten().take(1000) {
        let (l, t) = (-(f.center_x as i32), -(f.center_y as i32) - f.height as i32);
        x0 = x0.min(l); y0 = y0.min(t); x1 = x1.max(l + f.width as i32); y1 = y1.max(t + f.height as i32);
    }
    let (cw, ch) = ((x1 - x0 + 8) as u32, (y1 - y0 + 8) as u32);
    let mut img = image::RgbaImage::from_pixel(cw * cols as u32, ch * 8, image::Rgba([36, 38, 52, 255]));
    for (r, frames) in rows.iter().enumerate() {
        for (c, f) in frames.iter().take(cols).enumerate() {
            let (l, t) = (-(f.center_x as i32) - x0 + 4, -(f.center_y as i32) - f.height as i32 - y0 + 4);
            for y in 0..f.height as i32 { for x in 0..f.width as i32 {
                let s = ((y * f.width as i32 + x) * 4) as usize;
                if f.rgba_data[s + 3] > 0 {
                    let (px, py) = ((c as u32 * cw) as i32 + l + x, (r as u32 * ch) as i32 + t + y);
                    img.put_pixel(px as u32, py as u32, image::Rgba([f.rgba_data[s], f.rgba_data[s + 1], f.rgba_data[s + 2], 255]));
                }
            }}
        }
    }
    let scaled = image::imageops::resize(&img, img.width() * 2, img.height() * 2, image::imageops::FilterType::Nearest);
    scaled.save(&out).unwrap();
    println!("{} {} action {}: {} columns x 8 directions, {}x{} px", kind, id, action, cols, scaled.width(), scaled.height());
}
