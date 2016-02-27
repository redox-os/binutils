#![deny(warnings)]

extern crate binutils;

use std::env;
use std::fs;
use std::io;
use std::io::{Write, Read};
use std::mem;

use binutils::extra::{OptionalExt, fail, println, print};
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

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#;

fn encode_byte<R: Read, W: Write>(stdin: &mut R, mut stdout: &mut W) -> Option<u8> {
    let byte = if let Some(x) = stdin.bytes().next() {
        x.try(&mut stdout)
    } else {
        return None;
    };

    let hex = u8_to_hex(byte);
    print(&[hex_to_ascii(hex.0), hex_to_ascii(hex.1)], &mut stdout);

    Some(if byte.is_printable() {
        byte
    } else {
        b'.'
    })
}

fn encode<R: Read, W: Write>(mut stdin: R, mut stdout: W) {
    let rem;
    let mut ascii: [u8; 16] = unsafe { mem::uninitialized() };
    let mut line = 0;

    'a: loop {
        for &b in u32_byte_array(line * 16).iter() {
            let hex = u8_to_hex(b);
            print(&[hex_to_ascii(hex.0), hex_to_ascii(hex.1)], &mut stdout);
        }
        print(b": ", &mut stdout);

        for n in 0..8 {
            ascii[n * 2] = if let Some(x) = encode_byte(&mut stdin, &mut stdout) {
                x
            } else {
                rem = n;
                break 'a;
            };
            ascii[n * 2 + 1] = if let Some(x) = encode_byte(&mut stdin, &mut stdout) {
                x
            } else {
                rem = n;
                break 'a;
            };
            print(b" ", &mut stdout);
        }

        print(b" ", &mut stdout);
        println(&ascii, &mut stdout);

        line += 1;
    }

    if rem != 0 {
        for _ in 0..41 - rem * 5 {
            print(b" ", &mut stdout);
        }
        println(&ascii[..rem * 2], &mut stdout);
    }

}

fn decode<R: Read, W: Write>(stdin: R, mut stdout: W) {
    let mut stdin = stdin.bytes().filter(|x| x.as_ref().ok() != Some(&b' '));

    loop {
        stdin.nth(8); // Skip the first column
        for _ in 0..16 { // Process the inner 8 columns
            let h1 = ascii_to_hex(
                if let Some(x) = stdin.next() {
                    x.try(&mut stdout)
                } else {
                    return;
                }
            );
            let h2 = ascii_to_hex(
                if let Some(x) = stdin.next() {
                    x.try(&mut stdout)
                } else {
                    return;
                }
            );

            print(&[hex_to_u8((h1, h2))], &mut stdout);
        }

        loop {
            if let Some(x) = stdin.next() {
                if x.try(&mut stdout) == b'\n' {
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
    let mut args = env::args();
    if args.len() > 2 {
        fail("error: Too many arguments. Try 'hexdump -h'.", &mut stdout);
    }

    match args.nth(1) {
        None => encode(io::stdin(), stdout),
        Some(a) => match a.as_ref() { // MIR plz
            "-h" | "--help" => {
                println(HELP, &mut stdout);
            },
            "-r" | "--reverse" => {
                match args.next() {
                    None => {
                        let stdin = io::stdin();
                        decode(stdin.lock(), stdout);
                    }
                    Some(f) => {
                        let file = fs::File::open(f).try(&mut stdout);
                        decode(file, stdout);
                    }
                }
            },
            f => {
                let file = fs::File::open(f).try(&mut stdout);
                encode(file, stdout);
            },
        },
    }
}
