extern crate clap;
extern crate minigrep;

use clap::{App, Arg};
use std::process;

use minigrep::Config;

fn main() {
    let args = App::new("minigrep")
        .version("1.3.7")
        .arg(Arg::with_name("query").required(true).index(1))
        .arg(Arg::with_name("filename").required(true).index(2))
        .arg(Arg::with_name("case-insensitive").env("CASE_INSENSITIVE"))
        .get_matches();

    let config = Config::new(args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
