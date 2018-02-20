extern crate minigrep;
extern crate clap;

use clap::{Arg, App};
use std::process;

use minigrep::Config;

fn main() {
    let args = App::new("minigrep")
        .version("1.3.7")
        .arg(Arg::with_name("query").index(1))
        .arg(Arg::with_name("filename").index(2))
        .get_matches();
    
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
