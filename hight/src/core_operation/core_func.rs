

fn hight_encrypt(dst: &mut [u8; 8], src: &[u8; 8], mk: &[u8; 16], delta_table: &[u8; 128]) {
    let mut wk = [0u8; 8];
    wk[..4].copy_from_slice(&mk[12..]);
    wk[4..].copy_from_slice(&mk[..4]);

    let mut sk = [0u8; 128];
    for i in 0..8 {
        for j in 0..8 {
            sk[16 * i + j] = mk[((j + 8 - i) % 8)] + delta_table[16 * i + j];
            sk[16 * i + j + 8] = mk[((j + 8 - i) % 8) + 8] + delta_table[16 * i + j + 8];
        }
    }

    let mut state = *src; // Copy source to state directly

    // Initial transformation
    state[0] = state[0].wrapping_add(wk[0]);
    state[2] ^= wk[1];
    state[4] = state[4].wrapping_add(wk[2]);
    state[6] ^= wk[3];

    // Main rounds
    for i in 0..31 {
        let t0 = state[7];
        let t1 = state[6];
        state[7] = state[6];
        state[6] = state[5].wrapping_add(f1(state[4]) ^ sk[i * 4 + 2]);
        state[5] = state[4];
        state[4] = state[3] ^ (f0(state[2]).wrapping_add(sk[i * 4 + 1]));
        state[3] = state[2];
        state[2] = state[1].wrapping_add(f1(state[0]) ^ sk[i * 4 + 0]);
        state[1] = state[0];
        state[0] = t0 ^ (f0(t1).wrapping_add(sk[i * 4 + 3]));
    }

    // Final transformation
    state[1] = state[1].wrapping_add(f1(state[0]) ^ sk[124]);
    state[3] ^= f0(state[2]).wrapping_add(sk[125]);
    state[5] = state[5].wrapping_add(f1(state[4]) ^ sk[126]);
    state[7] ^= f0(state[6]).wrapping_add(sk[127]);

    state[0] = state[0].wrapping_add(wk[4]);
    state[2] ^= wk[5];
    state[4] = state[4].wrapping_add(wk[6]);
    state[6] ^= wk[7];

    dst.copy_from_slice(&state);
}

fn hight_decrypt(dst: &mut [u8; 8], src: &[u8; 8], mk: &[u8; 16], delta_table: &[u8; 128]) {
    let mut wk = [
        mk[12], mk[13], mk[14], mk[15],
        mk[0], mk[1], mk[2], mk[3],
    ];

    let mut sk = [0u8; 128];
    // Generate SK by iterating in reverse
    for i in (0..8).rev() {
        for j in (0..8).rev() {
            sk[127 - (16 * i + j + 8)] = mk[((j + 8 - i) % 8) + 8] + delta_table[16 * i + j + 8];
            sk[127 - (16 * i + j)] = mk[((j + 8 - i) % 8)] + delta_table[16 * i + j];
        }
    }

    let mut state: [u8; 8] = *src;
    state[0] = state[0].wrapping_sub(wk[4]);
    state[2] ^= wk[5];
    state[4] = state[4].wrapping_sub(wk[6]);
    state[6] ^= wk[7];

    // Pre-round transformation
    state[1] = state[1].wrapping_sub(f1(state[0]) ^ sk[124]);
    state[3] ^= f0(state[2]) + sk[125];
    state[5] = state[5].wrapping_sub(f1(state[4]) ^ sk[126]);
    state[7] ^= f0(state[6]) + sk[127];

    // Main rounds
    for i in 1..32 {
        let (temp0, temp2, temp4, temp6) = (state[0], state[2], state[4], state[6]);

        state[0] = state[1];
        state[2] = state[3];
        state[4] = state[5];
        state[6] = state[7];

        state[7] = temp0 ^ (f0(state[7]) + sk[i * 4]);
        state[5] = temp6.wrapping_sub(f1(state[5]) ^ sk[i * 4 + 1]);
        state[3] = temp4 ^ (f0(state[3]) + sk[i * 4 + 2]);
        state[1] = temp2.wrapping_sub(f1(state[1]) ^ sk[i * 4 + 3]);
    }

    // Post-round transformation
    state[0] = state[0].wrapping_sub(wk[0]);
    state[2] ^= wk[1];
    state[4] = state[4].wrapping_sub(wk[2]);
    state[6] ^= wk[3];

    *dst = state;
}