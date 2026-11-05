//! Mythic compression / Move-To-Front (MTF) decoder for Ultima Online UOP and cliloc files.

const MYTHIC_HEADER_XOR_KEY: u32 = 0x8E2C_9A3D;
const MYTHIC_FREQUENCY_HEADER_SIZE: usize = 1024; // 256 * 4 bytes

pub fn decompress_mythic(src: &[u8]) -> Option<Vec<u8>> {
    if src.len() < 4 {
        return None;
    }

    let raw_len = u32::from_le_bytes([src[0], src[1], src[2], src[3]]);
    let expected_len = (raw_len ^ MYTHIC_HEADER_XOR_KEY) as usize;

    let mtf_input = &src[4..];
    let mtf_len = mtf_input.len();
    if mtf_len < MYTHIC_FREQUENCY_HEADER_SIZE {
        return None;
    }

    // Step 1: Decode Move-To-Front (MTF)
    let mut symbols = [0u8; 256];
    for i in 0..256 {
        symbols[i] = i as u8;
    }

    let mut mtf_decoded = vec![0u8; mtf_len];
    for i in 0..mtf_len {
        let ind = mtf_input[i] as usize;
        let val = symbols[ind];
        mtf_decoded[i] = val;
        if ind > 0 {
            symbols.copy_within(0..ind, 1);
            symbols[0] = val;
        }
    }

    // Step 2: Frequency Table & Internal Decompression
    let mut partial_input = [0i32; 256 * 3];
    for i in 0..256 {
        let off = i * 4;
        partial_input[i] = i32::from_le_bytes([
            mtf_decoded[off],
            mtf_decoded[off + 1],
            mtf_decoded[off + 2],
            mtf_decoded[off + 3],
        ]);
    }

    let mut total_sum: i32 = 0;
    let mut non_zero_count: usize = 0;
    for i in 0..256 {
        total_sum += partial_input[i];
        if partial_input[i] != 0 {
            non_zero_count += 1;
        }
    }

    if total_sum <= 0 {
        return None;
    }

    let mut symbol_table = [0u8; 256];
    let mut frequency = [0u8; 256];
    for i in 0..256 {
        symbol_table[i] = i as u8;
    }

    let mut tmp = [0i32; 256];
    tmp.copy_from_slice(&partial_input[..256]);

    for i in 0..256 {
        let mut max_val = 0;
        let mut max_idx = 0;
        for j in 0..256 {
            if tmp[j] > max_val {
                max_val = tmp[j];
                max_idx = j;
            }
        }
        if max_val == 0 {
            break;
        }
        frequency[i] = max_idx as u8;
        tmp[max_idx] = 0;
    }

    let mut m: i32 = 0;
    for i in 0..non_zero_count {
        let freq = frequency[i] as usize;
        let table_idx = (m as usize) + MYTHIC_FREQUENCY_HEADER_SIZE;
        if table_idx >= mtf_len {
            return None;
        }
        symbol_table[mtf_decoded[table_idx] as usize] = freq as u8;
        partial_input[freq + 256] = m + 1;
        m += partial_input[freq];
        partial_input[freq + 512] = m;
    }

    let mut val = symbol_table[0];
    let mut count = 0;
    let total = total_sum as usize;
    let mut dst = vec![0u8; total];

    while count < total {
        dst[count] = val;
        let first_val = partial_input[val as usize + 256] as usize;

        if first_val < (partial_input[val as usize + 512] as usize) {
            let src_idx = first_val + MYTHIC_FREQUENCY_HEADER_SIZE;
            if src_idx >= mtf_len {
                break;
            }
            let idx = mtf_decoded[src_idx] as usize;
            partial_input[val as usize + 256] += 1;

            if idx != 0 {
                symbol_table.copy_within(1..=idx, 0);
                symbol_table[idx] = val;
                val = symbol_table[0];
            }
        } else if non_zero_count > 0 {
            non_zero_count -= 1;
            symbol_table.copy_within(1..=non_zero_count, 0);
            val = symbol_table[0];
        }
        count += 1;
    }

    if expected_len > 0 && expected_len <= dst.len() {
        dst.truncate(expected_len);
    }
    Some(dst)
}
