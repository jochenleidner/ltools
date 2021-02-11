//=============================================================================
// "logmsg.rs"
//
// Part of logmsg, part of LTools 
//
// Example:
//   $ logmsg "Running low on disk space" "WARN"
//
// Written 2021-02-08 by Jochen L. Leidner <leidner@acm.org>.
//=============================================================================

use std::env;
use ansi_term::Colour;
use ansi_term::Colour::RGB;


fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if  argc == 2 || argc == 3 {
        let msg_text = &args[1];
        let status_text = if argc == 3 { &args[2] } else { "INFO" };

	let color = match status_text {
	   "ERROR"     => Colour::Red.paint(status_text),
           "WARNING"   => RGB(255,127,80).paint(status_text),
           "WARN"      => Colour::Red.paint(status_text),
           "OK"        => Colour::Green.paint(status_text),
           "OKAY"      => Colour::Green.paint(status_text),
           "SUCCESS"   => Colour::Green.paint(status_text),
           "COMPLETE"  => Colour::Green.paint(status_text),
           "COMPLETED" => Colour::Green.paint(status_text),
	   _           => Colour::White.paint(status_text)
	};
	println!("{msg:.<70}.[{status:5}]\n", msg=msg_text, status=color)
    }
    else {
        eprintln!("\nUsage:\n\tlogmsg <msg> <status>\n");
    }
}

//=============================================================================
// end of file "logmsg.rs"
//=============================================================================
