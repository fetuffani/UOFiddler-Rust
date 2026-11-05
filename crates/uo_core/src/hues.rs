//! Hues reader for hues.mul (3,000 color palettes with 32-color gradient ranges and names).

use crate::color::color16_to_u32;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuesError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
pub struct Hue {
    pub index: u32,
    pub colors: [u16; 32],
    pub table_start: u16,
    pub table_end: u16,
    pub name: String,
}

impl Hue {
    pub fn get_rgba_palette(&self) -> [u32; 32] {
        let mut pal = [0u32; 32];
        for i in 0..32 {
            pal[i] = color16_to_u32(self.colors[i] | 0x8000);
        }
        pal
    }
}

pub struct HuesManager {
    pub hues: Vec<Hue>,
}

impl HuesManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, HuesError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut hues = Vec::with_capacity(3000);
        let mut index = 0u32;
        let mut name_buf = [0u8; 20];

        // 375 groups of 8 hues = 3000 hues
        for _ in 0..375 {
            let _header = match reader.read_u32::<LittleEndian>() {
                Ok(h) => h,
                Err(_) => break,
            };

            for _ in 0..8 {
                let mut colors = [0u16; 32];
                for c in &mut colors {
                    *c = reader.read_u16::<LittleEndian>().unwrap_or(0);
                }

                let table_start = reader.read_u16::<LittleEndian>().unwrap_or(0);
                let table_end = reader.read_u16::<LittleEndian>().unwrap_or(0);
                let _ = reader.read_exact(&mut name_buf);

                let len = name_buf
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(name_buf.len());
                let name = String::from_utf8_lossy(&name_buf[..len]).to_string();

                hues.push(Hue {
                    index,
                    colors,
                    table_start,
                    table_end,
                    name,
                });
                index += 1;
            }
        }

        Ok(Self { hues })
    }
}
