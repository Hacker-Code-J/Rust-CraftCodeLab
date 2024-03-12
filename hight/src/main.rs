fn main() {
    // Assuming `string_to_byte` is a function that converts hex string to byte array,
    // and `hight_encrypt` and `hight_decrypt` are implemented as per previous discussions.
    let key_string = "28dbc3bc49ffd87dcfa509b11d422be7";
    let input_string = "b41e6be2eba84a14";

    // Convert string representations to byte arrays
    let mk = hex::decode(key_string).expect("Invalid hex in key string");
    let pt = hex::decode(input_string).expect("Invalid hex in input string");

    // Check for correct sizes, this is more of Rust's safety coming into play
    assert_eq!(mk.len(), 16, "Key must be 16 bytes");
    assert_eq!(pt.len(), 8, "Plaintext must be 8 bytes");

    // Create arrays for ciphertext and decrypted text, based on plaintext size
    let mut ct = [0u8; 8];
    let mut dt = [0u8; 8];

    // Perform encryption and decryption
    hight_encrypt(&mut ct, &pt, &mk);
    hight_decrypt(&mut dt, &ct, &mk);

    // Optionally print out results, formatted as hex strings
    println!("Original: {}", hex::encode(pt));
    println!("Encrypted: {}", hex::encode(ct));
    println!("Decrypted: {}", hex::encode(dt));
}