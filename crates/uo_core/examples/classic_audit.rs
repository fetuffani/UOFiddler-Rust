//! Compares the old GUI classic-animation lookup (try every animN with one formula set)
//! against the Body.def/Bodyconv.def aware resolver.
use std::collections::HashMap;
use uo_core::*;

fn old_index(body: u32, action: u32, dir: u8, ft: u32) -> usize {
    let base = if ft == 2 {
        if body < 200 { body * 110 } else { 22000 + (body - 200) * 65 }
    } else if ft == 3 {
        if body < 300 { body * 65 } else if body < 400 { 33000 + (body - 300) * 110 } else { 35000 + (body - 400) * 175 }
    } else if body < 200 { body * 110 } else if body < 400 { 22000 + (body - 200) * 65 } else { 35000 + (body - 400) * 175 };
    (base + action * 5 + dir as u32) as usize
}

fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
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
    println!("Body.def entries: {}  Bodyconv.def entries: {}", res.body_def.len(), res.body_conv.len());

    let (mut old_bodies, mut new_bodies, mut old_acts, mut new_acts) = (0, 0, 0, 0);
    let mut only_new = Vec::new();
    let mut only_old = Vec::new();
    for body in 0..2048u32 {
        let (mut o, mut n) = (0, 0);
        for action in 0..80u32 {
            let ok_old = (1..=5u32).any(|ft| idx.get(&ft).map_or(false, |fi| fi.get_data(old_index(body, action, 0, ft)).map_or(false, |d| !decode_animation_mul(&d, false).is_empty())));
            let r = res.resolve(body);
            let ok_new = idx.get(&r.file_type).map_or(false, |fi| fi.get_data(ClassicAnimResolver::index(r.file_type, r.body, action, 0)).map_or(false, |d| !decode_animation_mul(&d, false).is_empty()));
            o += ok_old as u32; n += ok_new as u32;
        }
        old_acts += o; new_acts += n;
        if o > 0 { old_bodies += 1 }
        if n > 0 { new_bodies += 1 }
        if n > 0 && o == 0 && only_new.len() < 30 { only_new.push(body) }
        if o > 0 && n == 0 && only_old.len() < 30 { only_old.push(body) }
    }
    println!("OLD: bodies with animation {} / actions {}", old_bodies, old_acts);
    println!("NEW: bodies with animation {} / actions {}", new_bodies, new_acts);
    println!("bodies only found by NEW: {:?}", only_new);
    println!("bodies only found by OLD: {:?}", only_old);
}
