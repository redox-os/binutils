//! Binutils: The distribution of utilities to process, read, and write binary files for the
//! Redox operating system.

// If this code works, it was written by Ticki. If it does not, I don't know who the hell wrote it
// but it was definitively not me. Blame someone else.

#![deny(warnings)]
#![deny(missing_docs)]

#[macro_use]
pub extern crate extra;

/// Scan a byte stream for printable strings of 4 or more bytes.
pub mod strings;

/// Primitives for processing bits.
pub mod bits;

/// Converting between bases and endianesses
pub mod convert;
