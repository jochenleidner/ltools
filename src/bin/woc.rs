//============================================================================
// "woc.rs"
//
// Count and print word occurrences
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved
//============================================================================

use std::collections::HashMap;
use std::env;
use std::fs;

const USAGE_MSG : &str = "Usage:\n\twoc [--help|-h|<fileName>]\n";

fn usage() {
	eprint!("{}", USAGE_MSG);
}

/// Count word occurrences
pub fn word_count(words: &str) -> HashMap<String, u128> {
    let mut map = HashMap::new();
    for word in words
        .split(|chr: char| chr != '\'' && (chr.is_ascii_whitespace() ||
                                           chr.is_ascii_punctuation()))
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches(|chr: char| chr.is_ascii_punctuation()))
        .map(str::to_ascii_lowercase) {
			*map.entry(word).or_insert(0u128) += 1u128;
    	}
    map
}

fn count_words(file_name : &str) {
    match fs::read_to_string(file_name) {
		Ok(s)  => {
			let mut text = s; 
			let map = word_count(&mut text);
			for (k, v) in map {
				println!("{}\t{}", k, v);
			}
		}
		Err(_) => { eprintln!("ERROR reading file \"{}\"\n", file_name); }
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let argc : usize = args.len();
	match argc {
		1 => { count_words("/dev/stdin") },
		2 => { let argument = &args[1];
			   if argument == "-h" || argument == "--help" {
				  usage();
			   }
			   else {
				  count_words(argument);
			   }
			 },
		_ => { usage(); }
	}
}

//============================================================================
// end of file "woc.rs"
//============================================================================
