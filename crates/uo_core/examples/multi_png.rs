use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let mi = FileIndex::from_mul_idx(fm.get_file_path("multi.idx").unwrap(), fm.get_file_path("multi.mul").unwrap(), 0x4000).unwrap();
    let art = FileIndex::from_mul_idx(fm.get_file_path("artidx.mul").unwrap(), fm.get_file_path("art.mul").unwrap(), 0x14000).unwrap();
    let rs = detect_multi_record_size(&mi);
    all(&mi, &art, rs);
    for id in [60usize, 16, 35] {
        let d = mi.get_data(id).unwrap();
        for (label, size) in [("old", if d.len() % 16 == 0 && d.len() % 12 != 0 { 16 } else { 12 }), ("new", rs)] {
            let s = MultiStructure::parse_sized(id as u32, &d, size).unwrap();
            let Some(bmp) = s.render_isometric(127, |item| art.get_data(0x4000 + item as usize).and_then(|a| decode_static(&a))) else {
                println!("multi {} {} -> {} comps, RENDER FAILED (None)", id, label, s.components.len());
                continue;
            };
            println!("multi {} {} -> {} comps, {}x{}", id, label, s.components.len(), bmp.width, bmp.height);
            image::RgbaImage::from_raw(bmp.width as u32, bmp.height as u32, bmp.rgba).unwrap().save(format!("/out/multi_{}_{}.png", id, label)).unwrap();
        }
    }
}

#[allow(dead_code)]
fn all(mi: &FileIndex, art: &FileIndex, rs: usize) {
    let (mut ok, mut bad) = (0, Vec::new());
    for id in 0..0x4000usize {
        if !mi.is_valid(id) { continue; }
        let d = mi.get_data(id).unwrap();
        let r = MultiStructure::parse_sized(id as u32, &d, rs)
            .and_then(|s| s.render_isometric(127, |i| art.get_data(0x4000 + i as usize).and_then(|a| decode_static(&a))));
        if r.is_some() { ok += 1 } else { bad.push(id) }
    }
    println!("ALL multis: render ok {} failed {} {:?}", ok, bad.len(), &bad[..bad.len().min(20)]);
}
