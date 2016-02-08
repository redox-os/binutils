use std::mem;

/// Convert a base 256 (byte) to a hexadecimal representation
#[inline]
pub fn u8_to_hex(from: u8) -> (u8, u8) {
    (from >> 4, from & 0b1111)
}

/// Convert two hexadecimal digits to a byte
#[inline]
pub fn hex_to_u8((a, b): (u8, u8)) -> u8 {
    (a << 4) + b
}

/// Convert an u32 to an byte array. This operation is safe. It is an noop on big-endianness
#[inline]
pub fn u32_byte_array(int: u32) -> [u8; 4] {
    // Safe
    // ((int >> (32 - 8 - n * 8)) & 255) as u8
    // or unsafe:
    unsafe {
        mem::transmute::<_, [u8; 4]>(int.to_be())
    }
}

/// Convert hex to ascii
#[inline]
pub fn hex_to_ascii(b: u8) -> u8 {
    match b {
        0...9 => b'0' + b,
        _ => b'a' - 10 + b,
    }
}

/// Convert ascii to hex
#[inline]
pub fn ascii_to_hex(b: u8) -> u8 {
    match b {
        b'0'...b'9' => b - b'0',
        _ => b - b'a' + 10,
    }
}

/// Convert a base 256 (byte) to an octal representation
#[inline]
pub fn u8_to_oct(from: u8) -> (u8, u8, u8) {
    (from >> 6, (from >> 3) & 0b111, from & 0b111)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_u8() {
        for i in 0..255 {
            assert_eq!(hex_to_u8(u8_to_hex(i)), i);
        }
    }

    #[test]
    fn test_hex2ascii() {
        assert_eq!(hex_to_ascii(0x0), b'0');
        assert_eq!(hex_to_ascii(0x1), b'1');
        assert_eq!(hex_to_ascii(0x2), b'2');
        assert_eq!(hex_to_ascii(0x3), b'3');
        assert_eq!(hex_to_ascii(0x4), b'4');

        assert_eq!(hex_to_ascii(0xa), b'a');
        assert_eq!(hex_to_ascii(0xb), b'b');
        assert_eq!(hex_to_ascii(0xc), b'c');
    }

    #[test]
    fn ascii2hex() {
        for i in 0..16 {
            assert_eq!(ascii_to_hex(hex_to_ascii(i)), i);
        }
    }

    #[test]
    fn test_hex() {
        for i in 0..16 {
            assert_eq!(u8_to_hex(i), (0, i));
        }

        assert_eq!(u8_to_hex(0xDE), (0xD, 0xE));
        assert_eq!(u8_to_hex(0xAD), (0xA, 0xD));
        assert_eq!(u8_to_hex(0xBE), (0xB, 0xE));
        assert_eq!(u8_to_hex(0xAF), (0xA, 0xF));

        assert_eq!(u8_to_hex(0x12), (1, 2));
        assert_eq!(u8_to_hex(0xA5), (0xA, 5));
        assert_eq!(u8_to_hex(0x42), (4, 2));
    }

    #[test]
    fn test_oct() {
        assert_eq!(u8_to_oct(0o102), (1, 0, 2));
        assert_eq!(u8_to_oct(0o002), (0, 0, 2));
        assert_eq!(u8_to_oct(0o000), (0, 0, 0));
        assert_eq!(u8_to_oct(0o111), (1, 1, 1));
        assert_eq!(u8_to_oct(0o277), (2, 7, 7));
    }

    #[test]
    fn u32_byte() {
        for i in 0..255923 {
            assert_eq!(u32_byte_array(i)[3], (i & 255) as u8);
            assert_eq!(u32_byte_array(i)[0], (i >> (8 * 3)) as u8);
            for n in 0..4 {
                assert_eq!(u32_byte_array(i)[n], ((i >> (32 - 8 - n * 8)) & 255) as u8)
            }
        }
    }
}
