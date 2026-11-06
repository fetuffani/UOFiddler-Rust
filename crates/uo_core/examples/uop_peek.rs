use std::path::Path;
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    for name in ["MainMisc.uop", "tileart.uop", "string_dictionary.uop"] {
        let p = Path::new(&dir).join(name);
        let u = read_uop(&p).unwrap();
        println!("== {} ({} entries)", name, u.entries.len());
        for e in u.entries.iter().take(if name == "tileart.uop" { 2 } else { 4 }) {
            let d = read_uop_entry(&p, e).unwrap();
            println!("hash {:016x} comp {} len {} -> {}", e.hash, e.compressed, d.len(), d.iter().take(48).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
            let s: String = d.iter().take(200).map(|&b| if (32..127).contains(&b) { b as char } else { '.' }).collect();
            println!("   ascii: {}", s);
        }
    }
}
