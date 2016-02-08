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

#[test]
fn test_from_hex() {
    for i in 0..255 {
        assert_eq!(from_hex(to_hex(i)), i);
    }
}

/// Convert a base 256 (byte) to an octal representation
#[inline]
pub fn to_oct(from: u8) -> (u8, u8, u8) {
    (from >> 6, (from >> 3) & 0b111, from & 0b111)
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
