pub mod processor;
pub mod parser;

extern crate serde_json;
use serde_json::Value;

fn main() {

    use std::io::{self, Read};

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let parser = parser::parse_SelectorList(".foo[]").unwrap();
    let data: Value = serde_json::from_str(&buffer).unwrap();
//    let data: Value = serde_json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
    let adapter = match &parser[0] {
        &processor::Processor::Adapter(ref a) => a,
    };
    println!("{}", adapter(&data).unwrap().to_string());
}
