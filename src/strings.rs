use std::io::{Write, Read};

use extra::{OptionalExt, print};

/// A trait for characters/bytes that can be printable.
pub trait IsPrintable {
    /// Is this character printable?
    fn is_printable(self) -> bool;
}

impl IsPrintable for u8 {
    #[inline]
    fn is_printable(self) -> bool {
        // TODO handle unicode.
        self >= 0x20 && self <= 0x7e
    }
}

#[derive(Copy, Clone)]
struct Trailing {
    chars: [u8; 4],
    current: usize,
}

// Wow, such premature, much optimization
#[allow(dead_code)] // DAFUQ rustc?
impl Trailing {
    #[inline]
    fn new() -> Trailing {
        Trailing {
            chars: [0; 4],
            current: 0,
        }
    }

    #[inline]
    fn set(&mut self, b: u8) -> bool {
        self.chars[self.current] = b;
        self.current += 1;

        self.is_complete()
    }

    #[inline]
    fn reset(&mut self) {
        self.current = 0;
    }

    #[inline]
    fn is_complete(self) -> bool {
        self.current == 4
    }

    #[inline]
    fn chars(self) -> [u8; 4] {
        self.chars
    }
}

/// Read a stream of bytes and output printable strings of length 4 or more seperated by 0x0A
/// (NL)
pub fn read<R: Read, W: Write>(stdin: R, mut stdout: W) {
    let mut trailing = Trailing::new();

    for i in stdin.bytes() {
        let i = i.try(&mut stdout);

        if i.is_printable() {
            if trailing.is_complete() {
                print(&[i], &mut stdout);
            } else if trailing.set(i) {
                print(&trailing.chars(), &mut stdout);
            }
        } else {
            if trailing.is_complete() {
                print(b"\n", &mut stdout);
            }
            trailing.reset();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn printable() {
        assert!(!b'\0'.is_printable());
        assert!(!b'\t'.is_printable());
        assert!(!b'\n'.is_printable());
        assert!(!b'\r'.is_printable());
        assert!(!b'\x1b'.is_printable());
        assert!(b'a'.is_printable());
        assert!(b'B'.is_printable());
        assert!(b'x'.is_printable());
        assert!(b'~'.is_printable());
    }
}
