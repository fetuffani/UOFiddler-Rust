//! Converts between the legacy UOP containers (artLegacyMUL.uop, gumpartLegacyMUL.uop,
//! mapNLegacyMUL.uop) and the classic MUL/IDX files, in both directions.
//!
//! Sound (soundLegacyMUL.uop) and multi (MultiCollection.uop) are NOT handled: their UOP entries
//! do not carry the per-entry `extra` data the classic index needs, or use a different layout.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::client_files::{read_uop, read_uop_entry};
use crate::hash::hash_file_name;

const MAP_CHUNK: usize = 0xC4000;
const UOP_BLOCK_CAPACITY: usize = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyKind {
    Art,
    Gump,
    Map(u8),
}

impl LegacyKind {
    pub fn uop_name(&self) -> String {
        match self {
            Self::Art => "artLegacyMUL.uop".into(),
            Self::Gump => "gumpartLegacyMUL.uop".into(),
            Self::Map(n) => format!("map{}LegacyMUL.uop", n),
        }
    }
    /// (idx name, mul name); maps have no idx.
    pub fn mul_names(&self) -> (Option<String>, String) {
        match self {
            Self::Art => (Some("artidx.mul".into()), "art.mul".into()),
            Self::Gump => (Some("gumpidx.mul".into()), "gumpart.mul".into()),
            Self::Map(n) => (None, format!("map{}.mul", n)),
        }
    }
    fn entry_name(&self, i: usize) -> String {
        match self {
            Self::Art => format!("build/artlegacymul/{:08}.tga", i),
            Self::Gump => format!("build/gumpartlegacymul/{:08}.tga", i),
            Self::Map(n) => format!("build/map{}legacymul/{:08}.dat", n, i),
        }
    }
    fn max_entries(&self) -> usize {
        match self {
            Self::Art => 0x14000,
            Self::Gump => 0x10000,
            Self::Map(_) => 4096,
        }
    }
    pub fn all() -> Vec<LegacyKind> {
        let mut v = vec![Self::Art, Self::Gump];
        v.extend((0..=5).map(Self::Map));
        v
    }
}

/// Writes a MYP (version 5) container with the given (entry name, data) pairs, stored uncompressed.
pub fn write_uop(path: &Path, entries: &[(String, Vec<u8>)]) -> std::io::Result<()> {
    const HEADER: u64 = 0x28;
    let block_size = 12 + UOP_BLOCK_CAPACITY as u64 * 34;
    let blocks = entries.len().div_ceil(UOP_BLOCK_CAPACITY).max(1) as u64;
    let data_start = HEADER + blocks * block_size;

    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(b"MYP\0");
    out.extend_from_slice(&5u32.to_le_bytes());
    out.extend_from_slice(&0xFD23_EC43u32.to_le_bytes());
    out.extend_from_slice(&HEADER.to_le_bytes());
    out.extend_from_slice(&(UOP_BLOCK_CAPACITY as u32).to_le_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    out.resize(HEADER as usize, 0);

    let mut offset = data_start;
    for b in 0..blocks as usize {
        let chunk = &entries[b * UOP_BLOCK_CAPACITY..((b + 1) * UOP_BLOCK_CAPACITY).min(entries.len())];
        out.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
        let next = if (b as u64) + 1 < blocks { HEADER + (b as u64 + 1) * block_size } else { 0 };
        out.extend_from_slice(&next.to_le_bytes());
        for (name, data) in chunk {
            out.extend_from_slice(&offset.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&(data.len() as u32).to_le_bytes());
            out.extend_from_slice(&(data.len() as u32).to_le_bytes());
            out.extend_from_slice(&hash_file_name(name).to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            offset += data.len() as u64;
        }
        for _ in chunk.len()..UOP_BLOCK_CAPACITY {
            out.extend_from_slice(&[0u8; 34]);
        }
    }
    debug_assert_eq!(out.len() as u64, data_start);
    let mut f = fs::File::create(path)?;
    f.write_all(&out)?;
    for (_, d) in entries {
        f.write_all(d)?;
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct ConvertReport {
    pub entries: usize,
    pub bytes: u64,
    pub outputs: Vec<PathBuf>,
}

/// UOP -> MUL (+IDX). Writes `out_dir/<mul>` and `out_dir/<idx>` for art/gump.
pub fn uop_to_mul(kind: LegacyKind, uop_path: &Path, out_dir: &Path) -> Result<ConvertReport, String> {
    let uop = read_uop(uop_path).ok_or("UOP inválido ou ilegível")?;
    let by_hash: std::collections::HashMap<u64, usize> =
        uop.entries.iter().enumerate().map(|(i, e)| (e.hash, i)).collect();
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let (idx_name, mul_name) = kind.mul_names();
    let mul_path = out_dir.join(&mul_name);
    let mut report = ConvertReport::default();

    match kind {
        LegacyKind::Map(_) => {
            let mut mul = fs::File::create(&mul_path).map_err(|e| e.to_string())?;
            for i in 0..kind.max_entries() {
                let Some(&ei) = by_hash.get(&hash_file_name(&kind.entry_name(i))) else { break };
                let data = read_uop_entry(uop_path, &uop.entries[ei]).ok_or("falha ao ler entrada")?;
                mul.write_all(&data).map_err(|e| e.to_string())?;
                report.entries += 1;
                report.bytes += data.len() as u64;
            }
            report.outputs.push(mul_path);
        }
        _ => {
            let mut mul = fs::File::create(&mul_path).map_err(|e| e.to_string())?;
            let mut idx: Vec<u8> = Vec::new();
            let mut last_valid = 0usize;
            let mut pos: u32 = 0;
            let total = kind.max_entries();
            let mut records: Vec<(i32, i32, i32)> = vec![(-1, -1, -1); total];
            for i in 0..total {
                let Some(&ei) = by_hash.get(&hash_file_name(&kind.entry_name(i))) else { continue };
                let data = read_uop_entry(uop_path, &uop.entries[ei]).ok_or("falha ao ler entrada")?;
                let (payload, extra) = if kind == LegacyKind::Gump {
                    if data.len() < 8 {
                        continue;
                    }
                    let w = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                    let h = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                    (&data[8..], ((w << 16) | (h & 0xFFFF)) as i32)
                } else {
                    (&data[..], 0)
                };
                mul.write_all(payload).map_err(|e| e.to_string())?;
                records[i] = (pos as i32, payload.len() as i32, extra);
                pos += payload.len() as u32;
                last_valid = i + 1;
                report.entries += 1;
                report.bytes += payload.len() as u64;
            }
            for r in &records[..last_valid] {
                idx.extend_from_slice(&r.0.to_le_bytes());
                idx.extend_from_slice(&r.1.to_le_bytes());
                idx.extend_from_slice(&r.2.to_le_bytes());
            }
            let idx_path = out_dir.join(idx_name.unwrap());
            fs::write(&idx_path, idx).map_err(|e| e.to_string())?;
            report.outputs.push(mul_path);
            report.outputs.push(idx_path);
        }
    }
    Ok(report)
}

/// MUL (+IDX) -> UOP. Reads `in_dir/<mul>` (+ idx) and writes `out_uop`.
pub fn mul_to_uop(kind: LegacyKind, in_dir: &Path, out_uop: &Path) -> Result<ConvertReport, String> {
    let (idx_name, mul_name) = kind.mul_names();
    let mul = fs::read(in_dir.join(&mul_name)).map_err(|e| format!("{}: {}", mul_name, e))?;
    let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
    match kind {
        LegacyKind::Map(_) => {
            for (i, chunk) in mul.chunks(MAP_CHUNK).enumerate() {
                entries.push((kind.entry_name(i), chunk.to_vec()));
            }
        }
        _ => {
            let idx_name = idx_name.unwrap();
            let idx = fs::read(in_dir.join(&idx_name)).map_err(|e| format!("{}: {}", idx_name, e))?;
            for (i, rec) in idx.chunks_exact(12).enumerate().take(kind.max_entries()) {
                let off = i32::from_le_bytes(rec[0..4].try_into().unwrap());
                let len = i32::from_le_bytes(rec[4..8].try_into().unwrap());
                let extra = i32::from_le_bytes(rec[8..12].try_into().unwrap());
                if off < 0 || len <= 0 || (off as usize + len as usize) > mul.len() {
                    continue;
                }
                let payload = &mul[off as usize..off as usize + len as usize];
                let data = if kind == LegacyKind::Gump {
                    let (w, h) = (((extra as u32) >> 16) & 0xFFFF, (extra as u32) & 0xFFFF);
                    let mut d = Vec::with_capacity(8 + payload.len());
                    d.extend_from_slice(&w.to_le_bytes());
                    d.extend_from_slice(&h.to_le_bytes());
                    d.extend_from_slice(payload);
                    d
                } else {
                    payload.to_vec()
                };
                entries.push((kind.entry_name(i), data));
            }
        }
    }
    write_uop(out_uop, &entries).map_err(|e| e.to_string())?;
    Ok(ConvertReport {
        entries: entries.len(),
        bytes: entries.iter().map(|e| e.1.len() as u64).sum(),
        outputs: vec![out_uop.to_path_buf()],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("uoconv_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn gump_roundtrip() {
        let src = tmp("gsrc");
        let mut mul = Vec::new();
        let mut idx = Vec::new();
        for (i, (w, h)) in [(2u32, 3u32), (0, 0), (5, 1)].iter().enumerate() {
            if *w == 0 {
                idx.extend_from_slice(&[0xFF; 12]);
                continue;
            }
            let payload: Vec<u8> = (0..(w * h * 4)).map(|b| (b as usize + i) as u8).collect();
            idx.extend_from_slice(&(mul.len() as i32).to_le_bytes());
            idx.extend_from_slice(&(payload.len() as i32).to_le_bytes());
            idx.extend_from_slice(&(((w << 16) | h) as i32).to_le_bytes());
            mul.extend_from_slice(&payload);
        }
        fs::write(src.join("gumpart.mul"), &mul).unwrap();
        fs::write(src.join("gumpidx.mul"), &idx).unwrap();
        let uop = src.join("gumpartLegacyMUL.uop");
        let r = mul_to_uop(LegacyKind::Gump, &src, &uop).unwrap();
        assert_eq!(r.entries, 2);
        let out = tmp("gout");
        uop_to_mul(LegacyKind::Gump, &uop, &out).unwrap();
        assert_eq!(fs::read(out.join("gumpart.mul")).unwrap(), mul);
        let got = fs::read(out.join("gumpidx.mul")).unwrap();
        assert_eq!(&got[..12], &idx[..12]);
        assert_eq!(&got[24..36], &idx[24..36]);
    }

    #[test]
    fn map_roundtrip_multiple_blocks() {
        let src = tmp("msrc");
        // 250 chunks => 3 UOP blocks; last chunk is partial
        let mul: Vec<u8> = (0..MAP_CHUNK * 249 + 1234).map(|i| (i * 31 % 251) as u8).collect();
        fs::write(src.join("map1.mul"), &mul).unwrap();
        let uop = src.join("map1LegacyMUL.uop");
        let r = mul_to_uop(LegacyKind::Map(1), &src, &uop).unwrap();
        assert_eq!(r.entries, 250);
        let out = tmp("mout");
        uop_to_mul(LegacyKind::Map(1), &uop, &out).unwrap();
        assert_eq!(fs::read(out.join("map1.mul")).unwrap(), mul);
    }
}
