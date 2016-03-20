#![deny(warnings)]

extern crate binutils;

use std::env;
use std::fs;
use std::io;

use binutils::strings::read;
use binutils::extra::option::OptionalExt;
use binutils::extra::io::{WriteExt, fail};

const HELP: &'static [u8] = br#"
    NAME
        strings - inspect a binary file for strings of printable characters.
    SYNOPSIS
        strings [-h | --help] [FILE]
    DESCRIPTION
        This utility will read the file from the path given in the argument. If no argument is given, 'strings' will read from the standard input. The byte stream is then inspected for contiguous, printable ASCII characters of length 4 or more. These strings of printable characters are written to the standard output. Each contiguous strings are seperated by a newline (0x0A).

        This utility is useful for inspecting binary files for human readable information, to determine the contents. Note that all non-ASCII characters are treated as non-printable, due to the numerous false positives otherwise.

        This is a clone of GNU strings, though they differ in a number of ways.

    OPTIONS
        -h
        --help
            Print this manual page.
    AUTHOR
        This program was written by Ticki. Bugs should be reported in the Github repository, 'redox-os/binutils'.
    COPYRIGHT
        Copyright (c) 2016 Ticki

        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();
    let mut args = env::args();

    if args.len() > 2 {
        fail("too many arguments.", &mut stderr);
    }

    match args.nth(1) {
        None => {
            let stdin = io::stdin();
            read(stdin.lock(), stdout, stderr);
        }
        Some(a) => match a.as_ref() {
            "-h" | "--help" => {
                stdout.writeln(HELP).try(&mut stderr);
            },
            f => {
                let file = fs::File::open(f).try(&mut stderr);
                read(file, stdout, stderr);
            }
        },
    }
}
