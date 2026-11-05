//! Parser and manager for Enhanced Client (3D) GameData.uop and game configuration tables.

use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct ContainerBounds {
    pub gump_id: u32,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub name: String,
}

impl ContainerBounds {
    pub fn width(&self) -> i32 {
        (self.right - self.left).max(0)
    }

    pub fn height(&self) -> i32 {
        (self.bottom - self.top).max(0)
    }
}

#[derive(Clone, Debug)]
pub struct GameDataEntry {
    pub index: usize,
    pub hash: u64,
    pub compressed_size: usize,
    pub decompressed_size: usize,
    pub compression: u16,
    pub name: String,
    pub raw_data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct GameDataArchive {
    pub entries: Vec<GameDataEntry>,
    pub containers: HashMap<u32, ContainerBounds>,
    pub legacy_to_new: HashMap<u32, (u32, u32)>,
    pub body_to_mesh: HashMap<u32, (String, String)>,
}

impl GameDataArchive {
    pub const HASH_CONTAINERS: u64 = 0xE840D2286751791C;
    pub const HASH_LEGACY_CONV: u64 = 0x78AFB72A49CD9BE9;
    pub const HASH_BODY_CONV: u64 = 0xBF2AE37FAFC672E7;
    pub const HASH_RECIPES: u64 = 0xF45CF6B325F9D01A;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_directory<P: AsRef<Path>>(&mut self, dir: P) {
        let dir_ref = dir.as_ref();
        let target = "GameData.uop";
        let p = dir_ref.join(target);
        if p.is_file() {
            if let Ok(archive) = Self::load_from_uop(&p) {
                *self = archive;
                return;
            }
        }
        if let Ok(entries) = std::fs::read_dir(dir_ref) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                    if fname.eq_ignore_ascii_case(target) {
                        if let Ok(archive) = Self::load_from_uop(&path) {
                            *self = archive;
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn load_from_uop<P: AsRef<Path>>(uop_path: P) -> std::io::Result<Self> {
        let mut file = File::open(uop_path)?;
        let mut hdr = [0u8; 28];
        file.read_exact(&mut hdr)?;
        let magic = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
        if magic != 0x50594D {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UOP magic",
            ));
        }

        let mut next_block = u64::from_le_bytes([
            hdr[12], hdr[13], hdr[14], hdr[15], hdr[16], hdr[17], hdr[18], hdr[19],
        ]);

        let mut entries = Vec::new();
        let mut containers = HashMap::new();
        let mut legacy_to_new = HashMap::new();
        let mut body_to_mesh = HashMap::new();
        let mut entry_idx = 0;

        while next_block != 0 {
            file.seek(SeekFrom::Start(next_block))?;
            let mut block_hdr = [0u8; 12];
            file.read_exact(&mut block_hdr)?;
            let files_count =
                u32::from_le_bytes([block_hdr[0], block_hdr[1], block_hdr[2], block_hdr[3]])
                    as usize;
            next_block = u64::from_le_bytes([
                block_hdr[4],
                block_hdr[5],
                block_hdr[6],
                block_hdr[7],
                block_hdr[8],
                block_hdr[9],
                block_hdr[10],
                block_hdr[11],
            ]);

            for _ in 0..files_count {
                let mut eb = [0u8; 34];
                if file.read_exact(&mut eb).is_err() {
                    break;
                }
                let offset =
                    u64::from_le_bytes([eb[0], eb[1], eb[2], eb[3], eb[4], eb[5], eb[6], eb[7]]);
                let h_len = u32::from_le_bytes([eb[8], eb[9], eb[10], eb[11]]) as u64;
                let c_len = u32::from_le_bytes([eb[12], eb[13], eb[14], eb[15]]) as usize;
                let d_len = u32::from_le_bytes([eb[16], eb[17], eb[18], eb[19]]) as usize;
                let hash = u64::from_le_bytes([
                    eb[20], eb[21], eb[22], eb[23], eb[24], eb[25], eb[26], eb[27],
                ]);
                let comp_flag = u16::from_le_bytes([eb[32], eb[33]]);

                if offset == 0 || c_len == 0 {
                    continue;
                }

                let current_pos = file.stream_position()?;
                file.seek(SeekFrom::Start(offset + h_len))?;
                let mut raw = vec![0u8; c_len];
                file.read_exact(&mut raw)?;
                file.seek(SeekFrom::Start(current_pos))?;

                let payload = if comp_flag == 1 {
                    let mut decoder = ZlibDecoder::new(&raw[..]);
                    let mut decompressed = Vec::with_capacity(d_len);
                    if decoder.read_to_end(&mut decompressed).is_ok() {
                        decompressed
                    } else {
                        raw
                    }
                } else {
                    raw
                };

                let name = Self::identify_entry_name(hash, &payload, entry_idx);

                // Parse well-known tables
                if hash == Self::HASH_CONTAINERS || name.contains("legacycontainers") {
                    Self::parse_containers_csv(&payload, &mut containers);
                } else if hash == Self::HASH_LEGACY_CONV || name.contains("legacy_to_new") {
                    Self::parse_legacy_conv_csv(&payload, &mut legacy_to_new);
                } else if hash == Self::HASH_BODY_CONV || name.contains("body_to_mesh") {
                    Self::parse_body_conv_csv(&payload, &mut body_to_mesh);
                }

                entries.push(GameDataEntry {
                    index: entry_idx,
                    hash,
                    compressed_size: c_len,
                    decompressed_size: d_len,
                    compression: comp_flag,
                    name,
                    raw_data: payload,
                });

                entry_idx += 1;
            }
        }

        Ok(Self {
            entries,
            containers,
            legacy_to_new,
            body_to_mesh,
        })
    }

    pub fn identify_entry_name(hash: u64, data: &[u8], idx: usize) -> String {
        if hash == Self::HASH_CONTAINERS {
            return "legacycontainers.csv".to_string();
        } else if hash == Self::HASH_LEGACY_CONV {
            return "legacy_to_new_items.csv".to_string();
        } else if hash == Self::HASH_BODY_CONV {
            return "body_to_mesh.csv".to_string();
        } else if hash == Self::HASH_RECIPES {
            return "recipes.csv".to_string();
        }

        let first_line = data.split(|&b| b == b'\n').next().unwrap_or(&[]);
        let text = String::from_utf8_lossy(first_line).trim().to_string();
        let cleaned = text.trim_start_matches('#').trim();

        if !cleaned.is_empty()
            && cleaned.len() < 40
            && !cleaned.starts_with("int,")
            && !cleaned.contains("<xml")
            && !cleaned.contains("<?xml")
        {
            let sanitized: String = cleaned
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            return format!("{:02}_{}.csv", idx, sanitized.to_lowercase());
        }

        if cleaned.starts_with("<?xml") || cleaned.starts_with("<") {
            return format!("{:02}_table_{:016x}.xml", idx, hash);
        }

        format!("{:02}_table_{:016x}.csv", idx, hash)
    }

    fn parse_containers_csv(data: &[u8], map: &mut HashMap<u32, ContainerBounds>) {
        let text = String::from_utf8_lossy(data);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("gumpId") || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 5 {
                if let (Ok(gump_id), Ok(left), Ok(top), Ok(right), Ok(bottom)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<i32>(),
                    parts[2].parse::<i32>(),
                    parts[3].parse::<i32>(),
                    parts[4].parse::<i32>(),
                ) {
                    let name = if parts.len() > 5 {
                        parts[5].trim_matches('"').to_string()
                    } else {
                        format!("Container {}", gump_id)
                    };
                    map.insert(
                        gump_id,
                        ContainerBounds {
                            gump_id,
                            left,
                            top,
                            right,
                            bottom,
                            name,
                        },
                    );
                }
            }
        }
    }

    fn parse_legacy_conv_csv(data: &[u8], map: &mut HashMap<u32, (u32, u32)>) {
        let text = String::from_utf8_lossy(data);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("legacyId") || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                if let (Ok(legacy_id), Ok(new_id), Ok(new_sub)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                ) {
                    map.insert(legacy_id, (new_id, new_sub));
                }
            }
        }
    }

    fn parse_body_conv_csv(data: &[u8], map: &mut HashMap<u32, (String, String)>) {
        let text = String::from_utf8_lossy(data);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("BodyType") || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 2 {
                if let Ok(body) = parts[0].parse::<u32>() {
                    let fname = parts[1].trim_matches('"').to_string();
                    let desc = if parts.len() > 2 {
                        parts[2].trim_matches('"').to_string()
                    } else {
                        String::new()
                    };
                    map.insert(body, (fname, desc));
                }
            }
        }
    }

    pub fn parse_csv_rows(text: &str) -> (Vec<String>, Vec<Vec<String>>) {
        let mut rows: Vec<Vec<String>> = Vec::new();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let mut fields = Vec::new();
            let mut current = String::new();
            let mut in_quotes = false;
            for c in trimmed.chars() {
                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                    }
                    ',' if !in_quotes => {
                        fields.push(current.trim().to_string());
                        current.clear();
                    }
                    _ => {
                        current.push(c);
                    }
                }
            }
            fields.push(current.trim().to_string());
            if !fields.is_empty() {
                rows.push(fields);
            }
        }
        if rows.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            let headers = rows.remove(0);
            (headers, rows)
        }
    }
}
