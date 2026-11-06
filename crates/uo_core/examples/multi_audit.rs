//! Determines the multi.mul record size per file and finds multis the per-entry heuristic gets wrong.
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let mi = FileIndex::from_mul_idx(fm.get_file_path("multi.idx").unwrap(), fm.get_file_path("multi.mul").unwrap(), 0x4000).unwrap();
    let (mut n, mut only12, mut only16, mut both, mut neither) = (0, 0, 0, 0, 0);
    let mut ambiguous = Vec::new();
    for id in 0..0x4000usize {
        if !mi.is_valid(id) { continue; }
        let d = mi.get_data(id).unwrap();
        n += 1;
        match (d.len() % 12 == 0, d.len() % 16 == 0) {
            (true, false) => only12 += 1,
            (false, true) => only16 += 1,
            (true, true) => { both += 1; if ambiguous.len() < 12 { ambiguous.push((id, d.len())); } }
            _ => neither += 1,
        }
    }
    println!("valid multis: {n}  len%12 only: {only12}  len%16 only: {only16}  both: {both}  neither: {neither}");
    println!("ambiguous sample (id,len): {:?}", ambiguous);
    let rs = detect_multi_record_size(&mi);
    println!("detected record size: {}", rs);
    for id in [60usize] {
        if let Some(d) = mi.get_data(id) {
            println!("multi {} len {} -> %12={} %16={}", id, d.len(), d.len() % 12, d.len() % 16);
            let s12 = MultiStructure::parse(id as u32, &d);
            println!("   heuristic parse -> {} comps", s12.map_or(0, |s| s.components.len()));
            let ok = MultiStructure::parse_sized(id as u32, &d, rs).unwrap();
            println!("   file-level parse -> {} comps, bounds x {}..{} y {}..{}", ok.components.len(), ok.min_x, ok.max_x, ok.min_y, ok.max_y);
        } else { println!("multi {} has no data", id); }
    }
}
