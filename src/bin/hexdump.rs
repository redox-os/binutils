#![deny(warnings)]

extern crate binutils;

use std::env;
use std::fs;
use std::io::{self, Write, Read, Stderr};
use std::mem;

use binutils::extra::option::OptionalExt;
use binutils::extra::io::{WriteExt, fail};
use binutils::convert::{u8_to_hex, hex_to_u8, u32_byte_array, hex_to_ascii, ascii_to_hex};
use binutils::strings::IsPrintable;

const HELP: &'static [u8] = br#"
    NAME
        hexdump - dump the hexidecimal representation of a byte stream.
    SYNOPSIS
        hexdump [-h | --help] [-r | --reverse] [FILE]
    DESCRIPTION
        This utility will dump the hexidecimal representation of a file or the standard input, in a stylized way. Hexdump utility behaves like 'xxd'.

        The first column signifies the address of the first byte on the line. Each line contains 16 bytes, grouped in groups of two bytes, sepereated by space. The last column contains the printable characters in the last 16 bytes. The non-printable characters are replaced by a '.'.
    OPTIONS
        -h
        --help
            Print this manual page.
        -r
        --reverse
            Do the reverse dump (consume the dump and output the bytes it defines). This is useful for usage within editors.
    AUTHOR
        This program was written by Ticki. Bugs should be reported in the Github repository, 'redox-os/binutils'.
    COPYRIGHT
        Copyright (c) 2016 Ticki

        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

        Someone once read this. True story, bruh.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#;

/// Encode a single byte to the output stream
fn encode_byte<R: Read, W: Write>(stdin: &mut R, mut stdout: &mut W, stderr: &mut Stderr) -> Option<u8> {
    let byte = if let Some(x) = stdin.bytes().next() {
        x.try(&mut *stderr)
    } else {
        return None;
    };

    // Convert the raw byte to hexadecimal
    let hex = u8_to_hex(byte);

    // Write it to stdout.
    stdout.write(&[hex_to_ascii(hex.0), hex_to_ascii(hex.1)]).try(stderr);

    Some(if byte.is_printable() {
        byte
    } else {
        // If it is non-printable, write `.` to char-buffer instead.
        b'.'
    })
}

fn encode<R: Read, W: Write>(mut stdin: R, mut stdout: W, mut stderr: Stderr) {
    let rem;
    // This is the char buffer. The last 16 byte column, which prints the printable characters.
    // The non-printable ones are replaced with `.`.
    let mut ascii: [u8; 16] = unsafe { mem::uninitialized() };

    let mut line = 0;

    'a: loop {
        // Iterate over the bytes in the line number times 16.
        for &b in u32_byte_array(line * 16).iter() {
            let hex = u8_to_hex(b);
            // Print this value to the first column (denoting the address of the first byte of the
            // line.)
            stdout.write(&[hex_to_ascii(hex.0), hex_to_ascii(hex.1)]).try(&mut stderr);
        }
        stdout.write(b": ").try(&mut stderr);

        // Now, we go over the actual data, printing it in hexadecimal.
        for n in 0..8 {
            // We add the char to the char buffer, and print it in two hex digits.
            ascii[n * 2] = if let Some(x) = encode_byte(&mut stdin, &mut stdout, &mut stderr) {
                x
            } else {
                // The end of the file is reached, set the remainder, which will later be used for
                // alignment of the last column.
                rem = n;
                break 'a;
            };
            ascii[n * 2 + 1] = if let Some(x) = encode_byte(&mut stdin, &mut stdout, &mut stderr) {
                x
            } else {
                rem = n;
                break 'a;
            };
            // Seperate every two hex digits by a space.
            stdout.write(b" ").try(&mut stderr);
        }

        stdout.write(b" ").try(&mut stderr);
        // Print the ASCII buffer at the end of the line.
        stdout.writeln(&ascii).try(&mut stderr);

        // Increment the line number.
        line += 1;
    }

    if rem != 0 {
        // We now align the last column using the remainder set before.
        for _ in 0..41 - rem * 5 {
            stdout.write(b" ").try(&mut stderr);
        }
        stdout.write(&ascii[..rem * 2]).try(&mut stderr);
    }

    stdout.write(b"\n").try(&mut stderr);
}

fn decode<R: Read, W: Write>(stdin: R, mut stdout: W, mut stderr: Stderr) {
    let mut stdin = stdin.bytes().filter(|x| x.as_ref().ok() != Some(&b' '));

    loop {
        // Skip the first column
        stdin.nth(8);
        // Process the inner 8 columns
        for _ in 0..16 {
            // The first hex digit to decode.
            let h1 = ascii_to_hex(
                if let Some(x) = stdin.next() {
                    x.try(&mut stderr)
                } else {
                    return;
                }
            );
            // The second hex digit to decode.
            let h2 = ascii_to_hex(
                if let Some(x) = stdin.next() {
                    x.try(&mut stderr)
                } else {
                    return;
                }
            );

            // Write the decoded, joined hex digits in binary form.
            stdout.write(&[hex_to_u8((h1, h2))]).try(&mut stderr);
        }

        // Skip the rest until newline.
        loop {
            if let Some(x) = stdin.next() {
                if x.try(&mut stderr) == b'\n' {
                    break;
                }
            } else {
                return;
            }
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();

    let mut args = env::args();

    // Arguments should be <= 2
    if args.len() > 2 {
        fail("too many arguments.", &mut stderr);
    }

    match args.nth(1) {
        None => encode(io::stdin(), stdout, stderr),
        Some(a) => match a.as_ref() { // MIR plz
            "-h" | "--help" => {
                // HEEEEEELP.
                stdout.writeln(HELP).try(&mut stderr);
            },
            "-r" | "--reverse" => {
                match args.next() {
                    // Decode.
                    None => {
                        let stdin = io::stdin();
                        decode(stdin.lock(), stdout, stderr);
                    }
                    // Encode.
                    Some(f) => {
                        let file = fs::File::open(f).try(&mut stderr);
                        decode(file, stdout, stderr);
                    }
                }
            },
            // Read from a file, instead of standard input.
            f => {
                let file = fs::File::open(f).try(&mut stderr);
                encode(file, stdout, stderr);
            },
        },
    }
}
