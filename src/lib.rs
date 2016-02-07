//! Binutils: The distribution of utilities to process, read, and write binary files for the
//! Redox operating system.

// If this code works, it was written by Ticki. If it does not, I don't know who the hell wrote it
// but it was definitively not me. Blame someone else.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unused_mut)]

/// Scan a byte stream for printable strings of 4 or more bytes.
pub mod strings;

/// Extra helper functionality
pub mod extra;

