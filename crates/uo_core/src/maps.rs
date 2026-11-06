//! Ultima Online World Map Reader and Minimap Renderer (map*.mul, staidx*.mul, statics*.mul).

use memmap2::Mmap;
use std::collections::HashMap;
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

/// Patch data (mapdifN / mapdiflN / stadifN / stadifiN / stadiflN) that replaces whole 8x8 blocks.
#[derive(Default)]
struct MapDiff {
    /// mapdifN.mul: consecutive 196-byte blocks
    land: Option<Mmap>,
    /// block number -> position inside `land`
    land_blocks: HashMap<u32, usize>,
    /// stadifN.mul (static records) + stadifiN.mul (12-byte index entries)
    stat_data: Option<Mmap>,
    stat_idx: Option<Mmap>,
    stat_blocks: HashMap<u32, usize>,
}

pub struct MapManager {
    map_mmaps: [Option<Mmap>; 6],
    staidx_mmaps: [Option<Mmap>; 6],
    statics_mmaps: [Option<Mmap>; 6],
    diffs: [MapDiff; 6],
    pub actual_widths: [u32; 6],
}

fn mmap_of(p: &Option<PathBuf>) -> Option<Mmap> {
    let file = File::open(p.as_ref()?).ok()?;
    unsafe { Mmap::map(&file) }.ok()
}

fn stat_slice<'a>(idx: &Mmap, data: &'a Mmap, entry: usize) -> Option<&'a [u8]> {
    let o = entry * 12;
    let e = idx.get(o..o + 12)?;
    let lookup = i32::from_le_bytes([e[0], e[1], e[2], e[3]]);
    let length = i32::from_le_bytes([e[4], e[5], e[6], e[7]]);
    if lookup < 0 || length <= 0 {
        return Some(&[]);
    }
    data.get(lookup as usize..lookup as usize + length as usize)
}

fn block_list(m: &Option<Mmap>) -> HashMap<u32, usize> {
    m.as_ref()
        .map(|m| {
            m.chunks_exact(4)
                .enumerate()
                .map(|(i, c)| (u32::from_le_bytes([c[0], c[1], c[2], c[3]]), i))
                .collect()
        })
        .unwrap_or_default()
}

impl MapManager {
    pub fn new() -> Self {
        Self {
            map_mmaps: [None, None, None, None, None, None],
            staidx_mmaps: [None, None, None, None, None, None],
            statics_mmaps: [None, None, None, None, None, None],
            diffs: Default::default(),
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

    /// Loads the optional patch files of a facet (mapdif / stadif). Missing files are fine.
    pub fn load_diff(
        &mut self,
        map_id: usize,
        mapdif: Option<PathBuf>,
        mapdifl: Option<PathBuf>,
        stadif: Option<PathBuf>,
        stadifi: Option<PathBuf>,
        stadifl: Option<PathBuf>,
    ) {
        if map_id >= 6 {
            return;
        }
        let d = &mut self.diffs[map_id];
        d.land = mmap_of(&mapdif);
        d.land_blocks = if d.land.is_some() { block_list(&mmap_of(&mapdifl)) } else { HashMap::new() };
        d.stat_data = mmap_of(&stadif);
        d.stat_idx = mmap_of(&stadifi);
        d.stat_blocks = if d.stat_data.is_some() && d.stat_idx.is_some() {
            block_list(&mmap_of(&stadifl))
        } else {
            HashMap::new()
        };
    }

    /// Number of blocks patched by the diff files: (land blocks, static blocks).
    pub fn diff_counts(&self, map_id: usize) -> (usize, usize) {
        self.diffs.get(map_id).map_or((0, 0), |d| (d.land_blocks.len(), d.stat_blocks.len()))
    }

    pub fn has_map(&self, map_id: usize) -> bool {
        self.map_mmaps.get(map_id).map_or(false, |m| m.is_some())
    }

    /// 64 land cells (3 bytes each: u16 tile, i8 z) of an 8x8 block, optionally patched by mapdif.
    pub fn land_block(&self, map_id: usize, bx: u32, by: u32, use_diff: bool) -> Option<&[u8]> {
        let map = self.map_mmaps.get(map_id)?.as_ref()?;
        let max_by = MAP_DEFS[map_id].height / 8;
        let number = bx * max_by + by;
        if use_diff {
            let d = &self.diffs[map_id];
            if let (Some(pos), Some(land)) = (d.land_blocks.get(&number), d.land.as_ref()) {
                let off = pos * 196;
                if off + 196 <= land.len() {
                    return Some(&land[off + 4..off + 196]);
                }
            }
        }
        let off = number as usize * 196;
        (off + 196 <= map.len()).then(|| &map[off + 4..off + 196])
    }

    /// Raw static records (7 bytes each) of a block, optionally patched by stadif.
    pub fn static_block(&self, map_id: usize, bx: u32, by: u32, use_diff: bool) -> Option<&[u8]> {
        let max_by = MAP_DEFS[map_id].height / 8;
        let number = bx * max_by + by;
        let slice_of = stat_slice;
        if use_diff {
            let d = &self.diffs[map_id];
            if let (Some(pos), Some(idx), Some(data)) = (d.stat_blocks.get(&number), d.stat_idx.as_ref(), d.stat_data.as_ref()) {
                if let Some(s) = slice_of(idx, data, *pos) {
                    return Some(s);
                }
            }
        }
        let idx = self.staidx_mmaps.get(map_id)?.as_ref()?;
        let data = self.statics_mmaps.get(map_id)?.as_ref()?;
        slice_of(idx, data, number as usize)
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
        use_diff: bool,
    ) -> Option<(u32, u32, Vec<u8>)> {
        if map_id >= 6 || !self.has_map(map_id) {
            return None;
        }
        let max_bx = self.actual_widths[map_id] / 8;
        let max_by = MAP_DEFS[map_id].height / 8;
        let out_w = num_blocks_x * 8;
        let out_h = num_blocks_y * 8;
        let mut rgba = vec![255u8; (out_w * out_h * 4) as usize];

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
                let base_x = bx_idx * 8;
                let base_y = by_idx * 8;

                if let Some(tile_bytes) = self.land_block(map_id, cur_bx, cur_by, use_diff) {
                    for i in 0..64 {
                        let t_off = i * 3;
                        let tile_id = u16::from_le_bytes([tile_bytes[t_off], tile_bytes[t_off + 1]]) as usize;
                        let z = tile_bytes[t_off + 2] as i8 as i32;
                        let col = radar.get_land_rgba(tile_id);
                        let shade = z.clamp(-30, 30);
                        let px = base_x + (i as u32 % 8);
                        let py = base_y + (i as u32 / 8);
                        let dst = ((py * out_w + px) * 4) as usize;
                        rgba[dst] = (col[0] as i32 + shade).clamp(0, 255) as u8;
                        rgba[dst + 1] = (col[1] as i32 + shade).clamp(0, 255) as u8;
                        rgba[dst + 2] = (col[2] as i32 + shade).clamp(0, 255) as u8;
                        rgba[dst + 3] = 255;
                    }
                }

                if show_statics {
                    if let Some(raw) = self.static_block(map_id, cur_bx, cur_by, use_diff) {
                        for rec in raw.chunks_exact(7) {
                            let item_id = u16::from_le_bytes([rec[0], rec[1]]) as usize;
                            let col = radar.get_item_rgba(item_id);
                            if col[3] > 0 {
                                let px = base_x + (rec[2] & 7) as u32;
                                let py = base_y + (rec[3] & 7) as u32;
                                let dst = ((py * out_w + px) * 4) as usize;
                                rgba[dst] = col[0];
                                rgba[dst + 1] = col[1];
                                rgba[dst + 2] = col[2];
                                rgba[dst + 3] = 255;
                            }
                        }
                    }
                }
            }
        }
        Some((out_w, out_h, rgba))
    }
}
