//! Art decoders for Static Items and Land Tiles.

use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::color::color16_to_rgba;
use crate::hash::hash_file_name;

#[derive(Clone, Debug)]
pub struct ArtBitmap {
    pub width: u16,
    pub height: u16,
    /// RGBA pixels (width * height * 4)
    pub rgba: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct EcArtEntry {
    pub file_path: PathBuf,
    pub data_offset: u64,
    pub compressed_len: usize,
    pub decompressed_len: usize,
    pub is_compressed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct EcArtIndex {
    pub entries: HashMap<u64, EcArtEntry>,
}

impl EcArtIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_directory<P: AsRef<Path>>(&mut self, dir: P) {
        let dir_ref = dir.as_ref();
        let target_files = ["LegacyTexture.uop", "tileart.uop"];
        for fname in target_files {
            let p = dir_ref.join(fname);
            if p.is_file() {
                self.load_uop_file(&p);
            } else if let Ok(entries) = std::fs::read_dir(dir_ref) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(f) = path.file_name().and_then(|s| s.to_str()) {
                        if f.eq_ignore_ascii_case(fname) {
                            self.load_uop_file(&path);
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn load_uop_file(&mut self, path: &Path) {
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
                        EcArtEntry {
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

    pub fn get_static(&self, item_id: u32) -> Option<ArtBitmap> {
        let h = hash_file_name(&format!("build/tileartlegacy/{:08}.dds", item_id));
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

        if let Ok(dyn_img) = image::load_from_memory(&payload) {
            let rgba_img = dyn_img.to_rgba8();
            let (w, h) = (rgba_img.width(), rgba_img.height());
            if w > 0 && h > 0 && w <= 2048 && h <= 2048 {
                return Some(ArtBitmap {
                    width: w as u16,
                    height: h as u16,
                    rgba: rgba_img.into_raw(),
                });
            }
        }
        None
    }
}

/// Decodes static item art (Art.mul / Artidx.mul starting at ID 0x4000).
pub fn decode_static(raw_data: &[u8]) -> Option<ArtBitmap> {
    if raw_data.len() < 8 {
        return None;
    }

    let words: &[u16] =
        unsafe { std::slice::from_raw_parts(raw_data.as_ptr() as *const u16, raw_data.len() / 2) };

    let width = words[2];
    let height = words[3];

    if width == 0 || height == 0 || width > 1024 || height > 1024 {
        return None;
    }

    let word_count = words.len();
    if word_count < (4 + height as usize) {
        return None;
    }

    let w = width as usize;
    let h = height as usize;
    let mut rgba = vec![0u8; w * h * 4];

    let lookups = &words[4..4 + h];

    for y in 0..h {
        let mut count = h + 4 + (lookups[y] as usize);
        let mut x = 0usize;
        let row_offset = y * w * 4;

        while count + 1 < word_count {
            let x_offset = words[count] as usize;
            let x_run = words[count + 1] as usize;
            count += 2;

            if (x_offset | x_run) == 0 {
                break;
            }

            x += x_offset;
            if x + x_run > w {
                break;
            }

            for _ in 0..x_run {
                if count >= word_count {
                    break;
                }
                let color_raw = words[count];
                count += 1;

                let px_idx = row_offset + x * 4;
                let c = color16_to_rgba(color_raw ^ 0x8000);
                rgba[px_idx..px_idx + 4].copy_from_slice(&c);
                x += 1;
            }
        }
    }

    Some(ArtBitmap {
        width,
        height,
        rgba,
    })
}

/// Decodes a 44x44 diamond land tile (Art.mul 0..0x3FFF).
pub fn decode_land(raw_data: &[u8]) -> Option<ArtBitmap> {
    if raw_data.len() < 2048 {
        return None;
    }

    let words: &[u16] =
        unsafe { std::slice::from_raw_parts(raw_data.as_ptr() as *const u16, raw_data.len() / 2) };

    let mut rgba = vec![0u8; 44 * 44 * 4];
    let mut word_idx = 0usize;
    let mut x_offset = 21isize;
    let mut x_run = 2isize;

    // Top half (rows 0 to 21)
    for y in 0..22 {
        let row_start = y * 44 * 4;
        for x in x_offset..(x_offset + x_run) {
            if word_idx < 1024 && x >= 0 && x < 44 {
                let c = color16_to_rgba(words[word_idx] | 0x8000);
                word_idx += 1;
                let px_idx = row_start + (x as usize) * 4;
                rgba[px_idx..px_idx + 4].copy_from_slice(&c);
            }
        }
        x_offset -= 1;
        x_run += 2;
    }

    x_offset = 0;
    x_run = 44;

    // Bottom half (rows 22 to 43)
    for y in 22..44 {
        let row_start = y * 44 * 4;
        for x in x_offset..(x_offset + x_run) {
            if word_idx < 1024 && x >= 0 && x < 44 {
                let c = color16_to_rgba(words[word_idx] | 0x8000);
                word_idx += 1;
                let px_idx = row_start + (x as usize) * 4;
                rgba[px_idx..px_idx + 4].copy_from_slice(&c);
            }
        }
        x_offset += 1;
        x_run -= 2;
    }

    Some(ArtBitmap {
        width: 44,
        height: 44,
        rgba,
    })
}
