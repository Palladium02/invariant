#![feature(new_range_api)]

use std::{fs, range::Range};

mod ast;
mod lexer;
mod parser;
mod token;
mod traits;

use lexer::Lexer;
use token::Token;

fn main() {
    let input = fs::read_to_string("./examples/example1.inv").unwrap();
    let tokens = Lexer::new(&input).collect::<Vec<(Token, Range<usize>)>>();

    println!("{:?}", tokens)
}
