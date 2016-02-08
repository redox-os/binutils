/// Convert a base 256 (byte) to a hexadecimal representation
#[inline]
pub fn to_hex(from: u8) -> (u8, u8) {
    (from >> 4, from & 0b1111)
}

/// Convert two hexadecimal digits to a byte
#[inline]
pub fn from_hex((a, b): (u8, u8)) -> u8 {
    (a << 4) + b
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
pub fn to_oct(from: u8) -> (u8, u8, u8) {
    (from >> 6, (from >> 3) & 0b111, from & 0b111)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        for i in 0..255 {
            assert_eq!(from_hex(to_hex(i)), i);
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
            assert_eq!(to_hex(i), (0, i));
        }

        assert_eq!(to_hex(0xDE), (0xD, 0xE));
        assert_eq!(to_hex(0xAD), (0xA, 0xD));
        assert_eq!(to_hex(0xBE), (0xB, 0xE));
        assert_eq!(to_hex(0xAF), (0xA, 0xF));

        assert_eq!(to_hex(0x12), (1, 2));
        assert_eq!(to_hex(0xA5), (0xA, 5));
        assert_eq!(to_hex(0x42), (4, 2));
    }

    #[test]
    fn test_oct() {
        assert_eq!(to_oct(0o102), (1, 0, 2));
        assert_eq!(to_oct(0o002), (0, 0, 2));
        assert_eq!(to_oct(0o000), (0, 0, 0));
        assert_eq!(to_oct(0o111), (1, 1, 1));
        assert_eq!(to_oct(0o277), (2, 7, 7));
    }
}
