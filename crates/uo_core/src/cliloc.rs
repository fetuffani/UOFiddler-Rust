//! Cliloc localization table reader (e.g. cliloc.enu).

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ClilocEntry {
    pub number: i32,
    pub flag: u8,
    pub text: String,
}

pub struct ClilocManager {
    pub entries: Vec<ClilocEntry>,
}

impl ClilocManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        if data.len() < 6 {
            return Ok(Self {
                entries: Vec::new(),
            });
        }

        // Check if compressed with Mythic compression
        if !(data[0] == 0 || data[1] == 0 || data[2] == 0 || data[3] == 0) {
            if let Some(decompressed) = crate::mythic::decompress_mythic(&data) {
                data = decompressed;
            }
        }

        if data.len() < 6 {
            return Ok(Self {
                entries: Vec::new(),
            });
        }
        let mut cursor = 6usize;
        let mut entries = Vec::new();

        while cursor + 7 <= data.len() {
            let number = i32::from_le_bytes([
                data[cursor],
                data[cursor + 1],
                data[cursor + 2],
                data[cursor + 3],
            ]);
            let flag = data[cursor + 4];
            let text_len = u16::from_le_bytes([data[cursor + 5], data[cursor + 6]]) as usize;
            cursor += 7;

            if cursor + text_len > data.len() {
                break;
            }

            let raw_text = &data[cursor..cursor + text_len];
            cursor += text_len;

            let text = String::from_utf8_lossy(raw_text).to_string();
            entries.push(ClilocEntry { number, flag, text });
        }

        Ok(Self { entries })
    }
}
