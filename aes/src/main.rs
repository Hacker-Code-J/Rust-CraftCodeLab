mod aes_utils;
mod aes_key_expansion;
mod aes;

fn main() {
    use aes_config::{AesVersion, aes_parameters};

    let (nk, nr, round_keys_size) = aes_parameters(AesVersion::AES128);
    println!("Nk: {}, Nr: {}, Round Keys Size: {}", nk, nr, round_keys_size);
}