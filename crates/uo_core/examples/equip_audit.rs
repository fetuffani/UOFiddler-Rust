//! Checks animation->gump links against the real GumpArt index.
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    let rd = |n: &str| fm.get_file_path(n).and_then(|p| std::fs::read_to_string(p).ok());
    let m = EquipGumpMap::parse(rd("equipconv.def").as_deref(), rd("gump.def").as_deref());
    println!("Equipconv entries: {}  gump.def entries: {}", m.equip_conv.len(), m.gump_def.len());
    let exists = |g: u32| gi.is_valid(g as usize);
    let (mut male, mut fem, mut rev_ok) = (0, 0, 0);
    for anim in 0..1000u32 {
        let (gm, _) = m.gump_for_anim(400, anim, false, exists);
        let (gf, _) = m.gump_for_anim(401, anim, true, exists);
        if exists(gm) { male += 1; if m.anim_for_gump(gm).is_some() { rev_ok += 1 } }
        if exists(gf) { fem += 1 }
    }
    println!("anim ids 0..1000 with male gump: {}  female gump: {}  reverse ok: {}", male, fem, rev_ok);
    let (g, h) = m.gump_for_anim(401, 866, true, exists);
    println!("sample: body401 anim866 -> gump {} hue {}", g, h);
}
