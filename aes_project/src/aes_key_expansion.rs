use crate::aes_config::{NK, NR, S_BOX};

static RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000,
    0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1b000000, 0x36000000
];

fn rot_word(word: u32) -> u32 {
    (word << 8) | (word >> 24)
}

fn sub_word(word: u32) -> u32 {
    let mut result: u32 = 0u32;
    for i in 0..4 {
        let byte: usize = ((word >> (8 * (3 - i))) & 0xff) as usize;
        result |= (S_BOX[byte] as u32) << (8 * (3 - i));
    }
    result
}

pub fn key_expansion(u_key: &[u8], r_key: &mut [u32]) {
    for i in 0..NK {
        r_key[i] = ((u_key[4 * i] as u32) << 24)
            | ((u_key[4 * i + 1] as u32) << 16)
            | ((u_key[4 * i + 2] as u32) << 8)
            | (u_key[4 * i + 3] as u32);
    }

    for i in NK..(NR + 1) * 4 {
        let mut temp = r_key[i - 1];
        if i % NK == 0 {
            temp = sub_word(rot_word(temp)) ^ RCON[i / NK - 1];
        } else if NK > 6 && i % NK == 4 {
            // Additional S-box transformation for AES-256
            temp = sub_word(temp);
        }
        r_key[i] = r_key[i - NK] ^ temp;
    }
}

// use crate::aes_config::S_BOX;

// static RCON: [u32; 10] = [
//     0x01000000, 0x02000000, 0x04000000, 0x08000000,
//     0x10000000, 0x20000000, 0x40000000, 0x80000000,
//     0x1b000000, 0x36000000,
// ];

// #[inline]
// fn rot_word(word: u32) -> u32 {
//     (word << 8) | (word >> 24)
// }

// #[inline]
// fn sub_word(word: u32) -> u32 {
//     (0..4).fold(0, |acc, i| {
//         let byte = ((word >> (8 * (3 - i))) & 0xff) as usize;
//         acc | ((S_BOX[byte] as u32) << (8 * (3 - i)))
//     })
// }

// pub fn key_expansion(u_key: &[u8], r_key: &mut [u32]) {
//     u_key.chuNKs(4).enumerate().for_each(|(i, chuNK)| {
//         r_key[i] = u32::from_be_bytes([chuNK[0], chuNK[1], chuNK[2], chuNK[3]]);
//     });

//     (4..44).for_each(|i| {
//         let mut temp = r_key[i - 1];
//         if i % 4 == 0 {
//             temp = sub_word(rot_word(temp)) ^ RCON[i / 4 - 1];
//         }
//         r_key[i] = r_key[i - 4] ^ temp;
//     });
// }
