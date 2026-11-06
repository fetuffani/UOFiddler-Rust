//! Where does an equipment animation id live? (resolver / UOP / legacy files)
use std::collections::HashMap;
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let ids: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let cfg = [(1, "anim.idx", "anim.mul"), (2, "anim2.idx", "anim2.mul"), (3, "anim3.idx", "anim3.mul"), (4, "anim4.idx", "anim4.mul"), (5, "anim5.idx", "anim5.mul")];
    let mut idx: HashMap<u32, FileIndex> = HashMap::new();
    for (id, i, m) in cfg {
        if let (Some(ip), Some(mp)) = (fm.get_file_path(i), fm.get_file_path(m)) {
            if let Ok(fi) = FileIndex::from_mul_idx(ip, mp, 0x40000) { idx.insert(id, fi); }
        }
    }
    let rd = |n: &str| fm.get_file_path(n).and_then(|p| std::fs::read_to_string(p).ok());
    let res = ClassicAnimResolver::parse(rd("body.def").as_deref(), rd("bodyconv.def").as_deref());
    let mut uop = UopAnimationIndex::new();
    uop.load_directory(&dir);
    for body in ids {
        let r = res.resolve(body);
        let uop_actions: Vec<u32> = (0..80).filter(|&a| uop.is_action_defined(body, a)).collect();
        println!("body {}: resolver -> file {} body {} hue {}", body, r.file_type, r.body, r.hue);
        println!("   UOP actions defined: {:?}", uop_actions);
        for ft in 1..=5u32 {
            let n = (0..80u32).filter(|&a| idx.get(&ft).and_then(|fi| fi.get_data(ClassicAnimResolver::index(ft, if ft == r.file_type { r.body } else { body }, a, 0))).map_or(false, |d| !decode_animation_mul(&d, false).is_empty())).count();
            println!("   anim file {}: {} actions decode (formula for that file)", ft, n);
        }
    }
}
