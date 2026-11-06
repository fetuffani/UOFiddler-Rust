use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    let show = |ids: &[usize]| for &i in ids { if gi.is_valid(i) { let e = gi.entries[i]; println!("gump {} exists {}x{}", i, (e.extra >> 16) & 0xFFFF, e.extra & 0xFFFF); } };
    show(&(0..24).collect::<Vec<_>>());
    show(&(655..675).collect::<Vec<_>>());
    show(&[0x29A, 0x29B, 0x29C, 0x7D4, 0x7D5, 0x7D6, 0xC84, 0xC85]);
    println!("equip gumps in 50000..51000: {}  60000..61000: {}", (50000..51000).filter(|&i| gi.is_valid(i)).count(), (60000..61000).filter(|&i| gi.is_valid(i)).count());
    println!("max valid 50000+: {:?}", (50000..0x10000).rev().find(|&i| gi.is_valid(i)));
}
