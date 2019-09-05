pub fn float32_to_bytes(x: f32) -> [u8; 4] {
    return x.to_bits().to_be_bytes();
}

// Comparing to the version above, this one is suppose to crop
// fraction/significand part to limit representation to 20 bits, so 3.5 bytes.
pub fn float20_to_bytes(x: f32) -> [u8; 4] {
    let mut bytes = float32_to_bytes(x);
    bytes[3] = bytes[3] & 0xf0;

    return bytes;
}

pub fn bytes_to_float32() -> i32 {
    return 24;
}
