//==============================================================================
// "detabify.rs"
//
// Part of detabify - replace TAB characters by a number of (default: 8) spaces
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const STDIN : &str = "/dev/stdin"; // TODO: fix for Windows?

fn usage() {
   eprint!("usage:\n\tdetabify [n]\n\n");
   std::process::exit(1);
}

fn detabify(input: &str, width: u32) {
   // open buffered line reader for file:
   let file = BufReader::new(File::open(&input).unwrap());
   // make string of width spaces:
   let spaced = format!("{: ^1$}", "", width as usize);
   for line in file.lines() {
       let my_line = line.unwrap();
       let output_line = my_line.replace("\t", &spaced);
       println!("{}", output_line);
    }
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let argc = args.len();
   if argc == 2 {
       let mut width = 8u32;
       match args[1].parse::<u32>() {
       	     Ok(n)  => width = n,
	     Err(_) => usage()
       }
       detabify(STDIN, width);
   }
   else if argc == 1 {
       detabify(STDIN, 8u32);
   }
   else {
       usage();
   }
}

//==============================================================================
// end of file "detabify.rs"
//==============================================================================
