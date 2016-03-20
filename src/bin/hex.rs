#![deny(warnings)]

extern crate binutils;

use std::env;
use std::fs;
use std::io::{self, Stderr, Write, Read};

use binutils::extra::option::OptionalExt;
use binutils::extra::io::{WriteExt, fail};
use binutils::convert::{u8_to_hex, hex_to_u8, ascii_to_hex, hex_to_ascii};

const HELP: &'static [u8] = br#"
    NAME
        hex - read a binary file and output it in hexadecimal representation.
    SYNOPSIS
        hex [-h | --help] [-d | --decode] [FILE]
    DESCRIPTION
        This utility will read the file from the path given in the argument. If no argument is given, 'hex' will read from the standard input. The content of the file is then encoded/decoded in/from hexadecimal.

        In opposite to GNU Hexdump, 'hex' will treat the input as big endianness left-to-right byte stream. Furthermore, there is no stylistic representation, the output is just plain ASCII, with no spaces or new-lines for seperations.
    OPTIONS
        -h
        --help
            Print this manual page.
        -d
        --decode
            Decode hexadecimal.
    AUTHOR
        This program was written by Ticki. Bugs should be reported in the Github repository, 'redox-os/binutils'.
    COPYRIGHT
        Copyright (c) 2016 Ticki

        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#;

fn encode<R: Read, W: Write>(stdin: R, mut stdout: W, mut stderr: Stderr) {
    // Encode the input stream to hexadecimal output stream.

    for i in stdin.bytes() {
        let (a, b) = u8_to_hex(i.try(&mut stderr));
        stdout.write(&[hex_to_ascii(a), hex_to_ascii(b)]).try(&mut stderr);
    }
}

fn decode<R: Read, W: Write>(stdin: R, mut stdout: W, mut stderr: Stderr) {
    // Decode hexadecimal to base-256, raw byte stream.

    let mut iter = stdin.bytes();
    loop {
        let i = if let Some(x) = iter.next() {
            x.try(&mut stderr)
        } else {
            break
        };
        let j = if let Some(x) = iter.next() {
            x.try(&mut stderr)
        } else {
            break
        };

        stdout.write(&[hex_to_u8((ascii_to_hex(i), ascii_to_hex(j)))]).try(&mut stderr);
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();

    let mut args = env::args();
    if args.len() > 2 {
        fail("too many arguments.", &mut stderr);
    }

    match args.nth(1) {
        None => encode(io::stdin(), stdout, stderr),
        Some(a) => match a.as_ref() { // MIR plz
            "-h" | "--help" => {
                stdout.writeln(HELP).try(&mut stderr);
            },
            "-d" | "--decode" => {
                match args.next() {
                    Some(f) => {
                        let file = fs::File::open(f).try(&mut stderr);
                        decode(file, stdout, stderr);
                    },
                    None => {
                        let stdin = io::stdin();
                        decode(stdin.lock(), stdout, stderr);
                    },
                }
            },
            // Read from file instead of standard input.
            f => {
                let file = fs::File::open(f).try(&mut stderr);
                encode(file, stdout, stderr);
            },
        },
    }
}
