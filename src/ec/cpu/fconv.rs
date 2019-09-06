// Note: we're using IEEE 754 single-precision floating-point format,
// so instead of 7 bits exponent as in EC specification, floating-point numbers
// are using 8 bits for it.
//
// Useful links:
// http://weitz.de/ieee/ - for test-cases and explanation.
// https://en.wikipedia.org/wiki/Single-precision_floating-point_format
// https://en.wikipedia.org/wiki/Exponent_bias

// TODO: handle errors, return Result instead (most likely it would make the
// most sense)

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

pub fn bytes_to_float32(bytes: [u8; 4]) -> f32 {
    return f32::from_bits(u32::from_be_bytes(bytes));
}

pub fn bytes_to_float20(bytes: [u8; 4]) -> f32 {
    assert_eq!(bytes[3] & 0x0f, 0);
    return bytes_to_float32(bytes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_float32_to_bytes_conversion() {
        assert_eq!(float32_to_bytes(3.5_f32), [0x40, 0x60, 0x00, 0x00]);
        assert_eq!(float32_to_bytes(-789.3145_f32), [0xC4, 0x45, 0x54, 0x21]);
        assert_eq!(float32_to_bytes(789.3145_f32), [0x44, 0x45, 0x54, 0x21]);
    }

    #[test]
    fn test_float20_to_bytes_conversion() {
        assert_eq!(float20_to_bytes(3.5_f32), [0x40, 0x60, 0x00, 0x00]);
        assert_eq!(float20_to_bytes(-789.3145_f32), [0xC4, 0x45, 0x54, 0x20]);
        assert_eq!(float20_to_bytes(789.3145_f32), [0x44, 0x45, 0x54, 0x20]);
    }

    #[test]
    fn test_bytes_to_float32_conversion() {
        approx::abs_diff_eq!(bytes_to_float32([0x40, 0x60, 0x00, 0x00]), 3.5_f32);
        approx::abs_diff_eq!(bytes_to_float32([0xC4, 0x45, 0x54, 0x21]), -789.3145_f32);
        approx::abs_diff_eq!(bytes_to_float32([0x44, 0x45, 0x54, 0x21]), 789.3145_f32);
    }

    #[test]
    fn test_bytes_to_float20_conversion() {
        approx::abs_diff_eq!(bytes_to_float20([0x40, 0x60, 0x00, 0x00]), 3.5_f32);
        approx::abs_diff_eq!(bytes_to_float20([0xC4, 0x45, 0x54, 0x20]), -789.3145_f32);
        approx::abs_diff_eq!(bytes_to_float20([0x44, 0x45, 0x54, 0x20]), 789.3145_f32);
    }

}
