//! Classic animation body resolution: Body.def (body redirect + hue) and Bodyconv.def
//! (which animN.mul file holds a body, and under which body number), plus the
//! per-file index formulas used by the client for anim.mul .. anim5.mul.

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ClassicAnimResolver {
    /// Body.def: original body -> (new body, hue)
    pub body_def: HashMap<u32, (u32, u32)>,
    /// Bodyconv.def: original body -> (file type 2..=5, body inside that file)
    pub body_conv: HashMap<u32, (u32, u32)>,
}

/// Where a body lives after applying Body.def / Bodyconv.def.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedBody {
    pub file_type: u32,
    pub body: u32,
    pub hue: u32,
}

impl ClassicAnimResolver {
    pub fn parse(body_def: Option<&str>, body_conv: Option<&str>) -> Self {
        let mut r = Self::default();
        if let Some(text) = body_def {
            for line in text.lines() {
                let line = line.trim();
                if !line.as_bytes().first().map_or(false, |c| c.is_ascii_digit()) {
                    continue;
                }
                let (Some(open), Some(close)) = (line.find('{'), line.find('}')) else { continue };
                let Ok(orig) = line[..open].trim().parse::<u32>() else { continue };
                let new_body = line[open + 1..close]
                    .split(',')
                    .next()
                    .and_then(|s| s.trim().parse::<u32>().ok());
                let hue = line[close + 1..]
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);
                if let Some(nb) = new_body {
                    r.body_def.entry(orig).or_insert((nb, hue));
                }
            }
        }
        if let Some(text) = body_conv {
            for line in text.lines() {
                let line = line.trim();
                if !line.as_bytes().first().map_or(false, |c| c.is_ascii_digit()) {
                    continue;
                }
                let v: Vec<i64> = line
                    .split_whitespace()
                    .map_while(|s| s.parse::<i64>().ok())
                    .collect();
                if v.len() < 2 {
                    continue;
                }
                let orig = v[0] as u32;
                // columns: anim2, anim3, anim4, anim5 (first non-negative wins, same order as the client)
                for (i, &nb) in v[1..].iter().take(4).enumerate() {
                    if nb >= 0 {
                        r.body_conv.entry(orig).or_insert((i as u32 + 2, nb as u32));
                        break;
                    }
                }
            }
        }
        r
    }

    /// Applies Body.def then Bodyconv.def. Bodies not converted live in anim.mul (file type 1).
    pub fn resolve(&self, body: u32) -> ResolvedBody {
        let (body, hue) = self.body_def.get(&body).copied().unwrap_or((body, 0));
        match self.body_conv.get(&body) {
            Some(&(file_type, nb)) => ResolvedBody { file_type, body: nb, hue },
            None => ResolvedBody { file_type: 1, body, hue },
        }
    }

    /// Index of the first entry of a body/action (direction 0) in the given animN.idx.
    pub fn base_index(file_type: u32, body: u32) -> u32 {
        match file_type {
            2 => {
                if body < 200 { body * 110 } else { 22000 + (body - 200) * 65 }
            }
            3 => {
                if body < 300 {
                    body * 65
                } else if body < 400 {
                    33000 + (body - 300) * 110
                } else {
                    35000 + (body - 400) * 175
                }
            }
            5 => {
                if body < 200 && body != 34 {
                    body * 110
                } else if body < 400 {
                    22000 + (body - 200) * 65
                } else {
                    35000 + (body - 400) * 175
                }
            }
            // 1 (anim.mul) and 4 (anim4.mul)
            _ => {
                if body < 200 {
                    body * 110
                } else if body < 400 {
                    22000 + (body - 200) * 65
                } else {
                    35000 + (body - 400) * 175
                }
            }
        }
    }

    pub fn index(file_type: u32, body: u32, action: u32, direction: u8) -> usize {
        let dir = if direction <= 4 { direction as u32 } else { (direction - ((direction - 4) * 2)) as u32 };
        (Self::base_index(file_type, body) + action * 5 + dir) as usize
    }
}

/// Loads the frames of a body/action/direction the way the viewer does: Body.def/Bodyconv.def
/// resolved classic MUL first, then UOP, then every classic file with the legacy formulas.
pub fn load_animation_frames(
    resolver: &ClassicAnimResolver,
    mul: &HashMap<u32, crate::FileIndex>,
    uop: &crate::UopAnimationIndex,
    body: u32,
    action: u32,
    direction: u8,
) -> Option<Vec<crate::AnimationFrame>> {
    let flip = direction > 4;
    let r = resolver.resolve(body);
    if let Some(data) = mul
        .get(&r.file_type)
        .and_then(|fi| fi.get_data(ClassicAnimResolver::index(r.file_type, r.body, action, direction)))
    {
        let frames = crate::decode_animation_mul(&data, flip);
        if !frames.is_empty() {
            return Some(frames);
        }
    }
    if let Some(frames) = uop.get_animation_frames(body, action, direction) {
        return Some(frames);
    }
    for ft in 1..=5u32 {
        let idx = ClassicAnimResolver::index(ft, body, action, direction);
        if let Some(data) = mul.get(&ft).and_then(|fi| fi.get_data(idx)) {
            let frames = crate::decode_animation_mul(&data, flip);
            if !frames.is_empty() {
                return Some(frames);
            }
        }
    }
    None
}
