//! Real-data round trip: MUL -> UOP -> MUL, comparing every entry, plus loading the generated
//! UOP with the app's own FileIndex::from_uop reader.
use std::path::Path;
use uo_core::*;

fn idx_entries(p: &Path) -> Vec<(i32, i32, i32)> {
    std::fs::read(p).unwrap().chunks_exact(12).map(|r| (
        i32::from_le_bytes(r[0..4].try_into().unwrap()), i32::from_le_bytes(r[4..8].try_into().unwrap()), i32::from_le_bytes(r[8..12].try_into().unwrap()))).collect()
}

fn main() {
    let src = std::path::PathBuf::from(std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into()));
    let work = Path::new("/out/conv");
    let _ = std::fs::remove_dir_all(work);
    std::fs::create_dir_all(work).unwrap();
    for kind in [LegacyKind::Gump, LegacyKind::Art, LegacyKind::Map(2)] {
        let uop = work.join(kind.uop_name());
        let t = std::time::Instant::now();
        let r1 = mul_to_uop(kind, &src, &uop).unwrap();
        let back = work.join(format!("back_{}", kind.uop_name()));
        let r2 = uop_to_mul(kind, &uop, &back).unwrap();
        let (idx_name, mul_name) = kind.mul_names();
        let (a, b) = (std::fs::read(src.join(&mul_name)).unwrap(), std::fs::read(back.join(&mul_name)).unwrap());
        let mut mismatches = 0usize;
        let mut compared = 0usize;
        if let Some(idx) = idx_name {
            let oi = idx_entries(&src.join(&idx));
            let ni = idx_entries(&back.join(&idx));
            for (i, o) in oi.iter().enumerate() {
                if o.0 < 0 || o.1 <= 0 { continue; }
                compared += 1;
                let n = ni.get(i).copied().unwrap_or((-1, -1, -1));
                let same = n.1 == o.1 && n.2 == o.2 && n.0 >= 0
                    && a[o.0 as usize..(o.0 + o.1) as usize] == b[n.0 as usize..(n.0 + n.1) as usize];
                if !same { mismatches += 1; }
            }
        } else {
            compared = 1;
            if a != b { mismatches = 1; }
        }
        // does the app's own UOP loader read the generated file?
        let ext = if matches!(kind, LegacyKind::Map(_)) { ".dat" } else { ".tga" };
        let loaded = FileIndex::from_uop(&uop, ext, 0x14000).map(|f| (0..0x14000).filter(|&i| f.is_valid(i)).count());
        println!("{:<22} uop entries {:>6} -> mul entries {:>6} | compared {:>6} mismatches {} | FileIndex::from_uop valid: {:?} | {:.1}s",
            kind.uop_name(), r1.entries, r2.entries, compared, mismatches, loaded, t.elapsed().as_secs_f32());
    }
}
