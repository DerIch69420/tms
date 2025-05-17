mod parser;
mod interpreter;
mod ast;

use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Expected a script file.");
    let source = fs::read_to_string(&filename).expect("Failed to read script file.");

    let program = parser::parse(&source).expect("Parsing failed.");
    interpreter::run(program);
}

