//! Multis reader (houses, ships, structures) and high-performance isometric software renderer.

use crate::art::ArtBitmap;

#[derive(Clone, Copy, Debug)]
pub struct MultiComponent {
    pub item_id: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub offset_z: i16,
    pub flags: u32,
    pub unk1: u32,
}

#[derive(Clone, Debug)]
pub struct MultiStructure {
    pub multi_id: u32,
    pub components: Vec<MultiComponent>,
    pub min_x: i16,
    pub max_x: i16,
    pub min_y: i16,
    pub max_y: i16,
    pub min_z: i16,
    pub max_z: i16,
}

impl MultiStructure {
    /// Parses one multi, guessing the record size from this entry alone. This is ambiguous for
    /// lengths divisible by 48; prefer `parse_sized` with `detect_multi_record_size`.
    pub fn parse(multi_id: u32, data: &[u8]) -> Option<Self> {
        let record_size = if data.len() % 16 == 0 && data.len() % 12 != 0 {
            16
        } else {
            12
        };
        Self::parse_sized(multi_id, data, record_size)
    }

    /// Parses one multi with an explicit record size (12 = classic, 16 = High Seas / 7.0.9+).
    pub fn parse_sized(multi_id: u32, data: &[u8], record_size: usize) -> Option<Self> {
        if data.is_empty() || (record_size != 12 && record_size != 16) {
            return None;
        }

        let count = data.len() / record_size;
        let mut components = Vec::with_capacity(count);

        let mut min_x = i16::MAX;
        let mut max_x = i16::MIN;
        let mut min_y = i16::MAX;
        let mut max_y = i16::MIN;
        let mut min_z = i16::MAX;
        let mut max_z = i16::MIN;

        for i in 0..count {
            let off = i * record_size;
            let item_id = u16::from_le_bytes([data[off], data[off + 1]]);
            let ox = i16::from_le_bytes([data[off + 2], data[off + 3]]);
            let oy = i16::from_le_bytes([data[off + 4], data[off + 5]]);
            let oz = i16::from_le_bytes([data[off + 6], data[off + 7]]);
            let flags =
                u32::from_le_bytes([data[off + 8], data[off + 9], data[off + 10], data[off + 11]]);
            let unk1 = if record_size == 16 {
                u32::from_le_bytes([
                    data[off + 12],
                    data[off + 13],
                    data[off + 14],
                    data[off + 15],
                ])
            } else {
                0
            };

            min_x = min_x.min(ox);
            max_x = max_x.max(ox);
            min_y = min_y.min(oy);
            max_y = max_y.max(oy);
            min_z = min_z.min(oz);
            max_z = max_z.max(oz);

            components.push(MultiComponent {
                item_id,
                offset_x: ox,
                offset_y: oy,
                offset_z: oz,
                flags,
                unk1,
            });
        }

        if components.is_empty() {
            min_x = 0;
            max_x = 0;
            min_y = 0;
            max_y = 0;
            min_z = 0;
            max_z = 0;
        }

        Some(Self {
            multi_id,
            components,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        })
    }

    /// Parses a modern UOP Multi entry from MultiCollection.uop payload.
    pub fn parse_uop(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        let multi_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let comp_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let mut components = Vec::with_capacity(comp_count);

        let mut cursor = 8;
        let mut min_x = i16::MAX;
        let mut max_x = i16::MIN;
        let mut min_y = i16::MAX;
        let mut max_y = i16::MIN;
        let mut min_z = i16::MAX;
        let mut max_z = i16::MIN;

        for _ in 0..comp_count {
            if cursor + 14 > data.len() {
                break;
            }
            let item_id = u16::from_le_bytes([data[cursor], data[cursor + 1]]);
            let ox = i16::from_le_bytes([data[cursor + 2], data[cursor + 3]]);
            let oy = i16::from_le_bytes([data[cursor + 4], data[cursor + 5]]);
            let oz = i16::from_le_bytes([data[cursor + 6], data[cursor + 7]]);
            let flags = u16::from_le_bytes([data[cursor + 8], data[cursor + 9]]) as u32;
            let extra = u32::from_le_bytes([
                data[cursor + 10],
                data[cursor + 11],
                data[cursor + 12],
                data[cursor + 13],
            ]) as usize;
            cursor += 14 + (extra * 4);

            min_x = min_x.min(ox);
            max_x = max_x.max(ox);
            min_y = min_y.min(oy);
            max_y = max_y.max(oy);
            min_z = min_z.min(oz);
            max_z = max_z.max(oz);

            components.push(MultiComponent {
                item_id,
                offset_x: ox,
                offset_y: oy,
                offset_z: oz,
                flags,
                unk1: 0,
            });
        }

        if components.is_empty() {
            min_x = 0;
            max_x = 0;
            min_y = 0;
            max_y = 0;
            min_z = 0;
            max_z = 0;
        }

        Some(Self {
            multi_id,
            components,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        })
    }

    /// Renders the multi structure into a newly allocated RGBA bitmap with isometric projection.
    /// `get_art`: Callback to fetch static art item by ID.
    pub fn render_isometric<F>(&self, max_z_cutoff: i16, mut get_art: F) -> Option<ArtBitmap>
    where
        F: FnMut(u16) -> Option<ArtBitmap>,
    {
        if self.components.is_empty() {
            return None;
        }

        let mut sorted = self.components.clone();
        // Depth sorting: Z first, then (X + Y), then X
        sorted.sort_by_key(|c| {
            (
                c.offset_z,
                (c.offset_x as i32) + (c.offset_y as i32),
                c.offset_x,
            )
        });

        struct DrawItem {
            art: ArtBitmap,
            px: i32,
            py: i32,
        }

        let mut draw_items = Vec::with_capacity(sorted.len());
        let mut screen_min_x = i32::MAX;
        let mut screen_max_x = i32::MIN;
        let mut screen_min_y = i32::MAX;
        let mut screen_max_y = i32::MIN;

        for c in sorted {
            if c.offset_z > max_z_cutoff {
                continue;
            }

            if let Some(art) = get_art(c.item_id) {
                let px = ((c.offset_x as i32) - (c.offset_y as i32)) * 22 - (art.width as i32 / 2);
                let py = ((c.offset_x as i32) + (c.offset_y as i32)) * 22
                    - (c.offset_z as i32 * 4)
                    - (art.height as i32);

                screen_min_x = screen_min_x.min(px);
                screen_max_x = screen_max_x.max(px + art.width as i32);
                screen_min_y = screen_min_y.min(py);
                screen_max_y = screen_max_y.max(py + art.height as i32);

                draw_items.push(DrawItem { art, px, py });
            }
        }

        if draw_items.is_empty() {
            return None;
        }

        let padding = 10i32;
        let width = ((screen_max_x - screen_min_x) + padding * 2) as usize;
        let height = ((screen_max_y - screen_min_y) + padding * 2) as usize;

        if width == 0 || height == 0 || width > 4096 || height > 4096 {
            return None;
        }

        let mut canvas = vec![0u8; width * height * 4];
        let offset_x = -screen_min_x + padding;
        let offset_y = -screen_min_y + padding;

        for item in draw_items {
            let start_x = item.px + offset_x;
            let start_y = item.py + offset_y;
            blit_alpha(
                &item.art.rgba,
                item.art.width as usize,
                item.art.height as usize,
                &mut canvas,
                width,
                height,
                start_x,
                start_y,
            );
        }

        Some(ArtBitmap {
            width: width as u16,
            height: height as u16,
            rgba: canvas,
        })
    }
}

/// Blits an RGBA source image over a destination canvas with alpha transparency.
#[inline(always)]
fn blit_alpha(
    src: &[u8],
    sw: usize,
    sh: usize,
    dst: &mut [u8],
    dw: usize,
    dh: usize,
    dst_x: i32,
    dst_y: i32,
) {
    for sy in 0..sh {
        let dy = dst_y + (sy as i32);
        if dy < 0 || dy >= (dh as i32) {
            continue;
        }

        let src_row = sy * sw * 4;
        let dst_row = (dy as usize) * dw * 4;

        for sx in 0..sw {
            let dx = dst_x + (sx as i32);
            if dx < 0 || dx >= (dw as i32) {
                continue;
            }

            let s_idx = src_row + sx * 4;
            let sa = src[s_idx + 3] as u32;
            if sa == 0 {
                continue;
            }

            let d_idx = dst_row + (dx as usize) * 4;

            if sa == 255 {
                dst[d_idx..d_idx + 4].copy_from_slice(&src[s_idx..s_idx + 4]);
            } else {
                let sr = src[s_idx] as u32;
                let sg = src[s_idx + 1] as u32;
                let sb = src[s_idx + 2] as u32;

                let dr = dst[d_idx] as u32;
                let dg = dst[d_idx + 1] as u32;
                let db = dst[d_idx + 2] as u32;
                let da = dst[d_idx + 3] as u32;

                let inv_sa = 255 - sa;
                let out_a = sa + (da * inv_sa) / 255;
                if out_a > 0 {
                    dst[d_idx] = ((sr * sa + dr * da * inv_sa / 255) / out_a) as u8;
                    dst[d_idx + 1] = ((sg * sa + dg * da * inv_sa / 255) / out_a) as u8;
                    dst[d_idx + 2] = ((sb * sa + db * da * inv_sa / 255) / out_a) as u8;
                    dst[d_idx + 3] = out_a as u8;
                }
            }
        }
    }
}

/// Loads all modern UOP multi structures from MultiCollection.uop.
pub fn load_uop_multis<P: AsRef<std::path::Path>>(
    uop_path: P,
) -> std::collections::HashMap<u32, MultiStructure> {
    use flate2::read::ZlibDecoder;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    let mut map = std::collections::HashMap::new();
    let Ok(mut file) = File::open(uop_path) else {
        return map;
    };

    let mut header = [0u8; 20];
    if file.read_exact(&mut header).is_err() {
        return map;
    }

    if &header[0..4] != b"MYP\x00" {
        return map;
    }

    let mut next_block = u64::from_le_bytes([
        header[12], header[13], header[14], header[15], header[16], header[17], header[18],
        header[19],
    ]);

    struct EntryRef {
        data_offset: u64,
        compressed_len: usize,
        decompressed_len: usize,
        is_compressed: bool,
    }

    let mut entries = Vec::with_capacity(1000);

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
            let mut entry_bytes = [0u8; 34];
            if file.read_exact(&mut entry_bytes).is_err() {
                break;
            }

            let offset = u64::from_le_bytes([
                entry_bytes[0],
                entry_bytes[1],
                entry_bytes[2],
                entry_bytes[3],
                entry_bytes[4],
                entry_bytes[5],
                entry_bytes[6],
                entry_bytes[7],
            ]);
            let header_len = u32::from_le_bytes([
                entry_bytes[8],
                entry_bytes[9],
                entry_bytes[10],
                entry_bytes[11],
            ]) as u64;
            let compressed_len = u32::from_le_bytes([
                entry_bytes[12],
                entry_bytes[13],
                entry_bytes[14],
                entry_bytes[15],
            ]) as usize;
            let decompressed_len = u32::from_le_bytes([
                entry_bytes[16],
                entry_bytes[17],
                entry_bytes[18],
                entry_bytes[19],
            ]) as usize;
            let comp_flag = i16::from_le_bytes([entry_bytes[32], entry_bytes[33]]);

            if offset != 0 && compressed_len != 0 {
                entries.push(EntryRef {
                    data_offset: offset + header_len,
                    compressed_len,
                    decompressed_len,
                    is_compressed: comp_flag != 0,
                });
            }
        }
    }

    for entry in entries {
        if file.seek(SeekFrom::Start(entry.data_offset)).is_err() {
            continue;
        }

        let mut raw = vec![0u8; entry.compressed_len];
        if file.read_exact(&mut raw).is_err() {
            continue;
        }

        let payload = if entry.is_compressed {
            let mut decoder = ZlibDecoder::new(&raw[..]);
            let mut decompressed = Vec::with_capacity(entry.decompressed_len.max(0));
            if decoder.read_to_end(&mut decompressed).is_ok() {
                decompressed
            } else {
                continue;
            }
        } else {
            raw
        };

        if let Some(multi) = MultiStructure::parse_uop(&payload) {
            map.insert(multi.multi_id, multi);
        }
    }

    map
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultilistEntry {
    pub id: u32,
    pub name: String,
    pub multi_type: u8, // 0: Boat, 1: House, 2: Decoration, 3: Other
}

pub fn parse_multilist_xml(xml_content: &str) -> Vec<MultilistEntry> {
    let mut entries = Vec::new();
    for line in xml_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<Multi") {
            let id = trimmed
                .split("id=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse::<u32>().ok());
            let name = trimmed
                .split("name=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .unwrap_or("");
            let multi_type = trimmed
                .split("type=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(0);

            if let Some(i) = id {
                entries.push(MultilistEntry {
                    id: i,
                    name: if name.is_empty() {
                        format!("Multi 0x{:04X}", i)
                    } else {
                        name.to_string()
                    },
                    multi_type,
                });
            }
        }
    }
    entries
}

const EMBEDDED_MULTILIST_XML: &str = include_str!("../../../Multilist.xml");

pub fn load_multilist_entries(file_manager: &crate::FileManager) -> Vec<MultilistEntry> {
    let mut candidates = vec![
        std::path::PathBuf::from("Multilist.xml"),
        std::path::PathBuf::from("../Multilist.xml"),
        std::path::PathBuf::from("../../Multilist.xml"),
    ];

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("Multilist.xml"));
        }
    }

    for path in &candidates {
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let mut list = parse_multilist_xml(&content);
                if !list.is_empty() {
                    list.sort_by_key(|e| e.id);
                    list.dedup_by_key(|e| e.id);
                    return list;
                }
            }
        }
    }

    if let Some(p) = file_manager.get_file_path("Multilist.xml") {
        if let Ok(content) = std::fs::read_to_string(&p) {
            let mut list = parse_multilist_xml(&content);
            if !list.is_empty() {
                list.sort_by_key(|e| e.id);
                list.dedup_by_key(|e| e.id);
                return list;
            }
        }
    }

    // Default embedded fallback
    let mut list = parse_multilist_xml(EMBEDDED_MULTILIST_XML);
    list.sort_by_key(|e| e.id);
    list.dedup_by_key(|e| e.id);
    list
}

/// Decides the multi.mul record size for the whole file: if any entry length is not a multiple
/// of 12 the file uses 16-byte records; if any is not a multiple of 16 it uses 12-byte records.
/// A single entry cannot decide this (lengths divisible by 48 fit both).
pub fn detect_multi_record_size(index: &crate::FileIndex) -> usize {
    let (mut not12, mut not16) = (0usize, 0usize);
    for id in 0..index.entries.len() {
        if !index.is_valid(id) {
            continue;
        }
        let len = index.entries[id].length as usize;
        if len % 12 != 0 {
            not12 += 1;
        }
        if len % 16 != 0 {
            not16 += 1;
        }
    }
    match (not12 > 0, not16 > 0) {
        (true, false) => 16,
        (false, true) => 12,
        // fully ambiguous or inconsistent: High Seas (16) is the format of every modern client
        _ => 16,
    }
}
