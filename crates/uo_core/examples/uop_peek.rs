use std::path::Path;
use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let p = Path::new(&dir).join("MainMisc.uop");
    let u = read_uop(&p).unwrap();
    let e = &u.entries[3];
    println!("{}", decode_text(&read_uop_entry(&p, e).unwrap()));
}
