//! Yalda Grep Tool
//!
//! This tool allows users to search for a query string in a text file.
//!
//! # Example
//!
//! ```
//! use yalda_grep::{run, GrabConfig};
//!
//! fn main() {
//!     let config = GrabConfig {
//!         query: String::from("search"),
//!         file_path: String::from("example.txt"),
//!         ignore_case: true,
//!     };
//!
//!     if let Err(e) = run(config) {
//!         eprintln!("Error: {}", e);
//!     }
//! }
//! ```
use std::{env, process};


use grab::GrabConfig;
use run::run;

fn main() {
    println!("\n Yalda Grep Tool \n");



    let config = GrabConfig::grabargs(env::args()).unwrap_or_else(|err|{
        eprintln!("ERROR : {err}");
        process::exit(1);
    });

    
    println!("Searching for : {}\n",config.query);
    println!("In file : {}\n",config.file_path);



    if let Err(e) = run(config) {
        eprintln!("ERROR : {e}");
        process::exit(1);
    }
}