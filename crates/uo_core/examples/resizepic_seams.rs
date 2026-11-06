use uo_core::*;
fn seam(b: &GumpBitmap, horizontal: bool) -> f32 {
    let (w, h) = (b.width as usize, b.height as usize);
    let px = |x: usize, y: usize| &b.rgba[(y * w + x) * 4..(y * w + x) * 4 + 4];
    let n = if horizontal { h } else { w };
    let mut total = 0.0;
    for i in 0..n {
        let (a, c) = if horizontal { (px(0, i), px(w - 1, i)) } else { (px(i, 0), px(i, h - 1)) };
        total += (0..4).map(|k| (a[k] as f32 - c[k] as f32).abs()).sum::<f32>() / 4.0;
    }
    total / n as f32
}
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    let dec = |i: u32| { let e = gi.entries[i as usize]; decode_gump(&gi.get_data(i as usize).unwrap(), ((e.extra >> 16) & 0xFFFF) as u32, (e.extra & 0xFFFF) as u32).unwrap() };
    for id in std::env::args().skip(1).filter_map(|a| a.parse::<u32>().ok()) {
        let (t, b, l, r, c) = (dec(id + 1), dec(id + 7), dec(id + 3), dec(id + 5), dec(id + 4));
        let worst_edges = seam(&t, true).max(seam(&b, true)).max(seam(&l, false)).max(seam(&r, false));
        let worst_center = seam(&c, true).max(seam(&c, false));
        let opaque = |b: &GumpBitmap| b.rgba.chunks_exact(4).filter(|p| p[3] > 0).count() as f32 / (b.rgba.len() / 4) as f32;
        let lum = |b: &GumpBitmap| b.rgba.chunks_exact(4).filter(|p| p[3] > 0).map(|p| (p[0] as f32 + p[1] as f32 + p[2] as f32) / 3.0).sum::<f32>() / b.rgba.chunks_exact(4).filter(|p| p[3] > 0).count().max(1) as f32;
        let (tl, c) = (dec(id), dec(id + 4));
        println!("{:>5}: edges seam {:>6.1} | centre seam {:>6.1} | opaque corner {:.2} centre {:.2} | lum corner {:.0} centre {:.0} | corner {}x{}", id, worst_edges, worst_center, opaque(&tl), opaque(&c), lum(&tl), lum(&c), tl.width, tl.height);
    }
}
