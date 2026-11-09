//! What the map view and the editor read: the facet (with the pending edits folded in), the
//! tiledata and the art.

use super::atlas::Image;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uo_core::map_edit::{read_block_with, StaticItem};
use uo_core::{
    apply_hue_bytes, decode_land, decode_static, decode_texture, FileIndex, HuesManager,
    ItemTileData, MapBlock, MapEdits, MapManager, TileDataManager, MAP_DEFS,
};

use super::iso::flags;

pub struct MapAssets<'a> {
    pub maps: &'a MapManager,
    pub map_id: usize,
    pub edits: &'a MapEdits,
    pub art: Option<&'a FileIndex>,
    pub textures: Option<&'a FileIndex>,
    pub tiledata: Option<&'a TileDataManager>,
    pub hues: Option<&'a HuesManager>,
    /// Show the mapdif / stadif patches. Only the viewer asks for them; the editor works on the base map.
    pub use_diff: bool,
    /// Blocks already read this frame (the assets live for one frame, so this never goes stale).
    pub cache: RefCell<HashMap<(i32, i32), Option<Rc<MapBlock>>>>,
}

impl<'a> MapAssets<'a> {
    /// Facet size in tiles.
    pub fn size(&self) -> (i32, i32) {
        (
            self.maps.actual_widths[self.map_id] as i32,
            MAP_DEFS[self.map_id].height as i32,
        )
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let (w, h) = self.size();
        x >= 0 && y >= 0 && x < w && y < h
    }

    /// A block with the pending edits folded in.
    pub fn block(&self, bx: i32, by: i32) -> Option<Rc<MapBlock>> {
        if bx < 0 || by < 0 {
            return None;
        }
        if let Some(known) = self.cache.borrow().get(&(bx, by)) {
            return known.clone();
        }
        let block = read_block_with(self.maps, self.map_id, bx as u32, by as u32, self.use_diff).map(|mut b| {
            self.edits.apply(&mut b);
            Rc::new(b)
        });
        self.cache.borrow_mut().insert((bx, by), block.clone());
        block
    }

    /// Land graphic and height of a tile (edits included).
    pub fn land(&self, x: i32, y: i32) -> Option<(u16, i8)> {
        let b = self.block(x.div_euclid(8), y.div_euclid(8))?;
        let c = b.cell(x, y);
        Some((c.graphic, c.z))
    }

    /// The land height of a tile, when the tile is on the map.
    pub fn land_z(&self, x: i32, y: i32) -> Option<i8> {
        if x < 0 || y < 0 {
            return None;
        }
        self.land(x, y).map(|l| l.1)
    }

    /// A block as the client files hold it, without the edits.
    pub fn original_block(&self, bx: i32, by: i32) -> Option<MapBlock> {
        if bx < 0 || by < 0 {
            return None;
        }
        read_block_with(self.maps, self.map_id, bx as u32, by as u32, self.use_diff)
    }

    pub fn item(&self, graphic: u16) -> Option<&'a ItemTileData> {
        self.tiledata?.item_tiles.get(graphic as usize)
    }

    pub fn item_flags(&self, graphic: u16) -> u64 {
        self.item(graphic).map_or(0, |t| t.flags)
    }

    pub fn item_height(&self, graphic: u16) -> u8 {
        self.item(graphic).map_or(0, |t| t.height)
    }

    pub fn item_name(&self, graphic: u16) -> &'a str {
        self.item(graphic).map_or("", |t| t.name.as_str())
    }

    pub fn land_flags(&self, graphic: u16) -> u64 {
        self.tiledata
            .and_then(|t| t.land_tiles.get(graphic as usize))
            .map_or(0, |t| t.flags)
    }

    pub fn land_texture(&self, graphic: u16) -> u16 {
        self.tiledata
            .and_then(|t| t.land_tiles.get(graphic as usize))
            .map_or(0, |t| t.texture_id)
    }

    pub fn land_name(&self, graphic: u16) -> &'a str {
        self.tiledata
            .and_then(|t| t.land_tiles.get(graphic as usize))
            .map_or("", |t| t.name.as_str())
    }

    /// The 44x44 art of a land tile. A tile whose art is missing, transparent or pure black (the
    /// "void" tiles such as 0x0244, which the client shows as black) is drawn as a dark diamond so the
    /// ground can be seen when editing, instead of statics floating over nothing.
    pub fn land_image(&self, graphic: u16) -> Option<Image> {
        let art = self.art?;
        let bmp = art.get_data(graphic as usize).and_then(|d| decode_land(&d));
        match bmp {
            Some(b) if has_visible_pixels(&b.rgba) => Some(Image {
                width: b.width as usize,
                height: b.height as usize,
                rgba: b.rgba,
            }),
            _ => Some(void_land_image()),
        }
    }

    pub fn static_image(&self, graphic: u16, hue: u16) -> Option<Image> {
        let mut bmp = decode_static(&self.art?.get_data(0x4000 + graphic as usize)?)?;
        if hue != 0 {
            if let Some(h) = self.hues.and_then(|h| h.hues.get(hue as usize - 1)) {
                let partial = self.item_flags(graphic) & flags::PARTIAL_HUE != 0;
                apply_hue_bytes(&mut bmp.rgba, &h.get_rgba_palette(), partial);
            }
        }
        Some(Image {
            width: bmp.width as usize,
            height: bmp.height as usize,
            rgba: bmp.rgba,
        })
    }

    pub fn texture_image(&self, texture: u16) -> Option<Image> {
        let index = self.textures?;
        let extra = index.entries.get(texture as usize)?.extra;
        let tex = decode_texture(&index.get_data(texture as usize)?, extra)?;
        Some(Image {
            width: tex.size as usize,
            height: tex.size as usize,
            rgba: tex.rgba,
        })
    }
}

/// A shorthand over the frame cache, kept for the scene and the panel.
pub struct Blocks<'b, 'a> {
    pub assets: &'b MapAssets<'a>,
}

impl<'b, 'a> Blocks<'b, 'a> {
    pub fn new(assets: &'b MapAssets<'a>) -> Self {
        Self { assets }
    }

    pub fn block(&mut self, bx: i32, by: i32) -> Option<Rc<MapBlock>> {
        self.assets.block(bx, by)
    }

    /// Land graphic and height of a tile (edits included).
    pub fn land(&mut self, x: i32, y: i32) -> Option<(u16, i8)> {
        self.assets.land(x, y)
    }

    pub fn land_z(&mut self, x: i32, y: i32) -> i8 {
        self.assets.land(x, y).map_or(0, |l| l.1)
    }

    /// The statics of a tile (edits included).
    pub fn statics(&mut self, x: i32, y: i32) -> Vec<StaticItem> {
        match self.block(x.div_euclid(8), y.div_euclid(8)) {
            Some(b) => b.statics_at(x, y).copied().collect(),
            None => Vec::new(),
        }
    }
}

/// True when some pixel is drawn and not black.
fn has_visible_pixels(rgba: &[u8]) -> bool {
    rgba.chunks_exact(4).any(|p| p[3] != 0 && (p[0] | p[1] | p[2]) != 0)
}

/// Colour of the stand-in for void land (art missing or fully transparent).
const VOID_LAND: [u8; 4] = [38, 40, 50, 255];

/// A 44x44 diamond like a land tile's art, filled with the void colour.
pub fn void_land_image() -> Image {
    const N: usize = 44;
    let mut rgba = vec![0u8; N * N * 4];
    for y in 0..N {
        let half = if y < N / 2 { y + 1 } else { N - y };
        for x in (N / 2 - half)..(N / 2 + half) {
            rgba[(y * N + x) * 4..(y * N + x) * 4 + 4].copy_from_slice(&VOID_LAND);
        }
    }
    Image { width: N, height: N, rgba }
}

#[cfg(test)]
mod void_tests {
    use super::*;

    #[test]
    fn black_or_transparent_art_counts_as_void() {
        assert!(!has_visible_pixels(&[0, 0, 0, 255].repeat(10)), "opaque black");
        assert!(!has_visible_pixels(&[9, 9, 9, 0].repeat(10)), "transparent");
        assert!(has_visible_pixels(&[0, 0, 1, 255]));
    }

    #[test]
    fn void_land_is_a_diamond() {
        let img = void_land_image();
        let alpha = |x: usize, y: usize| img.rgba[(y * 44 + x) * 4 + 3];
        assert_eq!((img.width, img.height), (44, 44));
        assert_eq!(alpha(22, 22), 255, "centre");
        assert_eq!(alpha(0, 0), 0, "corner");
        assert_eq!(alpha(43, 43), 0, "corner");
        assert_eq!(alpha(22, 0), 255, "top point");
        let filled = img.rgba.chunks_exact(4).filter(|p| p[3] != 0).count();
        assert_eq!(filled, 2 * (1..=22).sum::<usize>() * 2, "a 44x44 diamond");
    }
}
