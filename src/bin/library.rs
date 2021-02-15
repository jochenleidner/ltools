//==============================================================================
// "library.rs"
//
// library - manage a library catalog of books - Part of LTools
//
// Copyright ©2021 by Jochen L. Leidner <leidner@acm.org>. All rights reserved.
//==============================================================================

use rusqlite::{NO_PARAMS, Connection, Result};
use rustyline::error::ReadlineError;
use uuid::Uuid;
use std::env;
use ::std::*;

const DATABASE_NAME    : &str = "library.db";
const READLINE_HISTORY : &str = ".library_history";
const DEBUG            : bool = false;

#[derive(Debug)]
struct Book {
    bookno    : i32,
    id        : String,
    pubtype   : String,
    authors   : String,
    editors   : String,
    title     : String,
    year      : String,
    edition   : String,
    location  : String,
    publisher : String,
}

/// print synopsis of use
fn usage() {
    println!("\nUsage:\n\tlibrary <command> [arguments...]\n\ncommands:\n\tnew\tcreate new empty database\n\tadd\tinsert new record\n\tdel\tdelete record with a given ID\n\tdump\tSQL dump of DB to stdout\n\tlist\thuman-friendly list of records\n\texport\trecords in BibTeX format\n\tsearch\tfind author or keywrods in title\n");
    std::process::exit(10);
}

fn input_string(prompt: &str) -> String {
    let mut rl = rustyline::Editor::<()>::new();
    let mut my_line : String = String::new();
    rl.load_history(READLINE_HISTORY);
    let readline = rl.readline(prompt);
    match readline {
        Ok(line) => {
	    my_line = line.as_str().to_string();
            rl.add_history_entry(&my_line);
        },
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
        }, 
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
    rl.save_history(READLINE_HISTORY).unwrap();
    my_line.trim().to_string()
}

fn list_book(book : Result<Book>) -> Result<()> {
    println!("{:?}", book); // TODO: create human-friendly output format
    Ok(())
}

fn book_to_sql(book : Result<Book>) -> Result<()> {
    println!("{:?}", book); // TODO: create SQL output format
    Ok(())
}

fn book_to_bibtex(book : Result<Book>) -> Result<()> {
    println!("{:?}", book); // TODO: create BibTeX output format
    Ok(())
}

fn forall_books(callback: fn (book: Result<Book>) -> Result<()>) -> Result<()> {
    let query = "select * from books;";
    let conn = Connection::open(DATABASE_NAME)?;
    let mut stmt = conn.prepare(&query, )?;
    let books = stmt.query_map(NO_PARAMS, |row| {
        Ok(Book {
            bookno    : row.get(0)?,
            id        : row.get(1)?,
            pubtype   : row.get(2)?,
            authors   : row.get(3)?,
            editors   : row.get(4)?,
            title     : row.get(5)?,
            year      : row.get(6)?,
            edition   : row.get(7)?,
            location  : row.get(8)?,
            publisher : row.get(9)?,
        })
       })?;
    for book in books {
        callback(book);
    }
    Ok(())
}

/// main program
fn main() -> Result<()> {
    // handle command line arguments:
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if (argc < 2) || ((argc == 2) && ((argv[1] == "--help") || (argv[1] == "-h"))) {
        usage();
    }
    let mut cmd_match : bool = false;

    // create a new, empty database:
        if (argc == 2) && (argv[1] == "new") {
        let conn = Connection::open(DATABASE_NAME)?;
        conn.execute(
            "create table if not exists books (
                bookno integer primary key not null unique,
                 id text,
                 pubtype text,
                 authors text,
                 editors text,
                 title text,
                 year text,
                 edition text,
                 location text,
                 publisher text
             )",
            NO_PARAMS,
        )?;
        cmd_match = true;
    }

    // enter a new publication record and store in the DB:
    if (argc == 2) && (argv[1] == "add") {
        let mut pubtype   = input_string("Type:  \t");
        if pubtype == "" {
            pubtype = "book".to_string();
        }
        let authors   = input_string("Authors:   \t");
        let editors   = input_string("Editors:   \t");
        let title     = input_string("Title:     \t");
        let year      = input_string("Year:      \t");
        let edition   = input_string("Edition:   \t");
        let location  = input_string("Location:  \t");
        let publisher = input_string("Publisher: \t");
        let id        = Uuid::new_v5(&Uuid::NAMESPACE_URL, &title.as_bytes()).to_hyphenated().to_string();
        let tuple = format!(
            "'{}','{}','{}','{}','{}','{}','{}','{}','{}'",
            id, pubtype, authors, editors, title, year, edition, location, publisher
        );
        let conn = Connection::open(DATABASE_NAME)?;
        let update = format!(
            "insert into books (id, pubtype, authors, editors, title, year, edition, location, publisher) values ({});",
            tuple
        );
	if DEBUG {
	  println!("{}", update);
	}
        conn.execute(&update, NO_PARAMS)?;
        cmd_match = true;
    }

    // search for an author or title:
    if (argc >= 3) &&
       (argv[1] == "search" || argv[1] == "query" || argv[1] == "s") {
        let query = format!(
            "select bookno, id, pubtype, authors, editors, title, year, edition, location, publisher from books where authors like '%{}%' or title like '%{}%';",
            argv[2], argv[2]
        );
        let conn = Connection::open(DATABASE_NAME)?;
        let mut stmt = conn.prepare(&query, )?;
        let books = stmt.query_map(NO_PARAMS, |row| {
            Ok(Book {
                bookno    : row.get(0)?,
                id        : row.get(1)?,
                pubtype   : row.get(2)?,
                authors   : row.get(3)?,
                editors   : row.get(4)?,
                title     : row.get(5)?,
                year      : row.get(6)?,
                edition   : row.get(7)?,
                location  : row.get(8)?,
                publisher : row.get(9)?,
            })
        })?;
        let mut count : i32 = 0;
        for book in books {
            count += 1;
            println!("#{}\t{:?}", count, book);
        }
        cmd_match = true;
    }

    // export as a human-friendly list:
    if (argc == 2) && (argv[1] == "list") {
           forall_books(list_book);
        cmd_match = true;
    }

    // export as a BibTeX file:
    if (argc == 2) && (argv[1] == "export") {
           eprint!("\n\n*** BibTeX export not implemented yet!\n\n"); // TODO: !! <----------------- fix me
           forall_books(book_to_bibtex);
        cmd_match = true;
    }

    // dump as a SQL commands:
    if (argc == 2) && (argv[1] == "dump") {
           eprint!("\n\n*** SQL dump not implemented yet!\n\n"); // TODO: !! <----------------- fix me
           forall_books(book_to_sql);
        cmd_match = true;
    }

    // remove a record from the DB:
    if (argc == 3) && (argv[1] == "del") {
        let which = &argv[2];
        let conn = Connection::open(DATABASE_NAME)?;
        let update = format!("delete from books where bookno = {};", which);
        conn.execute(&update, NO_PARAMS)?;
        cmd_match = true;
    }

    if !cmd_match {
        usage();
    }

    Ok(())
}

//==============================================================================
// end of file "library.rs"
//==============================================================================
