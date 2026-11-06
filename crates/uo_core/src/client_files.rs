//! Generic readers for the many small files the UO client loads: text/def/cfg tables,
//! IFF localized string files (Tooltips.enu, Intloc*.enu, Gt_*.enu ...), and a generic
//! MYP (.uop) container reader.

use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::hash::hash_file_name;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClientFileKind {
    Text,
    Def,
    Iff,
    Uop,
    Mul,
    Idx,
    Cliloc,
    Binary,
    Executable,
    Other,
}

impl ClientFileKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Text => "Texto",
            Self::Def => "Def/Cfg",
            Self::Iff => "Localização (IFF)",
            Self::Uop => "UOP",
            Self::Mul => "MUL",
            Self::Idx => "IDX",
            Self::Cliloc => "Cliloc",
            Self::Binary => "Binário",
            Self::Executable => "Executável/Lib",
            Self::Other => "Outro",
        }
    }
}

/// Classifies a client file by name (and, for IFF, by its first bytes when provided).
pub fn classify_file(name: &str, head: Option<&[u8]>) -> ClientFileKind {
    let lower = name.to_lowercase();
    let ext = lower.rsplit('.').next().unwrap_or("");
    if lower.starts_with("cliloc.") {
        return ClientFileKind::Cliloc;
    }
    if let Some(h) = head {
        if h.len() >= 4 && &h[..4] == b"FORM" {
            return ClientFileKind::Iff;
        }
        if h.len() >= 4 && &h[..3] == b"MYP" {
            return ClientFileKind::Uop;
        }
    }
    match ext {
        "uop" => ClientFileKind::Uop,
        "idx" => ClientFileKind::Idx,
        "mul" => ClientFileKind::Mul,
        "def" | "cfg" | "lst" | "cht" if ext == "def" || ext == "cfg" || ext == "lst" => ClientFileKind::Def,
        "txt" | "log" | "ini" | "xml" | "csv" | "json" => ClientFileKind::Text,
        "exe" | "dll" | "chm" | "nwb" => ClientFileKind::Executable,
        "bin" | "rle" | "tbp" | "iff" | "dat" | "uo1" | "bak" => ClientFileKind::Binary,
        _ => ClientFileKind::Other,
    }
}

/// Decodes bytes as text: UTF-16 (BOM), UTF-8, or Latin-1 as a fallback.
pub fn decode_text(data: &[u8]) -> String {
    if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xFE {
        let u: Vec<u16> = data[2..].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        return String::from_utf16_lossy(&u);
    }
    if data.len() >= 2 && data[0] == 0xFE && data[1] == 0xFF {
        let u: Vec<u16> = data[2..].chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        return String::from_utf16_lossy(&u);
    }
    let d = data.strip_prefix(&[0xEF, 0xBB, 0xBF][..]).unwrap_or(data);
    match std::str::from_utf8(d) {
        Ok(s) => s.to_string(),
        Err(_) => d.iter().map(|&b| b as char).collect(),
    }
}

/// True if the data looks like human readable text (few control characters).
pub fn looks_like_text(data: &[u8]) -> bool {
    let sample = &data[..data.len().min(4096)];
    if sample.is_empty() {
        return true;
    }
    if sample.len() >= 2 && ((sample[0] == 0xFF && sample[1] == 0xFE) || (sample[0] == 0xFE && sample[1] == 0xFF)) {
        return true;
    }
    let bad = sample
        .iter()
        .filter(|&&b| b == 0 || (b < 9) || (b > 13 && b < 32))
        .count();
    bad * 50 < sample.len()
}

/// Splits a line-oriented client table (Body.def, mobtypes.txt, Tile256.txt ...) into rows
/// of tokens. `{a, b}` groups are kept as a single token; `#` comments and the client's
/// `"#...` quoted comment lines are dropped.
pub fn parse_table(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let mut l = line.trim().trim_start_matches('\u{feff}');
        if l.starts_with("\"#") {
            continue;
        }
        if let Some(i) = l.find('#') {
            l = &l[..i];
        }
        let l = l.trim();
        if l.is_empty() {
            continue;
        }
        let mut toks = Vec::new();
        let mut cur = String::new();
        let mut depth = 0;
        for ch in l.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    cur.push(ch);
                }
                '}' => {
                    depth -= 1;
                    cur.push(ch);
                }
                c if c.is_whitespace() && depth == 0 => {
                    if !cur.is_empty() {
                        toks.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            toks.push(cur);
        }
        rows.push(toks);
    }
    rows
}

#[derive(Clone, Debug, Default)]
pub struct IffText {
    pub lang: String,
    pub name: String,
    pub declared_count: u32,
    pub strings: Vec<String>,
}

fn be32(d: &[u8], o: usize) -> Option<usize> {
    d.get(o..o + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

/// Parses the IFF "FORM/DATA/LANGINFO/TEXT" localized string container.
pub fn parse_iff_text(d: &[u8]) -> Option<IffText> {
    if d.len() < 12 || &d[..4] != b"FORM" {
        return None;
    }
    let mut out = IffText::default();
    // LANGINFO chunk: lang\0, u32, u32, name\0, u32, count(u32)
    if let Some(p) = d.windows(8).position(|w| w == b"LANGINFO") {
        let size = be32(d, p + 8)?;
        let body = d.get(p + 12..p + 12 + size.min(d.len() - p - 12))?;
        let mut parts = body.splitn(2, |&b| b == 0);
        out.lang = String::from_utf8_lossy(parts.next()?).to_string();
        let rest = parts.next().unwrap_or(&[]);
        if rest.len() > 8 {
            let after = &rest[8..];
            let mut p2 = after.splitn(2, |&b| b == 0);
            out.name = String::from_utf8_lossy(p2.next().unwrap_or(&[])).to_string();
            let tail = p2.next().unwrap_or(&[]);
            if tail.len() >= 8 {
                out.declared_count = u32::from_le_bytes([tail[4], tail[5], tail[6], tail[7]]);
            }
        }
    }
    let t = d.windows(4).position(|w| w == b"TEXT")?;
    let size = be32(d, t + 4)?;
    let start = t + 8;
    let end = (start + size).min(d.len());
    let body = &d[start..end];
    let mut strings: Vec<String> = body.split(|&b| b == 0).map(|s| decode_text(s)).collect();
    if body.last() == Some(&0) {
        strings.pop();
    }
    if out.declared_count > 0 {
        strings.truncate(out.declared_count as usize);
    }
    out.strings = strings;
    Some(out)
}

#[derive(Clone, Debug)]
pub struct UopFileEntry {
    pub hash: u64,
    pub offset: u64,
    pub header_len: u32,
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub compressed: bool,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct UopFile {
    pub version: u32,
    pub entries: Vec<UopFileEntry>,
}

/// Reads every entry of a MYP (.uop) container.
pub fn read_uop(path: &Path) -> Option<UopFile> {
    let mut f = File::open(path).ok()?;
    let mut h = [0u8; 28];
    f.read_exact(&mut h).ok()?;
    if &h[..3] != b"MYP" {
        return None;
    }
    let version = u32::from_le_bytes([h[4], h[5], h[6], h[7]]);
    let mut next = u64::from_le_bytes(h[12..20].try_into().ok()?);
    let mut out = UopFile { version, entries: Vec::new() };
    let mut guard = 0;
    while next != 0 && guard < 100_000 {
        guard += 1;
        f.seek(SeekFrom::Start(next)).ok()?;
        let mut bh = [0u8; 12];
        f.read_exact(&mut bh).ok()?;
        let count = u32::from_le_bytes([bh[0], bh[1], bh[2], bh[3]]) as usize;
        next = u64::from_le_bytes(bh[4..12].try_into().ok()?);
        for _ in 0..count {
            let mut e = [0u8; 34];
            if f.read_exact(&mut e).is_err() {
                break;
            }
            let offset = u64::from_le_bytes(e[0..8].try_into().unwrap());
            if offset == 0 {
                continue;
            }
            out.entries.push(UopFileEntry {
                hash: u64::from_le_bytes(e[20..28].try_into().unwrap()),
                offset,
                header_len: u32::from_le_bytes(e[8..12].try_into().unwrap()),
                compressed_len: u32::from_le_bytes(e[12..16].try_into().unwrap()),
                decompressed_len: u32::from_le_bytes(e[16..20].try_into().unwrap()),
                compressed: i16::from_le_bytes([e[32], e[33]]) != 0,
                name: None,
            });
        }
    }
    Some(out)
}

/// Reads (and decompresses if needed) one UOP entry.
pub fn read_uop_entry(path: &Path, e: &UopFileEntry) -> Option<Vec<u8>> {
    let mut f = File::open(path).ok()?;
    f.seek(SeekFrom::Start(e.offset + e.header_len as u64)).ok()?;
    let mut raw = vec![0u8; e.compressed_len as usize];
    f.read_exact(&mut raw).ok()?;
    if e.compressed {
        let mut out = Vec::with_capacity(e.decompressed_len as usize);
        ZlibDecoder::new(&raw[..]).read_to_end(&mut out).ok()?;
        Some(out)
    } else {
        Some(raw)
    }
}

/// Names known entries by trying the path patterns the client uses.
pub fn resolve_uop_names(file: &mut UopFile, file_name: &str) {
    let lower = file_name.to_lowercase();
    let mut known: HashMap<u64, String> = HashMap::new();
    let mut add = |s: String| {
        known.insert(hash_file_name(&s), s);
    };
    if lower.starts_with("animationframe") {
        for body in 0..2000u32 {
            for action in 0..80u32 {
                add(format!("build/animationlegacyframe/{:06}/{:02}.bin", body, action));
            }
        }
    } else if lower.starts_with("animationsequence") {
        for id in 0..4096u32 {
            add(format!("build/animationsequence/{:08}.bin", id));
        }
    } else if lower == "string_dictionary.uop" {
        add("build/stringdictionary/string_dictionary.bin".into());
    } else if let Some(stem) = lower.strip_suffix(".uop") {
        // generic pattern used by the *LegacyMUL.uop files
        let base = stem.trim_end_matches("legacymul");
        for i in 0..0x14000u32 {
            add(format!("build/{}legacymul/{:08}.tga", base, i));
            add(format!("build/{}legacymul/{:08}.dds", base, i));
        }
        for i in 0..64u32 {
            add(format!("build/{}/{:08}.dat", stem, i));
        }
        add(format!("build/{}/{}.bin", stem, stem));
    }
    for e in &mut file.entries {
        e.name = known.get(&e.hash).cloned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iff(name: &str, strings: &[&str]) -> Vec<u8> {
        let mut text = Vec::new();
        for s in strings {
            text.extend_from_slice(s.as_bytes());
            text.push(0);
        }
        let mut info = Vec::new();
        info.extend_from_slice(b"enu\0");
        info.extend_from_slice(&1u32.to_le_bytes());
        info.extend_from_slice(&0u32.to_le_bytes());
        info.extend_from_slice(name.as_bytes());
        info.push(0);
        info.extend_from_slice(&1u32.to_le_bytes());
        info.extend_from_slice(&(strings.len() as u32).to_le_bytes());
        let mut d = Vec::new();
        d.extend_from_slice(b"FORM\0\0\0\0DATAFORM\0\0\0\0LANGINFO");
        d.extend_from_slice(&(info.len() as u32).to_be_bytes());
        d.extend_from_slice(&info);
        d.extend_from_slice(b"TEXT");
        d.extend_from_slice(&(text.len() as u32).to_be_bytes());
        d.extend_from_slice(&text);
        d
    }

    #[test]
    fn iff_roundtrip() {
        let d = iff("intloc01", &["Participants", "", "Hide My Identity"]);
        let t = parse_iff_text(&d).unwrap();
        assert_eq!(t.lang, "enu");
        assert_eq!(t.name, "intloc01");
        assert_eq!(t.declared_count, 3);
        assert_eq!(t.strings, vec!["Participants", "", "Hide My Identity"]);
    }

    #[test]
    fn table_tokens() {
        let rows = parse_table("# c\n\"# quoted\n11 {28, 5} 1401 # x\n\n157\t1\t-1\n");
        assert_eq!(rows[0], vec!["11", "{28, 5}", "1401"]);
        assert_eq!(rows[1], vec!["157", "1", "-1"]);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn classify() {
        assert_eq!(classify_file("Body.def", None), ClientFileKind::Def);
        assert_eq!(classify_file("Tooltips.enu", Some(b"FORM")), ClientFileKind::Iff);
        assert_eq!(classify_file("Cliloc.enu", None), ClientFileKind::Cliloc);
        assert_eq!(classify_file("x.uop", None), ClientFileKind::Uop);
    }
}
