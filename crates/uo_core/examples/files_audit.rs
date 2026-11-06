//! Classifies every client file and exercises the IFF and UOP readers on the real data.
use std::collections::BTreeMap;
use std::io::Read;
use uo_core::*;

fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let files = fm.list_files();
    let mut kinds: BTreeMap<ClientFileKind, usize> = BTreeMap::new();
    let (mut iff_ok, mut iff_bad, mut iff_strings, mut wide) = (0, Vec::new(), 0, 0);
    for (name, path, _) in &files {
        let mut head = vec![0u8; 16];
        let n = std::fs::File::open(path).and_then(|mut f| f.read(&mut head)).unwrap_or(0);
        head.truncate(n);
        let k = classify_file(name, Some(&head));
        *kinds.entry(k).or_default() += 1;
        match k {
            ClientFileKind::Iff => {
                let d = std::fs::read(path).unwrap();
                match parse_iff_text(&d) {
                    Some(t) if t.name.len() > 0 => { iff_ok += 1; iff_strings += t.strings.len(); if t.strings.len() as u32 != t.declared_count { iff_bad.push(format!("{} count {} != {}", name, t.strings.len(), t.declared_count)); } if t.width == 2 { wide += 1; } }
                    Some(_) | None => iff_bad.push(name.clone()),
                }
            }
            ClientFileKind::Uop => {
                if let Some(mut u) = read_uop(path) {
                    resolve_uop_names(&mut u, name);
                    let named = u.entries.iter().filter(|e| e.name.is_some()).count();
                    let ok = u.entries.iter().take(50).filter(|e| read_uop_entry(path, e).is_some()).count();
                    println!("UOP {:<28} v{} entries {:>6} named {:>6} read-ok(first50) {}", name, u.version, u.entries.len(), named, ok);
                } else { println!("UOP {} -> unreadable", name); }
            }
            _ => {}
        }
    }
    println!("total files: {}", files.len());
    for (k, c) in &kinds { println!("  {:<20} {}", k.label(), c); }
    println!("IFF parsed: {} ({} strings, {} UTF-16 files)  problems: {:?}", iff_ok, iff_strings, wide, iff_bad);
}
