// Import the S_BOX from aes_config module
mod aes_config;
use aes_config::{AesVersion, aes_parameters, S_BOX};

// Rust implementation of AES Key Expansion

// Define a constant array for rCon
const RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000,
    0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1b000000, 0x36000000,
];

// Function equivalent to the RotWord macro
fn rot_word(word: u32) -> u32 {
    (word << 8) | (word >> 24)
}

// Function equivalent to the SubWord macro, assuming the existence of s_box
fn sub_word(word: u32) -> u32 {
    // Now uses the S_BOX from aes_config.rs
    ((S_BOX[(word >> 24) as usize] as u32) << 24) |
    ((S_BOX[((word >> 16) & 0xFF) as usize] as u32) << 16) |
    ((S_BOX[((word >> 8) & 0xFF) as usize] as u32) << 8) |
    (S_BOX[(word & 0xFF) as usize] as u32)
}

// Function for key expansion
// Note: uKey is a slice of u8, and rKey is a mutable slice of u32.
// This approach avoids using raw pointers and ensures memory safety.
fn key_expansion(u_key: &[u8], r_key: &mut [u32], nk: usize, nr: usize) {
    // First, determine the AES parameters for the desired version.
    let (nk, nr, _round_keys_size) = aes_parameters(AesVersion::AES128);

    let mut temp: u32;

    for i in 0..nk {
        r_key[i] = ((u_key[4 * i] as u32) << 24) |
                   ((u_key[4 * i + 1] as u32) << 16) |
                   ((u_key[4 * i + 2] as u32) << 8) |
                   (u_key[4 * i + 3] as u32);
    }

    for i in nk..((nr + 1) * 4) {
        temp = r_key[i - 1];
        if i % nk == 0 {
            temp = sub_word(rot_word(temp), &S_BOX) ^ RCON[i / nk - 1]; // Assuming S_BOX is defined
        } else if nk > 6 && i % nk == 4 {
            temp = sub_word(temp, &S_BOX); // Additional S-box transformation for AES-256, assuming S_BOX is defined
        }
        r_key[i] = r_key[i - nk] ^ temp;
    }
}