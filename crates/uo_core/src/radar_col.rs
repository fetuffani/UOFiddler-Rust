//! RadarCol reader for radarcol.mul (minimap color lookup table).

use crate::color::color16_to_rgba;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct RadarColManager {
    pub colors: Vec<u16>,
}

impl RadarColManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let word_count = data.len() / 2;
        let words: &[u16] =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, word_count) };

        let mut colors = words.to_vec();
        if colors.len() < 0x14000 {
            colors.resize(0x14000, 0);
        }

        Ok(Self { colors })
    }

    pub fn get_land_rgba(&self, index: usize) -> [u8; 4] {
        let idx = index & 0x3FFF;
        if idx < self.colors.len() {
            color16_to_rgba(self.colors[idx] | 0x8000)
        } else {
            [0, 0, 0, 0]
        }
    }

    pub fn get_land_color(&self, index: usize) -> u16 {
        let idx = index & 0x3FFF;
        if idx < self.colors.len() {
            self.colors[idx]
        } else {
            0
        }
    }

    pub fn get_item_color(&self, index: usize) -> u16 {
        let idx = index + 0x4000;
        if idx < self.colors.len() {
            self.colors[idx]
        } else {
            0
        }
    }

    pub fn get_item_rgba(&self, index: usize) -> [u8; 4] {
        let idx = index + 0x4000;
        if idx < self.colors.len() {
            color16_to_rgba(self.colors[idx] | 0x8000)
        } else {
            [0, 0, 0, 0]
        }
    }
}
