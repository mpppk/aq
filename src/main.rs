pub mod processor;
pub mod parser;

use std::io::{self, Read};

extern crate serde_json;
use serde_json::Value;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("aq")
        .version("0.0.1")
        .author("niboshi")
        .about("jq like data processor for multi format")
        .arg(Arg::with_name("query")
            .help("Filter input"))
        .get_matches();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let parser = parser::parse_SelectorList(matches.value_of("query").unwrap()).unwrap();
    let data: Value = serde_json::from_str(&buffer).unwrap();
    let adapter = match &parser[0] {
        &processor::Processor::Adapter(ref a) => a,
    };

    println!("{}", serde_json::to_string_pretty(&adapter(&data).unwrap()).unwrap());
}
