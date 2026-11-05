//! TileData reader for tiledata.mul (Land & Static Item flags, dimensions, and names).

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TileDataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid tiledata format")]
    InvalidFormat,
}

#[derive(Clone, Debug, Default)]
pub struct LandTileData {
    pub flags: u64,
    pub texture_id: u16,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ItemTileData {
    pub flags: u64,
    pub weight: u8,
    pub quality: u8,
    pub misc_data: u16,
    pub unk2: u8,
    pub quantity: u8,
    pub anim_id: u16,
    pub unk3: u8,
    pub hue: u8,
    pub stacking_offset: u8,
    pub value: u8,
    pub height: u8,
    pub name: String,
}

pub struct TileDataManager {
    pub land_tiles: Vec<LandTileData>,
    pub item_tiles: Vec<ItemTileData>,
    pub is_high_seas: bool,
}

impl TileDataManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, TileDataError> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let mut reader = BufReader::new(file);

        // Detect High Seas format by file size
        // Classic (0x4000 items): ~1.6MB; Classic (0x10000 items): 2,861,056 bytes
        // High Seas (0x10000 items): 493,568 (land) + 2048 * 1316 (items) = 3,188,736 bytes
        let is_high_seas = file_size >= 3_188_736;

        let mut land_tiles = Vec::with_capacity(0x4000);
        let mut name_buf = [0u8; 20];

        // 512 groups of 32 land tiles
        for _ in 0..512 {
            let _header = reader.read_u32::<LittleEndian>()?;
            for _ in 0..32 {
                let flags = if is_high_seas {
                    reader.read_u64::<LittleEndian>()?
                } else {
                    reader.read_u32::<LittleEndian>()? as u64
                };
                let texture_id = reader.read_u16::<LittleEndian>()?;
                reader.read_exact(&mut name_buf)?;

                let name = extract_c_string(&name_buf);
                land_tiles.push(LandTileData {
                    flags,
                    texture_id,
                    name,
                });
            }
        }

        // Remaining blocks are item tiles
        let mut item_tiles = Vec::with_capacity(0x10000);

        while let Ok(_header) = reader.read_u32::<LittleEndian>() {
            for _ in 0..32 {
                let flags = if is_high_seas {
                    if let Ok(f) = reader.read_u64::<LittleEndian>() {
                        f
                    } else {
                        break;
                    }
                } else {
                    if let Ok(f) = reader.read_u32::<LittleEndian>() {
                        f as u64
                    } else {
                        break;
                    }
                };

                let weight = reader.read_u8().unwrap_or(0);
                let quality = reader.read_u8().unwrap_or(0);
                let misc_data = reader.read_u16::<LittleEndian>().unwrap_or(0);
                let unk2 = reader.read_u8().unwrap_or(0);
                let quantity = reader.read_u8().unwrap_or(0);
                let anim_id = reader.read_u16::<LittleEndian>().unwrap_or(0);
                let unk3 = reader.read_u8().unwrap_or(0);
                let hue = reader.read_u8().unwrap_or(0);
                let stacking_offset = reader.read_u8().unwrap_or(0);
                let value = reader.read_u8().unwrap_or(0);
                let height = reader.read_u8().unwrap_or(0);

                if reader.read_exact(&mut name_buf).is_err() {
                    break;
                }
                let name = extract_c_string(&name_buf);

                item_tiles.push(ItemTileData {
                    flags,
                    weight,
                    quality,
                    misc_data,
                    unk2,
                    quantity,
                    anim_id,
                    unk3,
                    hue,
                    stacking_offset,
                    value,
                    height,
                    name,
                });

                if item_tiles.len() >= 0x10000 {
                    break;
                }
            }
            if item_tiles.len() >= 0x10000 {
                break;
            }
        }

        Ok(Self {
            land_tiles,
            item_tiles,
            is_high_seas,
        })
    }
}

fn extract_c_string(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}
