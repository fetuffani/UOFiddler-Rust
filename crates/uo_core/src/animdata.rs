//! UO Animdata (animdata.mul): tile animation timing and frame sequences.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct AnimDataEntry {
    pub item_id: u32,
    pub frames: [i8; 64],
    pub unk: u8,
    pub frame_count: u8,
    pub frame_interval: u8,
    pub frame_start: u8,
}

pub struct AnimDataManager {
    pub entries: HashMap<u32, AnimDataEntry>,
}

impl AnimDataManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let chunk_size = 4 + 8 * 68;
        let num_chunks = data.len() / chunk_size;
        let mut entries = HashMap::new();
        let mut offset = 0usize;
        let mut tile_id = 0u32;

        for _ in 0..num_chunks {
            // 4-byte chunk header
            offset += 4;

            for _ in 0..8 {
                if offset + 68 > data.len() {
                    break;
                }

                let mut frames = [0i8; 64];
                for k in 0..64 {
                    frames[k] = data[offset + k] as i8;
                }

                let unk = data[offset + 64];
                let frame_count = data[offset + 65];
                let frame_interval = data[offset + 66];
                let frame_start = data[offset + 67];
                offset += 68;

                if frame_count > 0 {
                    entries.insert(
                        tile_id,
                        AnimDataEntry {
                            item_id: tile_id,
                            frames,
                            unk,
                            frame_count,
                            frame_interval,
                            frame_start,
                        },
                    );
                }
                tile_id += 1;
            }
        }

        Ok(Self { entries })
    }
}
