//! Ultima Online World Map Reader and Minimap Renderer (map*.mul, staidx*.mul, statics*.mul).

use memmap2::Mmap;
use std::fs::File;
use std::path::PathBuf;

use crate::radar_col::RadarColManager;

#[derive(Clone, Copy, Debug)]
pub struct MapDef {
    pub map_id: u32,
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const MAP_DEFS: [MapDef; 6] = [
    MapDef {
        map_id: 0,
        name: "Felucca",
        width: 7168,
        height: 4096,
    },
    MapDef {
        map_id: 1,
        name: "Trammel",
        width: 7168,
        height: 4096,
    },
    MapDef {
        map_id: 2,
        name: "Ilshenar",
        width: 2304,
        height: 1600,
    },
    MapDef {
        map_id: 3,
        name: "Malas",
        width: 2560,
        height: 2048,
    },
    MapDef {
        map_id: 4,
        name: "Tokuno",
        width: 1448,
        height: 1448,
    },
    MapDef {
        map_id: 5,
        name: "Ter Mur",
        width: 1280,
        height: 4096,
    },
];

pub struct MapManager {
    map_mmaps: [Option<Mmap>; 6],
    staidx_mmaps: [Option<Mmap>; 6],
    statics_mmaps: [Option<Mmap>; 6],
    pub actual_widths: [u32; 6],
}

impl MapManager {
    pub fn new() -> Self {
        Self {
            map_mmaps: [None, None, None, None, None, None],
            staidx_mmaps: [None, None, None, None, None, None],
            statics_mmaps: [None, None, None, None, None, None],
            actual_widths: [7168, 7168, 2304, 2560, 1448, 1280],
        }
    }

    pub fn load_map(
        &mut self,
        map_id: usize,
        map_path: Option<PathBuf>,
        staidx_path: Option<PathBuf>,
        statics_path: Option<PathBuf>,
    ) {
        if map_id >= 6 {
            return;
        }

        if let Some(p) = map_path {
            if let Ok(file) = File::open(&p) {
                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                    let total_blocks = mmap.len() / 196;
                    let h_blocks = (MAP_DEFS[map_id].height / 8) as usize;
                    if h_blocks > 0 {
                        let w_blocks = total_blocks / h_blocks;
                        self.actual_widths[map_id] = (w_blocks * 8) as u32;
                    }
                    self.map_mmaps[map_id] = Some(mmap);
                }
            }
        }

        if let Some(p) = staidx_path {
            if let Ok(file) = File::open(&p) {
                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                    self.staidx_mmaps[map_id] = Some(mmap);
                }
            }
        }

        if let Some(p) = statics_path {
            if let Ok(file) = File::open(&p) {
                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                    self.statics_mmaps[map_id] = Some(mmap);
                }
            }
        }
    }

    /// Renders a fast radar minimap image for a given bounding box of tiles.
    /// Returns RGBA buffer of dimensions (num_blocks_x * 8, num_blocks_y * 8).
    pub fn render_minimap(
        &self,
        map_id: usize,
        start_bx: u32,
        start_by: u32,
        num_blocks_x: u32,
        num_blocks_y: u32,
        radar: &RadarColManager,
        show_statics: bool,
    ) -> Option<(u32, u32, Vec<u8>)> {
        if map_id >= 6 {
            return None;
        }

        let map_mmap = self.map_mmaps[map_id].as_ref()?;
        let total_w = self.actual_widths[map_id];
        let total_h = MAP_DEFS[map_id].height;

        let max_bx = total_w / 8;
        let max_by = total_h / 8;

        let out_w = num_blocks_x * 8;
        let out_h = num_blocks_y * 8;
        let mut rgba = vec![255u8; (out_w * out_h * 4) as usize];

        let staidx = self.staidx_mmaps[map_id].as_ref();
        let statics = self.statics_mmaps[map_id].as_ref();

        for bx_idx in 0..num_blocks_x {
            let cur_bx = start_bx + bx_idx;
            if cur_bx >= max_bx {
                continue;
            }

            for by_idx in 0..num_blocks_y {
                let cur_by = start_by + by_idx;
                if cur_by >= max_by {
                    continue;
                }

                let block_offset = (((cur_bx * max_by) + cur_by) as usize) * 196;
                if block_offset + 196 <= map_mmap.len() {
                    let tile_bytes = &map_mmap[block_offset + 4..block_offset + 196];
                    let base_x = bx_idx * 8;
                    let base_y = by_idx * 8;

                    for i in 0..64 {
                        let t_off = i * 3;
                        let tile_id =
                            u16::from_le_bytes([tile_bytes[t_off], tile_bytes[t_off + 1]]) as usize;
                        let z = tile_bytes[t_off + 2] as i8 as i32;

                        let col = radar.get_land_rgba(tile_id);
                        let shade = z.clamp(-30, 30);
                        let r = (col[0] as i32 + shade).clamp(0, 255) as u8;
                        let g = (col[1] as i32 + shade).clamp(0, 255) as u8;
                        let b = (col[2] as i32 + shade).clamp(0, 255) as u8;

                        let px = base_x + (i as u32 % 8);
                        let py = base_y + (i as u32 / 8);
                        let dst_idx = ((py * out_w + px) * 4) as usize;
                        rgba[dst_idx] = r;
                        rgba[dst_idx + 1] = g;
                        rgba[dst_idx + 2] = b;
                        rgba[dst_idx + 3] = 255;
                    }
                }

                // Render statics on top
                if show_statics {
                    if let (Some(idx_map), Some(stat_map)) = (staidx, statics) {
                        let idx_offset = (((cur_bx * max_by) + cur_by) as usize) * 12;
                        if idx_offset + 12 <= idx_map.len() {
                            let lookup = i32::from_le_bytes([
                                idx_map[idx_offset],
                                idx_map[idx_offset + 1],
                                idx_map[idx_offset + 2],
                                idx_map[idx_offset + 3],
                            ]);
                            let length = i32::from_le_bytes([
                                idx_map[idx_offset + 4],
                                idx_map[idx_offset + 5],
                                idx_map[idx_offset + 6],
                                idx_map[idx_offset + 7],
                            ]);

                            if lookup >= 0 && length > 0 {
                                let l = lookup as usize;
                                let len = length as usize;
                                if l + len <= stat_map.len() {
                                    let raw_stat = &stat_map[l..l + len];
                                    let count = len / 7;
                                    let base_x = bx_idx * 8;
                                    let base_y = by_idx * 8;

                                    for s_i in 0..count {
                                        let s_off = s_i * 7;
                                        let item_id = u16::from_le_bytes([
                                            raw_stat[s_off],
                                            raw_stat[s_off + 1],
                                        ])
                                            as usize;
                                        let sx = raw_stat[s_off + 2] & 0x7;
                                        let sy = raw_stat[s_off + 3] & 0x7;

                                        let col = radar.get_item_rgba(item_id);
                                        if col[3] > 0 {
                                            let px = base_x + sx as u32;
                                            let py = base_y + sy as u32;
                                            if px < out_w && py < out_h {
                                                let dst_idx = ((py * out_w + px) * 4) as usize;
                                                rgba[dst_idx] = col[0];
                                                rgba[dst_idx + 1] = col[1];
                                                rgba[dst_idx + 2] = col[2];
                                                rgba[dst_idx + 3] = 255;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Some((out_w, out_h, rgba))
    }
}
