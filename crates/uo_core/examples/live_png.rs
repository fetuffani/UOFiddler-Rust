//! Renders regions of the world with the live isometric renderer to PNG for visual checks.
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let art = FileIndex::from_mul_idx(fm.get_file_path("artidx.mul").unwrap(), fm.get_file_path("art.mul").unwrap(), 0x14000).unwrap();
    let tex = FileIndex::from_mul_idx(fm.get_file_path("texidx.mul").unwrap_or_default(), fm.get_file_path("texmaps.mul").unwrap(), 0x4000).ok();
    let td = TileDataManager::from_file(fm.get_file_path("tiledata.mul").unwrap()).ok();
    let mut mm = MapManager::new();
    for m in 0..6 {
        mm.load_map(m, fm.get_file_path(&format!("map{}.mul", m)), fm.get_file_path(&format!("staidx{}.mul", m)), fm.get_file_path(&format!("statics{}.mul", m)));
        mm.load_diff(m, fm.get_file_path(&format!("mapdif{}.mul", m)), fm.get_file_path(&format!("mapdifl{}.mul", m)),
            fm.get_file_path(&format!("stadif{}.mul", m)), fm.get_file_path(&format!("stadifi{}.mul", m)), fm.get_file_path(&format!("stadifl{}.mul", m)));
        println!("map{} present={} diff blocks (land,static)={:?}", m, mm.has_map(m), mm.diff_counts(m));
    }
    let ctx = LiveContext { map: &mm, art: &art, textures: tex.as_ref(), tiledata: td.as_ref() };
    let mut r = LiveRenderer::default();
    for (name, map, x, y) in [("britain", 0, 1496, 1624), ("trinsic", 0, 1900, 2750), ("malas_luna", 3, 1000, 530)] {
        let t = std::time::Instant::now();
        let buf = r.render(&ctx, map, x, y, 1200, 800, LiveOptions::default());
        println!("{}: rendered in {:.0} ms", name, t.elapsed().as_secs_f32() * 1000.0);
        image::RgbaImage::from_raw(1200, 800, buf).unwrap().save(format!("/out/live_{}.png", name)).unwrap();
    }
}
