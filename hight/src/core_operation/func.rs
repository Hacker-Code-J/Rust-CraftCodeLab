#[inline(always)]
pub fn f0(byte: u8) -> u8 {
    let rotated_left_1 = byte.rotate_left(1);
    let rotated_left_2 = byte.rotate_left(2);
    let rotated_left_7 = byte.rotate_left(7);

    rotated_left_1 ^ rotated_left_2 ^ rotated_left_7
}

#[inline(always)]
pub fn f1(byte: u8) -> u8 {
    let rotated_left_3 = byte.rotate_left(3);
    let rotated_left_4 = byte.rotate_left(4);
    let rotated_left_6 = byte.rotate_left(6);

    rotated_left_3 ^ rotated_left_4 ^ rotated_left_6
}
