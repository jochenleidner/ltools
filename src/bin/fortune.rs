//==============================================================================
// "fortune.rs"
//
// Part of fortune - output random message of the day or memorable quote from
// file separated by empty lines
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use fs::File;
use rand::Rng;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

const DEFAULT_FILE_NAME : &str = ".fortune";

fn usage() {
   eprintln!("usage:\n\tfortune [<fortune_cookie_file_name>]\n");
   std::process::exit(1);
}

fn fortune(file_name : &str) -> String {
    let mut fortunes = String::new();

    match File::open(file_name) {
        Ok(mut file) => {
            file.read_to_string(&mut fortunes).unwrap();
        },
        Err(error) => {
            eprintln!("Error opening file {} to read: {}", file_name, error);
            std::process::exit(10);
        },
    }

    let cookie_vec: Vec<&str> = fortunes.split("\n\n").collect();
    let n = cookie_vec.len();

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..n-1);
    
    cookie_vec[i].to_string()
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let s : String;
    let my_home : String;
    #[allow(deprecated)]
    match std::env::home_dir() {
    	  Some(path) => { s = path.display().to_string();
    		     	  my_home = s.clone()
	  	        }
          None       => { eprintln!("Error while obtaining home directory");
	  	     	  std::process::exit(20)
			}
    }

    if argc == 1 {
       let p = Path::new(&my_home).join(DEFAULT_FILE_NAME);
       let default_file_name : &str = p.to_str().unwrap();

       print!("\n{}\n\n", fortune(default_file_name));
    }
    else if argc == 2 {
       print!("\n{}\n\n", fortune(&argv[1]));
    }
    else {
       usage();
    }
}

//==============================================================================
// end of file "fortune.rs"
//==============================================================================
