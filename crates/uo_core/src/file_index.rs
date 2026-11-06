//! Generic binary file index reader for Ultima Online files (.idx + .mul or .uop).

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

use crate::hash::hash_file_name;

#[derive(Error, Debug)]
pub enum FileIndexError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid UOP magic signature")]
    InvalidUopMagic,
    #[error("Index out of bounds")]
    OutOfBounds,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IndexEntry {
    pub lookup: i64,
    pub length: i32,
    pub extra: i32,
    pub decompressed_length: i32,
    pub compression: u16, // 0 = uncompressed, 1 = zlib
}

impl IndexEntry {
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.lookup >= 0 && self.length > 0
    }
}

pub struct FileIndex {
    pub entries: Vec<IndexEntry>,
    mmap: Option<Mmap>,
    is_uop: bool,
}

impl FileIndex {
    /// Loads from separate IDX and MUL files.
    pub fn from_mul_idx<P: AsRef<Path>>(
        idx_path: P,
        mul_path: P,
        max_entries: usize,
    ) -> Result<Self, FileIndexError> {
        let idx_file = File::open(idx_path)?;
        let idx_size = idx_file.metadata()?.len();
        let count = ((idx_size / 12) as usize).min(max_entries);

        let mut reader = std::io::BufReader::new(idx_file);
        let mut entries = vec![IndexEntry::default(); max_entries];

        for i in 0..count {
            let lookup = reader.read_i32::<LittleEndian>()? as i64;
            let length = reader.read_i32::<LittleEndian>()?;
            let extra = reader.read_i32::<LittleEndian>()?;

            entries[i] = IndexEntry {
                lookup,
                length,
                extra,
                decompressed_length: length,
                compression: 0,
            };
        }

        let mul_file = File::open(mul_path)?;
        let mmap = unsafe { Mmap::map(&mul_file)? };

        Ok(Self {
            entries,
            mmap: Some(mmap),
            is_uop: false,
        })
    }

    /// Loads an archive from a UOP file, mapping entries by pattern `build/{basename}/{i:08d}{uop_ext}`.
    pub fn from_uop<P: AsRef<Path>>(
        uop_path: P,
        uop_ext: &str,
        max_entries: usize,
    ) -> Result<Self, FileIndexError> {
        let path_ref = uop_path.as_ref();
        let stem = path_ref
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mut hashes = HashMap::with_capacity(max_entries);
        for i in 0..max_entries {
            let entry_name = format!("build/{}/{:08}{}", stem, i, uop_ext);
            let h = hash_file_name(&entry_name);
            hashes.insert(h, i);
        }

        let mut file = File::open(path_ref)?;
        let magic = file.read_u32::<LittleEndian>()?;
        if magic != 0x5059_4D {
            return Err(FileIndexError::InvalidUopMagic);
        }

        let _ver = file.read_u32::<LittleEndian>()?;
        let _sig = file.read_u32::<LittleEndian>()?;
        let mut next_block = file.read_u64::<LittleEndian>()?;
        let _block_size = file.read_u32::<LittleEndian>()?;
        let _count = file.read_u32::<LittleEndian>()?;

        let mut entries = vec![IndexEntry::default(); max_entries];

        while next_block != 0 {
            file.seek(SeekFrom::Start(next_block))?;
            let files_count = file.read_u32::<LittleEndian>()?;
            next_block = file.read_u64::<LittleEndian>()?;

            for _ in 0..files_count {
                let offset = file.read_u64::<LittleEndian>()?;
                let h_len = file.read_u32::<LittleEndian>()?;
                let c_len = file.read_u32::<LittleEndian>()?;
                let d_len = file.read_u32::<LittleEndian>()?;
                let h = file.read_u64::<LittleEndian>()?;
                let _d_hash = file.read_u32::<LittleEndian>()?;
                let flag = file.read_u16::<LittleEndian>()?;

                if offset == 0 {
                    continue;
                }

                if let Some(&idx) = hashes.get(&h) {
                    if idx < max_entries {
                        entries[idx] = IndexEntry {
                            lookup: (offset + h_len as u64) as i64,
                            length: c_len as i32,
                            extra: 0,
                            decompressed_length: d_len as i32,
                            compression: flag,
                        };
                    }
                }
            }
        }

        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self {
            entries,
            mmap: Some(mmap),
            is_uop: true,
        })
    }

    #[inline(always)]
    pub fn is_uop(&self) -> bool {
        self.is_uop
    }

    #[inline(always)]
    pub fn is_valid(&self, index: usize) -> bool {
        if index < self.entries.len() {
            self.entries[index].is_valid()
        } else {
            false
        }
    }

    /// Reads the `(width, height)` of a gump entry without decoding the full bitmap.
    pub fn gump_size(&self, index: usize) -> Option<(u32, u32)> {
        if !self.is_valid(index) {
            return None;
        }
        let entry = self.entries[index];
        if !self.is_uop && entry.compression == 0 {
            return Some((((entry.extra >> 16) & 0xFFFF) as u32, (entry.extra & 0xFFFF) as u32));
        }
        let mmap = self.mmap.as_ref()?;
        let start = entry.lookup as usize;
        let end = start.checked_add(entry.length as usize)?;
        if end > mmap.len() {
            return None;
        }
        let slice = &mmap[start..end];
        let mut hdr = [0u8; 8];
        match entry.compression {
            0 => {
                if slice.len() < 8 {
                    return None;
                }
                hdr.copy_from_slice(&slice[..8]);
            }
            1 => {
                let mut decoder = ZlibDecoder::new(slice);
                decoder.read_exact(&mut hdr).ok()?;
            }
            _ => {
                let data = self.get_data(index)?;
                if data.len() < 8 {
                    return None;
                }
                hdr.copy_from_slice(&data[..8]);
            }
        }
        Some((
            u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]),
            u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]),
        ))
    }

    /// Returns the raw (decompressed if needed) byte payload for index.
    pub fn get_data(&self, index: usize) -> Option<Vec<u8>> {
        if !self.is_valid(index) {
            return None;
        }

        let entry = self.entries[index];
        let mmap = self.mmap.as_ref()?;

        let start = entry.lookup as usize;
        let end = start.checked_add(entry.length as usize)?;
        if end > mmap.len() {
            return None;
        }

        let slice = &mmap[start..end];

        match entry.compression {
            1 | 3 => {
                let mut decoder = ZlibDecoder::new(slice);
                let mut decompressed =
                    Vec::with_capacity(entry.decompressed_length.max(0) as usize);
                if decoder.read_to_end(&mut decompressed).is_ok() {
                    if entry.compression == 3 {
                        crate::mythic::decompress_mythic(&decompressed)
                    } else {
                        Some(decompressed)
                    }
                } else {
                    None
                }
            }
            _ => Some(slice.to_vec()),
        }
    }
}
