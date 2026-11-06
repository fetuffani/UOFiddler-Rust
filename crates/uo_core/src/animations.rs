//! Fast decoder for Ultima Online animations (MUL and UOP formats).

use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::color::color16_to_rgba;
use crate::hash::hash_file_name;

const DOUBLE_XOR: u32 = (0x200 << 22) | (0x200 << 12);

#[derive(Clone, Debug)]
pub struct AnimationFrame {
    pub center_x: i16,
    pub center_y: i16,
    pub width: u16,
    pub height: u16,
    /// RGBA pixels (width * height * 4 bytes)
    pub rgba_data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct UopAnimEntry {
    pub file_path: PathBuf,
    pub data_offset: u64,
    pub compressed_len: usize,
    pub decompressed_len: usize,
    pub is_compressed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct UopAnimationIndex {
    pub entries: HashMap<u64, UopAnimEntry>,
    pub sequence_replacements: HashMap<u32, Vec<u8>>,
}

impl UopAnimationIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_directory<P: AsRef<Path>>(&mut self, dir: P) {
        let dir_ref = dir.as_ref();
        for i in 1..=6 {
            let uop_name = format!("AnimationFrame{}.uop", i);
            let p = dir_ref.join(&uop_name);
            if p.is_file() {
                self.load_animation_uop_file(&p);
            } else if let Ok(entries) = std::fs::read_dir(dir_ref) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                        if fname.eq_ignore_ascii_case(&uop_name) {
                            self.load_animation_uop_file(&path);
                            break;
                        }
                    }
                }
            }
        }

        // Also check AnimationSequence.uop
        let seq_name = "AnimationSequence.uop";
        let seq_path = dir_ref.join(seq_name);
        if seq_path.is_file() {
            self.load_sequence_uop(&seq_path);
        } else if let Ok(entries) = std::fs::read_dir(dir_ref) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                    if fname.eq_ignore_ascii_case(seq_name) {
                        self.load_sequence_uop(&path);
                        break;
                    }
                }
            }
        }
    }

    pub fn load_animation_uop_file(&mut self, path: &Path) {
        let Ok(mut file) = File::open(path) else {
            return;
        };
        let mut header = [0u8; 28];
        if file.read_exact(&mut header).is_err() {
            return;
        };
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != 0x50594D {
            return;
        };

        let mut next_block = u64::from_le_bytes([
            header[12], header[13], header[14], header[15], header[16], header[17], header[18],
            header[19],
        ]);

        while next_block != 0 {
            if file.seek(SeekFrom::Start(next_block)).is_err() {
                break;
            }
            let mut block_header = [0u8; 12];
            if file.read_exact(&mut block_header).is_err() {
                break;
            }
            let count = u32::from_le_bytes([
                block_header[0],
                block_header[1],
                block_header[2],
                block_header[3],
            ]) as usize;
            next_block = u64::from_le_bytes([
                block_header[4],
                block_header[5],
                block_header[6],
                block_header[7],
                block_header[8],
                block_header[9],
                block_header[10],
                block_header[11],
            ]);

            for _ in 0..count {
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
                let comp_flag = i16::from_le_bytes([eb[32], eb[33]]);

                if offset != 0 && c_len != 0 {
                    self.entries.insert(
                        hash,
                        UopAnimEntry {
                            file_path: path.to_path_buf(),
                            data_offset: offset + h_len,
                            compressed_len: c_len,
                            decompressed_len: d_len,
                            is_compressed: comp_flag != 0,
                        },
                    );
                }
            }
        }
    }

    pub fn load_sequence_uop(&mut self, path: &Path) {
        let Ok(mut file) = File::open(path) else {
            return;
        };
        let mut header = [0u8; 28];
        if file.read_exact(&mut header).is_err() {
            return;
        };
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != 0x50594D {
            return;
        };

        let mut next_block = u64::from_le_bytes([
            header[12], header[13], header[14], header[15], header[16], header[17], header[18],
            header[19],
        ]);

        let mut seq_entries = HashMap::new();
        while next_block != 0 {
            if file.seek(SeekFrom::Start(next_block)).is_err() {
                break;
            }
            let mut block_header = [0u8; 12];
            if file.read_exact(&mut block_header).is_err() {
                break;
            }
            let count = u32::from_le_bytes([
                block_header[0],
                block_header[1],
                block_header[2],
                block_header[3],
            ]) as usize;
            next_block = u64::from_le_bytes([
                block_header[4],
                block_header[5],
                block_header[6],
                block_header[7],
                block_header[8],
                block_header[9],
                block_header[10],
                block_header[11],
            ]);

            for _ in 0..count {
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
                let comp_flag = i16::from_le_bytes([eb[32], eb[33]]);

                if offset != 0 && c_len != 0 {
                    seq_entries.insert(hash, (offset + h_len, c_len, d_len, comp_flag != 0));
                }
            }
        }

        for anim_id in 0..2048u32 {
            let h = hash_file_name(&format!("build/animationsequence/{:08}.bin", anim_id));
            if let Some(&(pos, clen, dlen, is_comp)) = seq_entries.get(&h) {
                if file.seek(SeekFrom::Start(pos)).is_err() {
                    continue;
                }
                let mut raw = vec![0u8; clen];
                if file.read_exact(&mut raw).is_err() {
                    continue;
                }
                let payload = if is_comp {
                    let mut dec = ZlibDecoder::new(&raw[..]);
                    let mut decomp = Vec::with_capacity(dlen);
                    if dec.read_to_end(&mut decomp).is_ok() {
                        decomp
                    } else {
                        continue;
                    }
                } else {
                    raw
                };

                if payload.len() >= 56 {
                    let group_count =
                        i32::from_le_bytes([payload[52], payload[53], payload[54], payload[55]])
                            as usize;
                    let mut replacements: Vec<u8> = (0..80u8).collect();
                    let mut cursor = 56;
                    for _ in 0..group_count {
                        if cursor + 12 > payload.len() {
                            break;
                        }
                        let group = i32::from_le_bytes([
                            payload[cursor],
                            payload[cursor + 1],
                            payload[cursor + 2],
                            payload[cursor + 3],
                        ]);
                        let frame_count = i32::from_le_bytes([
                            payload[cursor + 4],
                            payload[cursor + 5],
                            payload[cursor + 6],
                            payload[cursor + 7],
                        ]);
                        let repl_group = i32::from_le_bytes([
                            payload[cursor + 8],
                            payload[cursor + 9],
                            payload[cursor + 10],
                            payload[cursor + 11],
                        ]);
                        if frame_count == 0
                            && group >= 0
                            && group < 80
                            && repl_group >= 0
                            && repl_group < 80
                        {
                            replacements[group as usize] = repl_group as u8;
                        }
                        cursor += 64;
                        if cursor + 4 <= payload.len() {
                            let c1 = i32::from_le_bytes([
                                payload[cursor],
                                payload[cursor + 1],
                                payload[cursor + 2],
                                payload[cursor + 3],
                            ]) as usize;
                            cursor += 4 + c1 * 32;
                        }
                        if cursor + 4 <= payload.len() {
                            let c2 = i32::from_le_bytes([
                                payload[cursor],
                                payload[cursor + 1],
                                payload[cursor + 2],
                                payload[cursor + 3],
                            ]) as usize;
                            cursor += 4 + c2 * 4;
                        }
                    }
                    self.sequence_replacements.insert(anim_id, replacements);
                }
            }
        }
    }

    pub fn get_resolved_action(&self, body: u32, action: u32) -> u32 {
        if let Some(repl) = self.sequence_replacements.get(&body) {
            if let Some(&r) = repl.get(action as usize) {
                return r as u32;
            }
        }
        action
    }

    pub fn is_action_defined(&self, body: u32, action: u32) -> bool {
        let resolved = self.get_resolved_action(body, action);
        let h = hash_file_name(&format!(
            "build/animationlegacyframe/{:06}/{:02}.bin",
            body, resolved
        ));
        self.entries.contains_key(&h)
    }

    pub fn get_animation_frames(
        &self,
        body: u32,
        action: u32,
        direction: u8,
    ) -> Option<Vec<AnimationFrame>> {
        let resolved = self.get_resolved_action(body, action);
        let h = hash_file_name(&format!(
            "build/animationlegacyframe/{:06}/{:02}.bin",
            body, resolved
        ));
        let entry = self.entries.get(&h)?;

        let mut file = File::open(&entry.file_path).ok()?;
        file.seek(SeekFrom::Start(entry.data_offset)).ok()?;

        let mut raw = vec![0u8; entry.compressed_len];
        file.read_exact(&mut raw).ok()?;

        let payload = if entry.is_compressed {
            let mut decoder = ZlibDecoder::new(&raw[..]);
            let mut decompressed = Vec::with_capacity(entry.decompressed_len.max(0));
            decoder.read_to_end(&mut decompressed).ok()?;
            decompressed
        } else {
            raw
        };

        let frames = decode_animation_uop(&payload, direction);
        if frames.is_empty() {
            None
        } else {
            Some(frames)
        }
    }
}

/// Decodes an animation frame sequence from a classic MUL data block.
pub fn decode_animation_mul(data: &[u8], flip: bool) -> Vec<AnimationFrame> {
    if data.len() < 516 {
        return Vec::new();
    }

    // 1. Decode 256-color palette (512 bytes)
    let mut palette = [[0u8; 4]; 256];
    for i in 0..256 {
        let raw_c = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
        palette[i] = color16_to_rgba(raw_c | 0x8000);
    }

    // 2. Read frame count
    let frame_count = u32::from_le_bytes([data[512], data[513], data[514], data[515]]) as usize;
    if frame_count == 0 || frame_count > 100 {
        return Vec::new();
    }

    if data.len() < 516 + frame_count * 4 {
        return Vec::new();
    }

    let mut frames = Vec::with_capacity(frame_count);

    for f in 0..frame_count {
        let off_pos = 516 + f * 4;
        let frame_offset = u32::from_le_bytes([
            data[off_pos],
            data[off_pos + 1],
            data[off_pos + 2],
            data[off_pos + 3],
        ]) as usize;

        let start = 512 + frame_offset;
        if start + 8 > data.len() {
            continue;
        }

        let x_center = i16::from_le_bytes([data[start], data[start + 1]]);
        let y_center = i16::from_le_bytes([data[start + 2], data[start + 3]]);
        let width = u16::from_le_bytes([data[start + 4], data[start + 5]]);
        let height = u16::from_le_bytes([data[start + 6], data[start + 7]]);

        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            continue;
        }

        let w = width as usize;
        let h = height as usize;
        let mut rgba = vec![0u8; w * h * 4];

        let mut cursor = start + 8;
        let x_base = (x_center as i32) - 0x200;
        let y_base = ((y_center as i32) + (height as i32)) - 0x200;

        while cursor + 4 <= data.len() {
            let mut header = u32::from_le_bytes([
                data[cursor],
                data[cursor + 1],
                data[cursor + 2],
                data[cursor + 3],
            ]);
            cursor += 4;

            if header == 0x7FFF_7FFF {
                break;
            }

            header ^= DOUBLE_XOR;
            let row_y = ((header >> 12) & 0x3FF) as i32;
            let row_x = ((header >> 22) & 0x3FF) as i32;
            let run = (header & 0xFFF) as usize;

            if cursor + run > data.len() {
                break;
            }

            let py = y_base + row_y;
            if py >= 0 && (py as usize) < h {
                let row_offset = (py as usize) * w * 4;

                for r in 0..run {
                    let px = if !flip {
                        x_base + row_x + (r as i32)
                    } else {
                        (width as i32) - 1 - (x_base + row_x + (r as i32))
                    };

                    if px >= 0 && (px as usize) < w {
                        let pal_idx = data[cursor + r] as usize;
                        let col = palette[pal_idx];
                        let pixel_idx = row_offset + (px as usize) * 4;
                        rgba[pixel_idx..pixel_idx + 4].copy_from_slice(&col);
                    }
                }
            }

            cursor += run;
        }

        let eff_center_x = if flip {
            (width as i16) - x_center
        } else {
            x_center
        };

        frames.push(AnimationFrame {
            center_x: eff_center_x,
            center_y: y_center,
            width,
            height,
            rgba_data: rgba,
        });
    }

    frames
}

/// Decodes frames for a specific direction from an uncompressed UOP animation frame block.
pub fn decode_animation_uop(raw: &[u8], direction: u8) -> Vec<AnimationFrame> {
    if raw.len() < 36 {
        return Vec::new();
    }

    let frame_count = u32::from_le_bytes([raw[32], raw[33], raw[34], raw[35]]) as usize;
    let data_start = u32::from_le_bytes([raw[36], raw[37], raw[38], raw[39]]) as usize;

    if frame_count == 0 || data_start >= raw.len() {
        return Vec::new();
    }

    let mut headers = Vec::with_capacity(frame_count);
    let mut pos_cursor = data_start;

    for _ in 0..frame_count {
        if pos_cursor + 16 > raw.len() {
            break;
        }
        let frame_id = u16::from_le_bytes([raw[pos_cursor + 2], raw[pos_cursor + 3]]);
        let pixel_offset = u32::from_le_bytes([
            raw[pos_cursor + 12],
            raw[pos_cursor + 13],
            raw[pos_cursor + 14],
            raw[pos_cursor + 15],
        ]);
        headers.push((pos_cursor, frame_id, pixel_offset));
        pos_cursor += 16;
    }

    let mut filled = Vec::new();
    let mut last_id = 1u16;

    for (pos_c, fid, pix_off) in headers {
        while fid > last_id + 1 {
            last_id += 1;
            filled.push((0usize, last_id, 0u32));
        }
        filled.push((pos_c, fid, pix_off));
        last_id = fid;
    }

    let real_fc = ((filled.len() as f32) / 5.0).round() as usize;
    if real_fc == 0 {
        return Vec::new();
    }

    let flip = direction > 4;
    let read_dir = if direction <= 4 {
        direction as usize
    } else {
        (direction - ((direction - 4) * 2)) as usize
    };

    let mut frames = Vec::new();

    for (pos_c, fid, pix_off) in filled {
        let frame_dir = ((fid.saturating_sub(1)) as usize) / real_fc;
        if frame_dir < read_dir {
            continue;
        }
        if frame_dir > read_dir {
            break;
        }

        if pos_c == 0 {
            continue;
        }

        let frame_pos = pos_c + (pix_off as usize);
        if frame_pos + 520 > raw.len() {
            continue;
        }

        let mut palette = [[0u8; 4]; 256];
        for i in 0..256 {
            let off = frame_pos + i * 2;
            let c = u16::from_le_bytes([raw[off], raw[off + 1]]);
            let c_fixed = if c == 0 { 0 } else { c | 0x8000 };
            palette[i] = color16_to_rgba(c_fixed);
        }

        let x_center = i16::from_le_bytes([raw[frame_pos + 512], raw[frame_pos + 513]]);
        let y_center = i16::from_le_bytes([raw[frame_pos + 514], raw[frame_pos + 515]]);
        let width = u16::from_le_bytes([raw[frame_pos + 516], raw[frame_pos + 517]]);
        let height = u16::from_le_bytes([raw[frame_pos + 518], raw[frame_pos + 519]]);

        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            continue;
        }

        let w = width as usize;
        let h = height as usize;
        let mut rgba = vec![0u8; w * h * 4];

        let mut cursor = frame_pos + 520;
        let x_base = (x_center as i32) - 0x200;
        let y_base = ((y_center as i32) + (height as i32)) - 0x200;

        while cursor + 4 <= raw.len() {
            let mut header = u32::from_le_bytes([
                raw[cursor],
                raw[cursor + 1],
                raw[cursor + 2],
                raw[cursor + 3],
            ]);
            cursor += 4;

            if header == 0x7FFF_7FFF {
                break;
            }

            header ^= DOUBLE_XOR;
            let row_y = ((header >> 12) & 0x3FF) as i32;
            let row_x = ((header >> 22) & 0x3FF) as i32;
            let run = (header & 0xFFF) as usize;

            if cursor + run > raw.len() {
                break;
            }

            let py = y_base + row_y;
            if py >= 0 && (py as usize) < h {
                let row_offset = (py as usize) * w * 4;

                for r in 0..run {
                    let px = if !flip {
                        x_base + row_x + (r as i32)
                    } else {
                        (width as i32) - 1 - (x_base + row_x + (r as i32))
                    };

                    if px >= 0 && (px as usize) < w {
                        let pal_idx = raw[cursor + r] as usize;
                        let col = palette[pal_idx];
                        let pixel_idx = row_offset + (px as usize) * 4;
                        rgba[pixel_idx..pixel_idx + 4].copy_from_slice(&col);
                    }
                }
            }

            cursor += run;
        }

        let eff_center_x = if flip {
            (width as i16) - x_center
        } else {
            x_center
        };

        frames.push(AnimationFrame {
            center_x: eff_center_x,
            center_y: y_center,
            width,
            height,
            rgba_data: rgba,
        });
    }

    frames
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationDef {
    pub body: u32,
    pub name: String,
    pub anim_type: u8, // 0: Monster, 1: Sea, 2: Animal, 3: Human
    pub is_equip: bool,
    pub is_uop: bool,
}

/// Parses Animationlist.xml entries (<Mob .../> and <Equip .../>).
pub fn parse_animation_list_xml(xml_content: &str) -> Vec<AnimationDef> {
    let mut defs = Vec::new();
    for line in xml_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<Mob") || trimmed.starts_with("<Equip") {
            let is_equip = trimmed.starts_with("<Equip");
            let body = trimmed
                .split("body=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse::<u32>().ok());
            let name = trimmed
                .split("name=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .unwrap_or("");
            let anim_type = trimmed
                .split("type=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(if is_equip { 3 } else { 0 });

            if let Some(b) = body {
                defs.push(AnimationDef {
                    body: b,
                    name: if name.is_empty() {
                        format!("Body {}", b)
                    } else {
                        name.to_string()
                    },
                    anim_type,
                    is_equip,
                    is_uop: false,
                });
            }
        }
    }
    defs
}

/// Body ids that mobtypes.txt finally classifies as EQUIPMENT. A body can be listed twice (for
/// example 734 is both an equipment animation and a monster); the last line of the file wins.
pub fn equipment_bodies_from_mobtypes(content: &str) -> std::collections::HashSet<u32> {
    let mut last: std::collections::HashMap<u32, bool> = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let mut it = line.split_whitespace();
        if let (Some(id), Some(kind)) = (it.next().and_then(|v| v.parse::<u32>().ok()), it.next()) {
            last.insert(id, kind.eq_ignore_ascii_case("EQUIPMENT"));
        }
    }
    last.into_iter().filter(|(_, e)| *e).map(|(id, _)| id).collect()
}

/// Parses mobtypes.txt entries.
pub fn parse_mobtypes_txt(content: &str) -> Vec<AnimationDef> {
    let mut defs = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(body) = parts[0].parse::<u32>() {
                let cat = parts[1].to_uppercase();
                let anim_type = match cat.as_str() {
                    "MONSTER" => 0,
                    "SEA_MONSTER" => 1,
                    "ANIMAL" => 2,
                    "HUMAN" | "EQUIPMENT" => 3,
                    _ => 0,
                };
                let is_equip = cat == "EQUIPMENT";
                let flag = if parts.len() >= 3 {
                    u32::from_str_radix(parts[2], 16).unwrap_or(0)
                } else {
                    0
                };
                let is_uop = (flag & 0x10000) != 0;
                defs.push(AnimationDef {
                    body,
                    name: format!("{} ({})", parts[1], body),
                    anim_type,
                    is_equip,
                    is_uop,
                });
            }
        }
    }
    defs
}

const EMBEDDED_ANIMATIONLIST_XML: &str = include_str!("../../../Animationlist.xml");

/// Loads animation definitions from Animationlist.xml merged with mobtypes.txt.
pub fn load_animation_defs(file_manager: &crate::FileManager) -> Vec<AnimationDef> {
    // 1. Check mobtypes.txt in UO directory or local candidates
    let mut mob_map: std::collections::HashMap<u32, (u8, bool, bool, String)> =
        std::collections::HashMap::new();
    let mob_candidates = [
        file_manager.get_file_path("mobtypes.txt"),
        Some(std::path::PathBuf::from("mobtypes.txt")),
        Some(std::path::PathBuf::from("../mobtypes.txt")),
    ];
    for opt_p in mob_candidates.iter().flatten() {
        if opt_p.is_file() {
            if let Ok(content) = std::fs::read_to_string(opt_p) {
                for def in parse_mobtypes_txt(&content) {
                    mob_map.insert(
                        def.body,
                        (def.anim_type, def.is_equip, def.is_uop, def.name),
                    );
                }
                if !mob_map.is_empty() {
                    break;
                }
            }
        }
    }

    // 2. Load Animationlist.xml
    let mut xml_content: Option<String> = None;
    let candidates = [
        std::path::PathBuf::from("Animationlist.xml"),
        std::path::PathBuf::from("../Animationlist.xml"),
        std::path::PathBuf::from("../../Animationlist.xml"),
    ];
    for path in &candidates {
        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(path) {
                xml_content = Some(content);
                break;
            }
        }
    }

    if xml_content.is_none() {
        if let Some(p) = file_manager.get_file_path("Animationlist.xml") {
            if let Ok(content) = std::fs::read_to_string(&p) {
                xml_content = Some(content);
            }
        }
    }

    let mut defs = if let Some(content) = xml_content {
        parse_animation_list_xml(&content)
    } else {
        parse_animation_list_xml(EMBEDDED_ANIMATIONLIST_XML)
    };

    let mut seen_bodies = std::collections::HashSet::new();
    for def in &mut defs {
        seen_bodies.insert(def.body);
        if let Some(&(mt, is_eq, is_u, _)) = mob_map.get(&def.body) {
            def.anim_type = mt;
            if is_eq {
                def.is_equip = true;
            }
            if is_u {
                def.is_uop = true;
            }
        } else {
            // Heuristic for modern UOP bodies if not in mobtypes
            if (def.body >= 605 && def.body <= 608)
                || (def.body >= 666 && def.body <= 667)
                || (def.body >= 694 && def.body <= 727)
                || (def.body >= 753 && def.body <= 758)
            {
                def.is_uop = true;
            }
        }
    }

    // Add remaining bodies from mobtypes.txt that were not in Animationlist.xml
    for (body, (anim_type, is_equip, is_uop, name)) in mob_map {
        if seen_bodies.insert(body) {
            defs.push(AnimationDef {
                body,
                name,
                anim_type,
                is_equip,
                is_uop,
            });
        }
    }

    defs.sort_by_key(|d| d.body);
    defs
}

#[cfg(test)]
mod equipment_tests {
    use super::*;

    #[test]
    fn last_mobtypes_line_wins() {
        let set = equipment_bodies_from_mobtypes("# c\n434\tEQUIPMENT\t0\n734\tEQUIPMENT\t0\n734\tMONSTER\t10000\t0 # Slith\n400\tHUMAN\t20000\n");
        assert!(set.contains(&434));
        assert!(!set.contains(&734), "734 is finally a monster");
        assert!(!set.contains(&400));
    }
}
