use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep;

// $ cargo run test poem.txt
// $ cargo run frog poem.txt
// $ CASE_INSENSITIVE=1 cargo run to poem.txt
// $ cargo run to poem.txt > output.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
