use std::fs::File;
use json_parser::parser::JsonParser;

fn main() {
    let file = File::open("test.json").unwrap();
    let parser = JsonParser::parse(file).unwrap();

    dbg!(parser);
}