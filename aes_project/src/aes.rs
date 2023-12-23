use crate::aes_config::{AES_BLOCK_SIZE, ROUND_KEYS_SIZE, NK, NR, S_BOX};
use crate::aes_key_expansion::{key_expansion};

fn add_round_key(state: &mut [u8], r_key: &[u32]) {
    for i in 0..AES_BLOCK_SIZE {
        state[i] ^= ((r_key[i / 4] >> (8 * (3 - (i % 4)))) & 0xFF) as u8;
    }
}

fn sub_bytes(state: &mut [u8]) {
    for i in 0..AES_BLOCK_SIZE {
        state[i] = S_BOX[state[i] as usize];
    }
}

fn shift_rows(state: &mut [u8]) {
    // Ensure the state has the correct size (16 bytes for AES)
    assert_eq!(state.len(), 16, "State must be 16 bytes");

    let mut temp;

    // Row 1: shift left by 1
    temp = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = temp;

    // Row 2: shift left by 2
    temp = state[2];
    state[2] = state[10];
    state[10] = temp;
    temp = state[6];
    state[6] = state[14];
    state[14] = temp;

    // Row 3: shift left by 3 (or right by 1)
    temp = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = state[3];
    state[3] = temp;
}

fn mul_gf256(a: u8, b: u8) -> u8 {
    let mut res = 0;
    let msb_mask = 0x80;
    let modulo = 0x1B;

    let mut temp_a = a;
    let mut temp_b = b;

    for _ in 0..8 {
        if temp_b & 1 == 1 {
            res ^= temp_a;
        }

        let msb = temp_a & msb_mask;
        temp_a <<= 1;

        if msb != 0 {
            temp_a ^= modulo;
        }

        temp_b >>= 1;
    }

    res
}

fn mix_columns(state: &mut [u8]) {
    assert_eq!(state.len(), 16, "State must be 16 bytes");

    for i in 0..4 {
        let temp = [
            mul_gf256(0x02, state[i * 4]) ^
            mul_gf256(0x03, state[i * 4 + 1]) ^
            state[i * 4 + 2] ^
            state[i * 4 + 3],

            state[i * 4] ^
            mul_gf256(0x02, state[i * 4 + 1]) ^
            mul_gf256(0x03, state[i * 4 + 2]) ^
            state[i * 4 + 3],

            state[i * 4] ^
            state[i * 4 + 1] ^
            mul_gf256(0x02, state[i * 4 + 2]) ^
            mul_gf256(0x03, state[i * 4 + 3]),

            mul_gf256(0x03, state[i * 4]) ^
            state[i * 4 + 1] ^
            state[i * 4 + 2] ^
            mul_gf256(0x02, state[i * 4 + 3]),
        ];

        for j in 0..4 {
            state[i * 4 + j] = temp[j];
        }
    }
}

pub fn aes_encrypt(plaintext: &[u8], key: &[u8], ciphertext: &mut [u8]) {
    assert_eq!(plaintext.len(), AES_BLOCK_SIZE);
    assert_eq!(key.len(), NK * 4);
    assert_eq!(ciphertext.len(), AES_BLOCK_SIZE);

    let mut state = [0u8; AES_BLOCK_SIZE];
    state.copy_from_slice(plaintext);

    let mut round_key = [0u32; ROUND_KEYS_SIZE / 4];
    key_expansion(key, &mut round_key);

    add_round_key(&mut state, &round_key[0..4]);

    for round in 1..= NR {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        if round != NR {
            mix_columns(&mut state);
        }
        add_round_key(&mut state, &round_key[4 * round..4 * (round + 1)]);
    }

    ciphertext.copy_from_slice(&state);
}

