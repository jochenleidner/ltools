//==============================================================================
// "sregex.rs"
//
// sregex - read a file (or stdin), replace all occurrences of one or more
// regular expression patterns and print the result to stdout
// Part of LTools
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use fs::File;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Read;

fn usage() {
    eprintln!("usage:\n\treplregex [<file>] <pattern> <replacement-text>\n");
    std::process::exit(1);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc > 4 || argc < 3 {
        usage();
    }

    let mut file_name = "/dev/stdin";
    let pattern     : &str = &argv[argc - 2];
    let replacement : &str = &argv[argc - 1];
    if argc == 4 {
        file_name = &argv[1];
    }

    let mut text = String::new();
    match File::open(file_name) {
        Ok(mut file) => {
            file.read_to_string(&mut text).unwrap();
        },
        Err(error) => {
            eprintln!("Error opening file {} to read: {}", file_name, error);
            std::process::exit(10);
        },
    }

    let re = Regex::new(pattern).unwrap();
    let new_text = re.replace_all(&text, replacement);

    println!("{}", new_text);
}

//==============================================================================
// end of file "sregex.rs"
//==============================================================================
