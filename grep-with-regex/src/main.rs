use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn process_lines <T: BufRead + Sized> (reader: T, re: Regex){
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
        .help("THe pattern to search for")
        .takes_value(true)
        .default_value("Do")
        .required(true))
    .arg(Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .default_value("quote.md")
        .required(true))
    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or_default();
    let f = File::open(input).unwrap();
    let mut reader = BufReader::new(f);

    process_lines(reader, re);

}
