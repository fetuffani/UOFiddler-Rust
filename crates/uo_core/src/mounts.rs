//! Mounts: which creature a mount item stands for and which animation groups the rider and the
//! mount use. Follows ClassicUO (`Game/Data/Mounts.cs`, `Item.GetGraphicForAnimation`, the
//! mounted people groups 23..29 and `Mobile.GetGroupForAnimation`).
//!
//! The tiledata of the reference client has no mount items (no layer 25, no animation id), so the
//! item -> creature link comes from `data/mounts.txt`; an `AnimID` in the tiledata, when there is
//! one, still wins (as in ClassicUO).

use std::collections::HashMap;
use std::sync::OnceLock;

const DATA: &str = include_str!("../data/mounts.txt");

/// UO layer of the mount item.
pub const MOUNT_LAYER: u8 = 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountInfo {
    /// animation body of the mount creature
    pub body: u32,
    /// pixels the rider (and what he wears) is moved down
    pub offset_y: i32,
}

/// `mounts.txt`: mount item id -> creature body and rider offset.
pub fn mount_table() -> &'static HashMap<u16, MountInfo> {
    static MAP: OnceLock<HashMap<u16, MountInfo>> = OnceLock::new();
    MAP.get_or_init(|| parse(DATA))
}

pub fn mount_info(item_id: u16) -> Option<MountInfo> {
    mount_table().get(&item_id).copied()
}

fn parse(text: &str) -> HashMap<u16, MountInfo> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let mut it = line.split_whitespace();
        let (Some(id), Some(body), Some(off)) = (it.next(), it.next(), it.next()) else { continue };
        let (Ok(id), Ok(body), Ok(off)) = (u16::from_str_radix(id.trim_start_matches("0x"), 16), body.parse::<u32>(), off.parse::<i32>()) else {
            continue;
        };
        map.insert(id, MountInfo { body, offset_y: off });
    }
    map
}

/// People animation group (classic numbering) of a rider for the action picked in the viewer:
/// walking -> 23 (ride slow), running -> 24 (ride fast), melee -> 26, bow -> 27, crossbow -> 28,
/// the mounted groups 23..=29 stay as they are, everything else is the mounted stand (25).
pub fn rider_group(action: u32) -> u32 {
    match action {
        0 | 1 | 15 => 23,
        2 | 3 => 24,
        9..=14 | 31 => 26,
        18 => 27,
        19 => 28,
        23..=29 => action,
        _ => 25,
    }
}

/// Group of the mount creature while its rider does `rider_group`: walks or runs with him, stands
/// otherwise. `anim_type` is the creature class of mobtypes.txt (2 animal, anything else monster):
/// animals have Walk 0 / Run 1 / Stand 2, monsters Walk 0 / Stand 1 (no run).
pub fn mount_group(rider_group: u32, anim_type: u8) -> u32 {
    let animal = anim_type == 2;
    match rider_group {
        23 => 0,
        24 => u32::from(animal),
        _ => {
            if animal {
                2
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_parses_and_matches_classicuo() {
        let t = mount_table();
        assert_eq!(t.len(), 74);
        assert_eq!(t[&0x3EA2], MountInfo { body: 204, offset_y: 0 }); // dark brown horse
        assert_eq!(t[&0x3E9B].body, 0xC0); // ethereal unicorn
        assert_eq!(t[&0x3E9C].body, 0xBF); // ethereal kirin
        assert_eq!(t[&0x3E9D].offset_y, -9);
        assert!(mount_info(0x1517).is_none());
    }

    #[test]
    fn rider_and_mount_groups() {
        assert_eq!([rider_group(0), rider_group(2), rider_group(4), rider_group(9), rider_group(18), rider_group(19), rider_group(26)], [23, 24, 25, 26, 27, 28, 26]);
        assert_eq!(mount_group(23, 2), 0);
        assert_eq!(mount_group(24, 2), 1); // a horse runs
        assert_eq!(mount_group(24, 0), 0); // a dragon has no run
        assert_eq!(mount_group(25, 2), 2);
        assert_eq!(mount_group(26, 0), 1);
    }

    #[test]
    fn parse_skips_comments_and_bad_lines() {
        let m = parse("# c\n0x0001 200 -9 # name\nbad\n0x0002 x 0\n");
        assert_eq!(m.len(), 1);
        assert_eq!(m[&1], MountInfo { body: 200, offset_y: -9 });
    }
}
