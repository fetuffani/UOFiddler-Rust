//! Audits AnimationFrame*.uop coverage: which bodies/actions exist in the UOP files
//! versus the bodies listed by Animationlist.xml + mobtypes.txt.
use std::collections::{BTreeMap, BTreeSet};
use uo_core::*;

fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut idx = UopAnimationIndex::new();
    idx.load_directory(&dir);
    println!("UOP entries indexed: {}", idx.entries.len());

    let mut found: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for body in 0..4096u32 {
        for action in 0..80u32 {
            let h = hash_file_name(&format!("build/animationlegacyframe/{:06}/{:02}.bin", body, action));
            if idx.entries.contains_key(&h) {
                found.entry(body).or_default().insert(action);
            }
        }
    }
    let matched: usize = found.values().map(|s| s.len()).sum();
    println!("entries matched by body/action enumeration: {} ({} bodies)", matched, found.len());

    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let defs = load_animation_defs(&fm);
    let listed: BTreeSet<u32> = defs.iter().map(|d| d.body).collect();
    println!("bodies listed by xml+mobtypes: {}", listed.len());
    let missing: Vec<u32> = found.keys().copied().filter(|b| !listed.contains(b)).collect();
    println!("UOP bodies NOT listed in the viewer: {}", missing.len());
    println!("first 40: {:?}", &missing[..missing.len().min(40)]);
    println!("max body in uop: {:?}", found.keys().max());
    decode_audit(&idx, &found);
}

#[allow(dead_code)]
fn decode_audit(idx: &UopAnimationIndex, found: &BTreeMap<u32, BTreeSet<u32>>) {
    let (mut ok, mut empty) = (0usize, 0usize);
    let mut bad: Vec<(u32, u32)> = Vec::new();
    for (body, actions) in found {
        for a in actions {
            let mut any = false;
            for d in 0..5u8 {
                if idx.get_animation_frames(*body, *a, d).is_some() { any = true; break; }
            }
            if any { ok += 1 } else { empty += 1; if bad.len() < 40 { bad.push((*body, *a)); } }
        }
    }
    println!("decode ok: {}  empty/failed: {}", ok, empty);
    println!("sample failures (body,action): {:?}", bad);
}
