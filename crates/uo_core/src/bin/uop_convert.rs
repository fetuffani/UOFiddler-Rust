//! uop_convert: converts the legacy UOP containers to MUL/IDX and back.
//!
//!   uop_convert to-mul <dir-with-uops> <out-dir>    artLegacyMUL/gumpartLegacyMUL/mapNLegacyMUL .uop -> .mul/.idx
//!   uop_convert to-uop <dir-with-muls> <out-dir>    art/gumpart/mapN .mul/.idx -> .uop
//!
//! Every kind found in the input directory is converted (file names are matched case-insensitively).
use std::path::{Path, PathBuf};
use uo_core::{mul_to_uop, uop_to_mul, LegacyKind};

fn find(dir: &Path, name: &str) -> Option<PathBuf> {
    std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).find(|p| {
        p.file_name().and_then(|n| n.to_str()).map_or(false, |n| n.eq_ignore_ascii_case(name))
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 || !matches!(args[1].as_str(), "to-mul" | "to-uop") {
        eprintln!("uso: uop_convert <to-mul|to-uop> <pasta-origem> <pasta-destino>");
        std::process::exit(2);
    }
    let (input, output) = (Path::new(&args[2]), Path::new(&args[3]));
    let _ = std::fs::create_dir_all(output);
    let mut done = 0;
    let mut failed = 0;
    for kind in LegacyKind::all() {
        let res = if args[1] == "to-mul" {
            let Some(uop) = find(input, &kind.uop_name()) else { continue };
            print!("{} -> mul ... ", kind.uop_name());
            uop_to_mul(kind, &uop, output)
        } else {
            let (_, mul) = kind.mul_names();
            if find(input, &mul).is_none() {
                continue;
            }
            print!("{} -> uop ... ", mul);
            mul_to_uop(kind, input, &output.join(kind.uop_name()))
        };
        match res {
            Ok(r) => {
                println!("ok ({} entradas, {} bytes)", r.entries, r.bytes);
                done += 1;
            }
            Err(e) => {
                println!("ERRO: {}", e);
                failed += 1;
            }
        }
    }
    println!("{} convertidos, {} com erro", done, failed);
    if failed > 0 {
        std::process::exit(1);
    }
}
