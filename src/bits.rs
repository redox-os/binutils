use std::ops::Range;

/// A trait for slicing integer's bits
pub trait BitSlice : Sized {
    /// Slice an integer's bits by a range
    fn bit_slice(self, range: Range<Self>) -> Self;
}

impl BitSlice for u8 {
    fn bit_slice(self, range: Range<Self>) -> Self {
        self << range.start >> range.start >> (8 - range.end)
    }
}

#[test]
fn bit_slice_test() {
    assert_eq!(0b10000100.bit_slice(1..2), 0);
    assert_eq!(0b10000100.bit_slice(0..2), 0b10);
    assert_eq!(0b10001110.bit_slice(4..8), 0b1110);
    assert_eq!(0b00001110.bit_slice(0..4), 0);
    assert_eq!(0b00001110.bit_slice(3..8), 0b1110);
}
