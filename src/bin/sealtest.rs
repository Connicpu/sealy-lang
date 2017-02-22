extern crate sealy_lang;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("scripts/parsetest.seal").unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf).unwrap();

    let input = &buf[..];
    let lexer = sealy_lang::lexer::Lexer::new(input);
    let result = sealy_lang::parser::parse_seal(lexer);
    println!("{:?}", result);
}
