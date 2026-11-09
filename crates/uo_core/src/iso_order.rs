//! The draw order of the isometric client, shared by the map view and the multi renderer.
//!
//! Things are drawn from the far diagonal to the near one (`x + y`); on one tile the priority breaks
//! ties. The height `z` is only a priority, never the first key: sorting by `z` first draws the floor of
//! an upper storey over a wall that stands in front of it.

const BACKGROUND: u64 = 0x1;
/// "Moves with the house" flag of a multi component (bit 40 of the tiledata flags).
const MULTI_MOVABLE: u64 = 0x100 << 32;

/// The sort key: the diagonal first, the priority breaking ties in hundredths.
pub fn depth(x: i32, y: i32, priority: i32) -> f32 {
    (x + y) as f32 + (127 + priority) as f32 * 0.01
}

/// The priority of a static on its tile, as in the client: its z, one less for background
/// pieces, one more for what has height and for what moves with a house.
pub fn static_priority(z: i32, tile_flags: u64, height: u8) -> i32 {
    let mut priority = z;
    if tile_flags & BACKGROUND != 0 {
        priority -= 1;
    }
    if height != 0 {
        priority += 1;
    }
    if tile_flags & MULTI_MOVABLE != 0 {
        priority += 1;
    }
    priority
}

pub fn static_depth(x: i32, y: i32, z: i32, tile_flags: u64, height: u8) -> f32 {
    depth(x, y, static_priority(z, tile_flags, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_diagonal_comes_before_the_height() {
        // a floor at z 20 on the far diagonal is behind a wall at z 0 on the near one
        assert!(static_depth(1, 1, 20, 0, 0) < static_depth(2, 2, 0, 0, 10));
        // on one tile the higher piece is drawn later
        assert!(static_depth(3, 3, 0, 0, 0) < static_depth(3, 3, 5, 0, 0));
        // background pieces go under, tall ones over
        assert!(static_priority(0, BACKGROUND, 0) < static_priority(0, 0, 0));
        assert!(static_priority(0, 0, 0) < static_priority(0, 0, 5));
    }
}
