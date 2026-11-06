//! Lists the resizepic candidates found in the gump index and checks well-known ids.
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let gi = FileIndex::from_mul_idx(fm.get_file_path("gumpidx.mul").unwrap(), fm.get_file_path("gumpart.mul").unwrap(), 0xFFFF).unwrap();
    let size_of = |id: u32| -> Option<(u32, u32)> {
        let i = id as usize;
        gi.is_valid(i).then(|| (((gi.entries[i].extra >> 16) & 0xFFFF) as u32, (gi.entries[i].extra & 0xFFFF) as u32))
    };
    let dec = |i: u32| { let e = gi.entries[i as usize]; decode_gump(&gi.get_data(i as usize).unwrap(), ((e.extra >> 16) & 0xFFFF) as u32, (e.extra & 0xFFFF) as u32) };
    let found = find_resizepics(0xFFFF, size_of, |id| { let p: Vec<GumpBitmap> = (0..9).filter_map(|k| dec(id + k)).collect(); frame_tiles_seamlessly(&p) }, &ResizepicLists::builtin());
    println!("candidates: {}", found.len());
    for known in [83u32, 9200, 9250, 9300, 9350, 3000, 5054, 2600, 3500, 217, 229, 238, 260, 1001, 1029, 1400] {
        println!("  known {:>5}: {}", known, found.contains(&known));
    }
    println!("first 30: {:?}", &found[..found.len().min(30)]);
}
