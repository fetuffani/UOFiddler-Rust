//! Link between equipment animation ids and paperdoll gump ids.
//!
//! Rules (client 7.x, see docs/analise_estatica_uop_gump_client_uo_classic.md):
//! * `Equipconv.def`: `bodyType equipId convertTo gumpId hue`; gumpId 0 = equipId + 50000,
//!   -1 = convertTo + 50000, anything else is the literal gump id.
//! * No entry: animId + 50000 (male). For female owners the candidate is male + 10000
//!   (60000 range) and is used only if that gump exists.
//! * `gump.def` redirects a gump id to another (and a hue) after resolution.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquipConvEntry {
    pub convert_to: i32,
    pub gump_id: i32,
    pub hue: i32,
}

#[derive(Clone, Debug, Default)]
pub struct EquipGumpMap {
    /// (body type, equip/anim id) -> entry
    pub equip_conv: HashMap<(u32, u32), EquipConvEntry>,
    /// gump.def: original gump -> (new gump, hue)
    pub gump_def: HashMap<u32, (u32, i32)>,
}

fn strip_comment(line: &str) -> &str {
    line.split('#').next().unwrap_or("").trim()
}

impl EquipGumpMap {
    pub fn parse(equipconv: Option<&str>, gump_def: Option<&str>) -> Self {
        let mut m = Self::default();
        if let Some(t) = equipconv {
            for line in t.lines() {
                let l = strip_comment(line);
                let v: Vec<i32> = l.split_whitespace().map_while(|s| s.parse().ok()).collect();
                if v.len() >= 5 && v[0] >= 0 && v[1] >= 0 {
                    m.equip_conv.insert(
                        (v[0] as u32, v[1] as u32),
                        EquipConvEntry { convert_to: v[2], gump_id: v[3], hue: v[4] },
                    );
                }
            }
        }
        if let Some(t) = gump_def {
            for line in t.lines() {
                let l = strip_comment(line);
                let (Some(open), Some(close)) = (l.find('{'), l.find('}')) else { continue };
                let Ok(orig) = l[..open].trim().parse::<u32>() else { continue };
                let Some(new) = l[open + 1..close].split(',').next().and_then(|s| s.trim().parse::<u32>().ok()) else { continue };
                let hue = l[close + 1..].split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                m.gump_def.entry(orig).or_insert((new, hue));
            }
        }
        m
    }

    /// Paperdoll gump id for an equipment animation id. `exists` tells whether a gump is present in GumpArt.
    pub fn gump_for_anim(&self, body_type: u32, anim_id: u32, female: bool, exists: impl Fn(u32) -> bool) -> (u32, i32) {
        let mut hue = 0;
        let male = match self.equip_conv.get(&(body_type, anim_id)) {
            Some(e) => {
                hue = e.hue;
                match e.gump_id {
                    0 => anim_id + 50000,
                    -1 => (e.convert_to.max(0) as u32) + 50000,
                    g => g as u32,
                }
            }
            None => anim_id + 50000,
        };
        let mut gump = male;
        if female && (50000..60000).contains(&male) && exists(male + 10000) {
            gump = male + 10000;
        } else if !female && (50000..60000).contains(&male) && !exists(male) && exists(male + 10000) {
            // the gump of the owner's sex is missing: use the other sex's gump (as the game client does)
            gump = male + 10000;
        }
        if let Some(&(g2, h2)) = self.gump_def.get(&gump) {
            gump = g2;
            if h2 != 0 { hue = h2; }
        }
        (gump, hue)
    }

    /// Equipment animation id actually used for a body type: `Equipconv.def` can redirect it
    /// (`convertToID`); without an entry the item's own animation id is used.
    pub fn convert_anim(&self, body_type: u32, anim_id: u32) -> u32 {
        match self.equip_conv.get(&(body_type, anim_id)) {
            Some(e) if e.convert_to > 0 => e.convert_to as u32,
            _ => anim_id,
        }
    }

    /// Reverse lookup: which equipment animation id does a paperdoll gump belong to?
    /// Returns (anim id, female).
    pub fn anim_for_gump(&self, gump_id: u32) -> Option<(u32, bool)> {
        // explicit Equipconv targets first
        for (&(_, equip), e) in &self.equip_conv {
            let g = match e.gump_id {
                0 => equip + 50000,
                -1 => e.convert_to.max(0) as u32 + 50000,
                g => g as u32,
            };
            if g == gump_id {
                return Some((equip, gump_id >= 60000));
            }
        }
        match gump_id {
            50000..=59999 => Some((gump_id - 50000, false)),
            60000..=69999 => Some((gump_id - 60000, true)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_gump_falls_back_to_the_other_sex() {
        let m = EquipGumpMap::default();
        // only the female gump (60434) exists
        let only_female = |g: u32| g == 60434;
        assert_eq!(m.gump_for_anim(400, 434, false, only_female).0, 60434, "male owner falls back to the female gump");
        assert_eq!(m.gump_for_anim(401, 434, true, only_female).0, 60434);
        // only the male gump (50434) exists
        let only_male = |g: u32| g == 50434;
        assert_eq!(m.gump_for_anim(401, 434, true, only_male).0, 50434, "female owner falls back to the male gump");
        assert_eq!(m.gump_for_anim(400, 434, false, only_male).0, 50434);
        // nothing exists: the male id is returned (the caller draws nothing)
        assert_eq!(m.gump_for_anim(400, 434, false, |_| false).0, 50434);
    }

    #[test]
    fn convert_anim_uses_equipconv() {
        let m = EquipGumpMap::parse(Some("401\t866\t867\t60867\t0\t# helm\n"), None);
        assert_eq!(m.convert_anim(401, 866), 867);
        assert_eq!(m.convert_anim(400, 866), 866);
        assert_eq!(m.convert_anim(401, 5), 5);
    }
}
