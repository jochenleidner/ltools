//==============================================================================
// "xmastree.rs"
//
// xmastree, part of LTools
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use chrono::Datelike;
use chrono::Utc;
use std::convert::TryFrom;
use std::env;


fn usage() {
   eprint!("usage:\n\txmastree [-i|<height> <width> <year>]\n\n");
   std::process::exit(1);
}


#[allow(unreachable_code)]
fn get_args() -> (bool, usize, usize, usize) {
    let args: Vec<String> = env::args().collect();
    // if in interactive mode:
    if (args.len() == 2) && (args[1] == "-i" || args[1] == "--interactive") {
       let now = Utc::now();
       let (_is_common_era, year) = now.year_ce(); // (**)
       match usize::try_from(year) {
         Ok(value) => { (true, 5, 5, value) },
         Err(_)    => { (true, 5, 5, 2020) },
       }
    }
    else if args.len() == 4 {
       let mut height : usize = 5;
       let mut width  : usize = 5;
       let mut year   : usize = 2020;
       match args[1].parse::<usize>() {
             Ok(n)  => height = n,
             Err(_) => usage()
       }
       match args[2].parse::<usize>() {
             Ok(n)  => width = n,
             Err(_) => usage()
       }
       match args[3].parse::<usize>() {
             Ok(n)  => year = n,
             Err(_) => usage()
       }
       (false, height, width, year)
    }
    else {
       usage();
       (false, 5usize, 5usize, 2020usize)
    }
}


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn input_number(text : &str) -> usize {
   println!("{}", text);
   let n : usize = get_input().trim().parse::<usize>().unwrap();
   n
}


// ----------------------------------------------------------------------------

fn main() {
    let ret : (bool, usize, usize, usize) = get_args();
    // Rust doesn't permit destructuring assignments yet. Soon this will be:
    //   let ret : (is_interactive_mode, height, width, year) = get_args();
    // [but see comment "(**)" above.]
    let is_interactive_mode : bool     = ret.0;
    let mut height : usize             = ret.1;
    let mut width  : usize             = ret.2;
    let mut year   : usize             = ret.3;

    if height > 20 || width > 20 {
       eprint!("parameter out of range (3..20)!\n");
       usage();
    }
    
    if is_interactive_mode  {
        println!("Christmas Tree\n");
        height = input_number("Please enter height (3..20, e.g. 5): ");
        width  = input_number("\nwidth (3..20, e.g. 7): ");
        year   = input_number("\nYear (e.g. 2021): ");
    }
    
    for _a in 1usize..height {
        for b in 1usize..height {
            let s = format!("{: ^1$}", "", (40usize - width - b));
            print!("{}", s);
            let s = format!("{:*^1$}", "", (b * 2 - 1));
            println!("{}", s);
        }
    }
    for _b in 1usize..height {
        let s = format!("{: ^1$}", "***", 80usize - (2 * width));
        println!("{}", s);
    }

    println!("\n\nI wish You a Merry Christmas and a wonderful {}!", year + 1);
}

//==============================================================================
// end of file "xmastree.rs"
//==============================================================================
