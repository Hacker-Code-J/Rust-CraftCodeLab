mod aes;
mod aes_key_expansion;
mod aes_utils;
mod aes_config;

// use rand::Rng; // rand crate is required for random number generation
// use std::fmt::Write; // for using write! macro with String
use std::time::{Duration, Instant};

// use crate::aes_config::ROUND_KEYS_SIZE;
use crate::aes::aes_encrypt;

fn measure_time<F>(func: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    let end = Instant::now();

    end.duration_since(start)
}

fn string_to_byte_array(hex_string: &str, byte_array: &mut [u8]) {
    for (i, chunk) in hex_string.as_bytes().chunks(2).enumerate() {
        let hex_chunk = std::str::from_utf8(chunk).expect("Invalid UTF-8");
        byte_array[i] = u8::from_str_radix(hex_chunk, 16).expect("Invalid hex string");
    }
}

// // Function to generate a random key
// fn random_key_generation() -> [u8; aes_config::AES_BLOCK_SIZE] {
//     let mut key = [0u8; aes_config::AES_BLOCK_SIZE];
//     let mut rng = rand::thread_rng();
//     for byte in key.iter_mut() {
//         *byte = rng.gen();
//     }
//     key
// }

// fn key_expansion_test() {
//     // Uncomment the next line if you want to generate a random key
//     let u_key = random_key_generation();
    
//     // Using a fixed key for testing
//     // let u_key: [u8; aes_config::AES_BLOCK_SIZE] = [
//     //     0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
//     //     0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
//     // ];

//     let mut key_str = String::new();
//     for byte in u_key.iter() {
//         write!(&mut key_str, "{:02x}", byte).unwrap();
//     }
//     println!("{}", key_str);

//     let mut r_keys = [0u32; ROUND_KEYS_SIZE / 4];
//     aes_key_expansion::key_expansion(&u_key, &mut r_keys);

//     for &key in r_keys.iter() {
//         println!("{:08x}", key);
//     }
// }

// fn main() {
//     // // Define your AES encrypt function here (as an example)
//     // fn aes_encrypt(input: &[u8], key: &[u8], output: &mut [u8]) {
//     //     // AES encryption logic
//     // }

//     // let input = [0u8; 16]; // Example input
//     // let key = [0u8; 16];   // Example key
//     // let mut output = [0u8; 16]; // Buffer for output

//     // let duration = measure_time(aes_encrypt, &input, &key, &mut output);

//     // println!("Time taken: {:?}", duration);
// }

fn main() {
    let input_string = "f34481ec3cc627bacd5dc3fb08f273e6";
    let mut input = [0u8; 16];
    string_to_byte_array(input_string, &mut input);

    // Print the plaintext
    println!("Plaintext:\n{}\n", input.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(""));

    let key_string = "00000000000000000000000000000000";
    const AES_VERSION: usize = 128;
    let mut key = [0u8; AES_VERSION / 8];
    string_to_byte_array(key_string, &mut key);

    // Print the key
    println!("Key:\n{}\n", key.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(""));

    let mut output = [0u8; 16];

    // Call the AES_Encrypt function
    aes_encrypt(&input, &key, &mut output);

    // Print the ciphertext
    println!("Ciphertext:\n{}\n", output.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(""));

    // Measure encryption time
    let time_enc = measure_time(|| aes_encrypt(&input, &key, &mut output));
    println!("Time for AES_Encrypt: {:?}", time_enc);
}
