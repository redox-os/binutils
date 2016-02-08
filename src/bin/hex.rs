#[deny(warnings)]
#[deny(unused_mut)]

extern crate binutils;

use std::env;
use std::fs;
use std::io;
use std::process::exit;
use std::io::{Write, Read};

use binutils::extra::OptionalExt;
use binutils::convert::{to_hex, from_hex};

const HELP: &'static [u8] = br#"
    NAME
        hex - read a binary file and output it in hexadecimal representation
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
    COPYRIGHT
        Copyright (c) 2016 Ticki

        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#;

fn to_ascii(b: u8) -> u8 {
    match b {
        0...9 => b'0' + b,
        _ => b'a' - 10 + b,
    }
}

fn from_ascii(b: u8) -> u8 {
    match b {
        b'0'...b'9' => b - b'0',
        _ => b - b'a' + 10,
    }
}

fn encode<R: Read, W: Write>(stdin: R, mut stdout: W) {
    for i in stdin.bytes() {
        let (a, b) = to_hex(i.try());
        stdout.write(&[to_ascii(a), to_ascii(b)]).try();
    }
}

fn decode<R: Read, W: Write>(stdin: R, mut stdout: W) {
    let mut iter = stdin.bytes().map(|x| x.try());
    loop {
        let i = if let Some(x) = iter.next() {
            x
        } else {
            break
        };
        let j = if let Some(x) = iter.next() {
            x
        } else {
            break
        };

        stdout.write(&[from_hex((from_ascii(i), from_ascii(j)))]).try();
    }
}

fn main() {
    let mut stdout = io::stdout();
    let mut args = env::args();
    if args.len() > 2 {
        println!("error: Too many arguments. Try 'hex -h'.");
        exit(1);
    }

    match args.nth(1) {
        None => encode(io::stdin(), stdout),
        Some(a) => match a.as_ref() { // MIR plz
            "-h" | "--help" => {
                stdout.write(HELP).try();
            },
            "-d" | "--decode" => {
                match args.next() {
                    Some(f) => decode(fs::File::open(f).try(), stdout),
                    None => decode(io::stdin(), stdout),
                }
            },
            f => encode(fs::File::open(f).try(), stdout),
        },
    }
}
