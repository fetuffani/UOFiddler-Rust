//! Which equipment ids have data in anim.mul (file 1)? Prints the valid ranges and details for given ids.
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let fi = FileIndex::from_mul_idx(fm.get_file_path("anim.idx").unwrap(), fm.get_file_path("anim.mul").unwrap(), 0x40000).unwrap();
    let valid_entries = (0..0x40000usize).filter(|&i| fi.is_valid(i)).count();
    println!("anim.mul valid index entries: {}", valid_entries);
    let has = |body: u32| (0..80u32).any(|a| (0..5u8).any(|d| fi.is_valid(ClassicAnimResolver::index(1, body, a, d))));
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    for b in 400..1500u32 {
        if has(b) { match ranges.last_mut() { Some(r) if r.1 + 1 == b => r.1 = b, _ => ranges.push((b, b)) } }
    }
    println!("equipment bodies with index entries (400..1500): {:?}", ranges);
    for body in std::env::args().skip(1).filter_map(|a| a.parse::<u32>().ok()) {
        let n_valid = (0..80u32).filter(|&a| fi.is_valid(ClassicAnimResolver::index(1, body, a, 0))).count();
        let n_dec = (0..80u32).filter(|&a| fi.get_data(ClassicAnimResolver::index(1, body, a, 0)).map_or(false, |d| !decode_animation_mul(&d, false).is_empty())).count();
        println!("body {}: valid actions {} decodable {}  (index of action 0 = {})", body, n_valid, n_dec, ClassicAnimResolver::index(1, body, 0, 0));
    }
}
