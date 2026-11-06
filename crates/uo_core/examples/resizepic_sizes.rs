use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    let sz = |i: u32| (((gi.entries[i as usize].extra >> 16) & 0xFFFF) as u32, (gi.entries[i as usize].extra & 0xFFFF) as u32);
    for id in std::env::args().skip(1).filter_map(|a| a.parse::<u32>().ok()) {
        let s: Vec<(u32, u32)> = (0..9).map(|k| sz(id + k)).collect();
        println!("{:>5}: tl{:?} t{:?} tr{:?} | l{:?} c{:?} r{:?} | bl{:?} b{:?} br{:?}", id, s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8]);
    }
}
