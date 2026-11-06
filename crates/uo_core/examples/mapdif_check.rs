use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let mut mm = MapManager::new();
    for m in 0..3usize {
        mm.load_map(m, fm.get_file_path(&format!("map{}.mul", m)), fm.get_file_path(&format!("staidx{}.mul", m)), fm.get_file_path(&format!("statics{}.mul", m)));
        mm.load_diff(m, fm.get_file_path(&format!("mapdif{}.mul", m)), fm.get_file_path(&format!("mapdifl{}.mul", m)),
            fm.get_file_path(&format!("stadif{}.mul", m)), fm.get_file_path(&format!("stadifi{}.mul", m)), fm.get_file_path(&format!("stadifl{}.mul", m)));
        let (bw, bh) = (mm.actual_widths[m] / 8, MAP_DEFS[m].height / 8);
        let (mut land_diff, mut stat_diff) = (0, 0);
        for bx in 0..bw { for by in 0..bh {
            if mm.land_block(m, bx, by, true) != mm.land_block(m, bx, by, false) { land_diff += 1; }
            if mm.static_block(m, bx, by, true) != mm.static_block(m, bx, by, false) { stat_diff += 1; }
        }}
        println!("map{}: declared diff blocks {:?}; blocks whose land differs with mapdif on/off: {}, statics: {}", m, mm.diff_counts(m), land_diff, stat_diff);
    }
}
