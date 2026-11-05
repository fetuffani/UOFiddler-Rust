//! UO Speech keywords reader (speech.mul).

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SpeechEntry {
    pub entry_id: i16,
    pub keyword: String,
    pub order: usize,
}

pub struct SpeechManager {
    pub entries: Vec<SpeechEntry>,
}

impl SpeechManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let mut entries = Vec::new();
        let mut offset = 0usize;
        let mut order = 0usize;

        while offset + 4 <= data.len() {
            let entry_id = i16::from_be_bytes([data[offset], data[offset + 1]]);
            let length = i16::from_be_bytes([data[offset + 2], data[offset + 3]]) as usize;
            offset += 4;

            if length > 256 || offset + length > data.len() {
                break;
            }

            let raw_kw = &data[offset..offset + length];
            offset += length;

            let keyword = String::from_utf8_lossy(raw_kw).to_string();
            entries.push(SpeechEntry {
                entry_id,
                keyword,
                order,
            });
            order += 1;
        }

        Ok(Self { entries })
    }
}
