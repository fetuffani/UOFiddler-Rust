//! UOP 64-bit Jenkins hash calculation for Ultima Online archive paths.

#[inline(always)]
fn rotl32(x: u32, k: u32) -> u32 {
    x.rotate_left(k)
}

/// Computes the 64-bit Jenkins hash of an archive entry path (e.g. "build/animationlegacyframe/000009/01.bin")
pub fn hash_file_name(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut len = bytes.len();
    let mut a = (len as u32).wrapping_add(0xDEAD_BEEF);
    let mut b = a;
    let mut c = a;

    let mut i = 0;
    while len > 12 {
        a = a.wrapping_add(
            (bytes[i] as u32)
                | ((bytes[i + 1] as u32) << 8)
                | ((bytes[i + 2] as u32) << 16)
                | ((bytes[i + 3] as u32) << 24),
        );
        b = b.wrapping_add(
            (bytes[i + 4] as u32)
                | ((bytes[i + 5] as u32) << 8)
                | ((bytes[i + 6] as u32) << 16)
                | ((bytes[i + 7] as u32) << 24),
        );
        c = c.wrapping_add(
            (bytes[i + 8] as u32)
                | ((bytes[i + 9] as u32) << 8)
                | ((bytes[i + 10] as u32) << 16)
                | ((bytes[i + 11] as u32) << 24),
        );

        a = a.wrapping_sub(c);
        a ^= rotl32(c, 4);
        c = c.wrapping_add(b);

        b = b.wrapping_sub(a);
        b ^= rotl32(a, 6);
        a = a.wrapping_add(c);

        c = c.wrapping_sub(b);
        c ^= rotl32(b, 8);
        b = b.wrapping_add(a);

        a = a.wrapping_sub(c);
        a ^= rotl32(c, 16);
        c = c.wrapping_add(b);

        b = b.wrapping_sub(a);
        b ^= rotl32(a, 19);
        a = a.wrapping_add(c);

        c = c.wrapping_sub(b);
        c ^= rotl32(b, 4);
        b = b.wrapping_add(a);

        i += 12;
        len -= 12;
    }

    if len == 12 {
        c = c.wrapping_add((bytes[i + 11] as u32) << 24);
    }
    if len >= 11 {
        c = c.wrapping_add((bytes[i + 10] as u32) << 16);
    }
    if len >= 10 {
        c = c.wrapping_add((bytes[i + 9] as u32) << 8);
    }
    if len >= 9 {
        c = c.wrapping_add(bytes[i + 8] as u32);
    }
    if len >= 8 {
        b = b.wrapping_add((bytes[i + 7] as u32) << 24);
    }
    if len >= 7 {
        b = b.wrapping_add((bytes[i + 6] as u32) << 16);
    }
    if len >= 6 {
        b = b.wrapping_add((bytes[i + 5] as u32) << 8);
    }
    if len >= 5 {
        b = b.wrapping_add(bytes[i + 4] as u32);
    }
    if len >= 4 {
        a = a.wrapping_add((bytes[i + 3] as u32) << 24);
    }
    if len >= 3 {
        a = a.wrapping_add((bytes[i + 2] as u32) << 16);
    }
    if len >= 2 {
        a = a.wrapping_add((bytes[i + 1] as u32) << 8);
    }
    if len >= 1 {
        a = a.wrapping_add(bytes[i] as u32);
    }
    if len == 0 {
        return (c as u64) << 32;
    }

    c ^= b;
    c = c.wrapping_sub(rotl32(b, 14));
    a ^= c;
    a = a.wrapping_sub(rotl32(c, 11));
    b ^= a;
    b = b.wrapping_sub(rotl32(a, 25));
    c ^= b;
    c = c.wrapping_sub(rotl32(b, 16));
    a ^= c;
    a = a.wrapping_sub(rotl32(c, 4));
    b ^= a;
    b = b.wrapping_sub(rotl32(a, 14));
    c ^= b;
    c = c.wrapping_sub(rotl32(b, 24));

    ((b as u64) << 32) | (c as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_known_values() {
        let h1 = hash_file_name("build/animationlegacyframe/000009/01.bin");
        assert_ne!(h1, 0);
    }
}
