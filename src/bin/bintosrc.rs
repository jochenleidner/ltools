//=============================================================================
// "bintosrc.rs"
//
// bintosrc - generate C source code from binary file for a static data object
//
// Example:
//   $ bintosrc < badger.gif > badger.c
//   $ cc -c badger.c
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//=============================================================================

use std::io;
use std::io::Read;
use std::env;

static HEADER_C : &str =
  "/* compile with: cc -c data.c */\nstatic const unsigned char data[] = {";
static HEADER_CPP : &str =
  "// compile with: c++ -c data.cc\nstatic const unsigned char data[] = {";
static HEADER_R : &str =
  "// compile with: rustc data.rs\nstatic data : Vec!<u8> = [";
// TODO: test Rust static initializer by compiling ist!
static TRAILER: &str =
  "\n];\n\n";

fn main() {
   // TODO: add proper handling of options
   let _args: Vec<String> = env::args().collect();
   let mut header = HEADER_C;
   let arg1 = "-l";
   let arg2 = "c";
   if arg1 == "-l" {
      match arg2 {
      	    "c++"  => header = HEADER_CPP,
      	    "rust" => header = HEADER_R,
	    &_     => header = HEADER_C
      }
   }
   println!("{}", header);
   let mut first_time : bool = true;

   // while not end of file, read a byte..
   let mut count : u64 = 1;
   for i in io::stdin().bytes() {
       let b = i.unwrap();
       if !first_time {
       	  print!(", ");
          if count == 16 {
             print!("\n  ");
             count = 1;
          }
          else {
               count += 1;
          }
       }
       else {
       	    first_time = false;
            print!("  ");
       }
       print!("{:#04x}", b); // ..and write it out in hex
   }
   print!("{}", TRAILER);
}

//=============================================================================
// end of file "bintosrc.rs"
//=============================================================================
