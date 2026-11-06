//! Software isometric "live" renderer of the world map using the real art (land tiles, stretched
//! textures on slopes, statics), in the style of the game client. Output is an RGBA buffer.

use std::collections::HashMap;

use crate::art::{decode_land, decode_static, ArtBitmap};
use crate::file_index::FileIndex;
use crate::maps::{MapManager, MAP_DEFS};
use crate::textures::{decode_texture, TextureBitmap};
use crate::tiledata::TileDataManager;

pub struct LiveContext<'a> {
    pub map: &'a MapManager,
    pub art: &'a FileIndex,
    pub textures: Option<&'a FileIndex>,
    pub tiledata: Option<&'a TileDataManager>,
}

#[derive(Clone, Copy, Debug)]
pub struct LiveOptions {
    pub show_statics: bool,
    pub use_diff: bool,
    /// Statics above this z are hidden (127 = show everything); useful to peel off roofs.
    pub max_z: i32,
    /// Stretch textures over sloped land (as the client does) instead of drawing flat art.
    pub textured_slopes: bool,
}

impl Default for LiveOptions {
    fn default() -> Self {
        Self { show_statics: true, use_diff: true, max_z: 127, textured_slopes: true }
    }
}

#[derive(Default)]
pub struct LiveRenderer {
    land: HashMap<u16, Option<ArtBitmap>>,
    stat: HashMap<u16, Option<ArtBitmap>>,
    tex: HashMap<u16, Option<TextureBitmap>>,
}

const BG: [u8; 4] = [15, 20, 18, 255];

fn blit(buf: &mut [u8], bw: i32, bh: i32, bmp: &ArtBitmap, x0: i32, y0: i32) {
    let (w, h) = (bmp.width as i32, bmp.height as i32);
    for y in 0..h {
        let dy = y0 + y;
        if dy < 0 || dy >= bh {
            continue;
        }
        for x in 0..w {
            let dx = x0 + x;
            if dx < 0 || dx >= bw {
                continue;
            }
            let s = ((y * w + x) * 4) as usize;
            if bmp.rgba[s + 3] == 0 {
                continue;
            }
            let d = ((dy * bw + dx) * 4) as usize;
            buf[d..d + 3].copy_from_slice(&bmp.rgba[s..s + 3]);
            buf[d + 3] = 255;
        }
    }
}

/// Fills the triangle (a,b,c) sampling `tex` with per-vertex UVs.
fn textured_triangle(buf: &mut [u8], bw: i32, bh: i32, tex: &TextureBitmap, p: [(f32, f32); 3], uv: [(f32, f32); 3]) {
    let min_x = p.iter().map(|v| v.0).fold(f32::MAX, f32::min).floor().max(0.0) as i32;
    let max_x = p.iter().map(|v| v.0).fold(f32::MIN, f32::max).ceil().min(bw as f32 - 1.0) as i32;
    let min_y = p.iter().map(|v| v.1).fold(f32::MAX, f32::min).floor().max(0.0) as i32;
    let max_y = p.iter().map(|v| v.1).fold(f32::MIN, f32::max).ceil().min(bh as f32 - 1.0) as i32;
    let det = (p[1].1 - p[2].1) * (p[0].0 - p[2].0) + (p[2].0 - p[1].0) * (p[0].1 - p[2].1);
    if det.abs() < 1e-6 {
        return;
    }
    let n = tex.size as i32;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let l0 = ((p[1].1 - p[2].1) * (fx - p[2].0) + (p[2].0 - p[1].0) * (fy - p[2].1)) / det;
            let l1 = ((p[2].1 - p[0].1) * (fx - p[2].0) + (p[0].0 - p[2].0) * (fy - p[2].1)) / det;
            let l2 = 1.0 - l0 - l1;
            if l0 < -0.001 || l1 < -0.001 || l2 < -0.001 {
                continue;
            }
            let u = l0 * uv[0].0 + l1 * uv[1].0 + l2 * uv[2].0;
            let v = l0 * uv[0].1 + l1 * uv[1].1 + l2 * uv[2].1;
            let tx = ((u * n as f32) as i32).clamp(0, n - 1);
            let ty = ((v * n as f32) as i32).clamp(0, n - 1);
            let s = ((ty * n + tx) * 4) as usize;
            let d = ((y * bw + x) * 4) as usize;
            buf[d..d + 3].copy_from_slice(&tex.rgba[s..s + 3]);
            buf[d + 3] = 255;
        }
    }
}

impl LiveRenderer {
    fn land_art(&mut self, art: &FileIndex, id: u16) -> Option<&ArtBitmap> {
        self.land
            .entry(id)
            .or_insert_with(|| art.get_data(id as usize).and_then(|d| decode_land(&d)))
            .as_ref()
    }
    fn static_art(&mut self, art: &FileIndex, id: u16) -> Option<&ArtBitmap> {
        self.stat
            .entry(id)
            .or_insert_with(|| art.get_data(0x4000 + id as usize).and_then(|d| decode_static(&d)))
            .as_ref()
    }
    fn texture(&mut self, tex: &FileIndex, id: u16) -> Option<&TextureBitmap> {
        self.tex
            .entry(id)
            .or_insert_with(|| {
                let d = tex.get_data(id as usize)?;
                decode_texture(&d, tex.entries.get(id as usize)?.extra)
            })
            .as_ref()
    }

    /// Renders a `width` x `height` view centred on world cell (cx, cy). Returns RGBA.
    pub fn render(
        &mut self,
        c: &LiveContext,
        map_id: usize,
        cx: i32,
        cy: i32,
        width: i32,
        height: i32,
        opt: LiveOptions,
    ) -> Vec<u8> {
        let mut buf = vec![0u8; (width * height * 4) as usize];
        for px in buf.chunks_exact_mut(4) {
            px.copy_from_slice(&BG);
        }
        if map_id >= 6 || !c.map.has_map(map_id) {
            return buf;
        }
        let map_w = c.map.actual_widths[map_id] as i32;
        let map_h = MAP_DEFS[map_id].height as i32;

        let land_at = |x: i32, y: i32| -> Option<(u16, i32)> {
            if x < 0 || y < 0 || x >= map_w || y >= map_h {
                return None;
            }
            let b = c.map.land_block(map_id, (x / 8) as u32, (y / 8) as u32, opt.use_diff)?;
            let i = ((y % 8) * 8 + (x % 8)) as usize * 3;
            Some((u16::from_le_bytes([b[i], b[i + 1]]), b[i + 2] as i8 as i32))
        };

        let center_z = land_at(cx, cy).map_or(0, |l| l.1);
        let (ox, oy) = (width / 2, height / 2 + center_z * 4 - 22);

        // Visible diagonal range with generous margins for elevated / tall objects.
        let s_min = -(height / 2) / 22 - 12;
        let s_max = (height / 2) / 22 + 24;
        let d_max = (width / 2) / 22 + 4;

        for s in s_min..=s_max {
            for d in -d_max..=d_max {
                if (s + d) & 1 != 0 {
                    continue;
                }
                let (u, v) = ((s + d) / 2, (s - d) / 2);
                let (x, y) = (cx + u, cy + v);
                let Some((tile, z)) = land_at(x, y) else { continue };
                let sx = ox + d * 22;
                let sy = oy + s * 22;

                // ---- land
                let flat_art = c.tiledata.and_then(|t| t.land_tiles.get(tile as usize)).map_or(0, |l| l.texture_id);
                let zr = land_at(x + 1, y).map_or(z, |l| l.1);
                let zl = land_at(x, y + 1).map_or(z, |l| l.1);
                let zb = land_at(x + 1, y + 1).map_or(z, |l| l.1);
                let flat = z == zr && z == zl && z == zb;
                let mut drawn = false;
                if !flat && opt.textured_slopes && flat_art != 0 {
                    if let Some(tx) = c.textures {
                        if let Some(t) = self.texture(tx, flat_art) {
                            let top = (sx as f32, (sy - z * 4) as f32);
                            let right = ((sx + 22) as f32, (sy + 22 - zr * 4) as f32);
                            let left = ((sx - 22) as f32, (sy + 22 - zl * 4) as f32);
                            let bottom = (sx as f32, (sy + 44 - zb * 4) as f32);
                            textured_triangle(&mut buf, width, height, t, [top, right, bottom], [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
                            textured_triangle(&mut buf, width, height, t, [top, bottom, left], [(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
                            drawn = true;
                        }
                    }
                }
                if !drawn {
                    let avg = if flat { z } else { (z + zr + zl + zb) / 4 };
                    if let Some(a) = self.land_art(c.art, tile) {
                        blit(&mut buf, width, height, a, sx - 22, sy - avg * 4);
                    }
                }

                // ---- statics of this cell
                if opt.show_statics {
                    if let Some(raw) = c.map.static_block(map_id, (x / 8) as u32, (y / 8) as u32, opt.use_diff) {
                        let (lx, ly) = ((x % 8) as u8, (y % 8) as u8);
                        let mut items: Vec<(i32, u16, usize)> = raw
                            .chunks_exact(7)
                            .enumerate()
                            .filter(|(_, r)| (r[2] & 7) == lx && (r[3] & 7) == ly)
                            .map(|(n, r)| (r[4] as i8 as i32, u16::from_le_bytes([r[0], r[1]]), n))
                            .filter(|(sz, _, _)| *sz <= opt.max_z)
                            .collect();
                        items.sort_by_key(|&(sz, _, n)| (sz, n));
                        for (sz, id, _) in items {
                            if let Some(a) = self.static_art(c.art, id) {
                                let (w, h) = (a.width as i32, a.height as i32);
                                blit(&mut buf, width, height, &a, sx - w / 2, sy + 44 - h - sz * 4);
                            }
                        }
                    }
                }
            }
        }
        buf
    }
}
