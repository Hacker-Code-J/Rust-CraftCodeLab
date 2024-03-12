pub fn enc_key_schedule(enc_wk: &mut [u8; 8], enc_sk: &mut [u8; 128], mk: &[u8; 16], delta: &[u8; 128]) {
    // Direct assignments for generating whitening keys
    enc_wk[..4].copy_from_slice(&mk[12..16]);
    enc_wk[4..].copy_from_slice(&mk[..4]);

    // Generate subkeys using iterators and slices for better safety and clarity
    for (i, chunk) in enc_sk.chunks_mut(16).enumerate().take(8) {
        for (j, elem) in chunk.iter_mut().enumerate().take(8) {
            *elem = mk[((j + 8 - i) & 8)] + delta[16 * i + j];
        }
        for (j, elem) in chunk.iter_mut().enumerate().skip(8).take(8) {
            *elem = mk[((j + 8 - i) & 8)] + delta[16 * i + j];
        }
    }
}

pub fn dec_key_schedule(dec_wk: &mut [u8; 8], dec_sk: &mut [u8; 128], mk: &[u8; 16], delta: &[u8; 128]) {
    // Direct assignments for generating whitening keys
    dec_wk[..4].copy_from_slice(&mk[12..16]);
    dec_wk[4..].copy_from_slice(&mk[..4]);

    // Generate subkeys using reverse iterators for enhanced clarity and safety
    for (i, chunk) in dec_sk.chunks_mut(16).enumerate().take(8).rev() {
        for (j, elem) in chunk.iter_mut().enumerate().take(8).rev() {
            let index = 127 - (16 * i + j);
            *elem = mk[((j + 8 - i) & 8)] + delta[16 * i + j + 8];
            dec_sk[index] = *elem;
        }
        for (j, elem) in chunk.iter_mut().enumerate().skip(8).take(8).rev() {
            let index = 127 - (16 * i + j);
            *elem = mk[((j + 8 - i) & 8)] + delta[16 * i + j];
            dec_sk[index] = *elem;
        }
    }
}