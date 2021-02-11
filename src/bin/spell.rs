//============================================================================
// "spell.rs"
//
// Part of spell, a naive spell-checker in Rust
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    collections::HashMap,
	path::Path,
};
use regex::Regex;

const DEFAULT_FILE_NAME : &str = ".dict.en";

fn main() -> Result<(), Box<dyn Error>> {

    // parse command line arguments:
    let args: Vec<String> = env::args().collect();
	let argc = args.len();
    if (argc != 2) && (argc != 3) {
        println!("Usage:\n\tspell <text-file> [<dictionary-file>]\n");
        return Ok(());
    }
    let text_file_name = File::open(&args[1])?;
    let text_reader = BufReader::new(&text_file_name);

    // load dictionary:
    let my_home : String;
    #[allow(deprecated)]
    match std::env::home_dir() {
        Some(path) => {		let s = path.display().to_string();
                        	my_home = s.clone()
                      }
        None       => {		eprintln!("Error obtaining home directory");
	  	     	  	std::process::exit(20)
		      }
    }
    let mut dict = HashMap::new();
    let p = Path::new(&my_home).join(DEFAULT_FILE_NAME);
    let mut dict_file_name = p.to_str().unwrap();
    if argc == 3 {
    	dict_file_name = &args[2];
    }
    let dict_file = File::open(dict_file_name)?;
    let dict_file_reader = BufReader::new(&dict_file);
    for line in dict_file_reader.lines() {
        let word = line?.trim().to_string();
	  	dict.insert(word, ""); // TODO: use e.g. a serialized trie
    }

    // for each line in text, tokenize & look up in spelling lexicon:
    for line in text_reader.lines() {
        let line = line?.trim().to_string();
	let regex = Regex::new(r"[\n\d\-\s,.;:!?]+").unwrap();
	// Execute the regex to tokenize:
	let v = regex.split(&line).collect::<Vec<_>>();
	// check if in dictionary:
	for w in v {
	    if w.len() > 1 {
	       let has_match;
	       match dict.get(w) {
	    	  Some(_) => has_match = true,
		  None    => match dict.get(&w.to_lowercase()) {
	    	  	        Some(_) => has_match = true,
		  		None    => has_match = false
	       		     }
	       }
	       if !has_match {
	          println!("{}", w);
	       }
	    }
	}
    }

    Ok(())
}

//============================================================================
// end of file "spell.rs"
//============================================================================
