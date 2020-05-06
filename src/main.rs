#![allow(dead_code)]
#![feature(const_fn)]

mod element;
mod elements;
mod lexer;
mod parser;
mod printer;
mod token;
mod tree;

use std::env;
use std::fs;

fn main() {
    println!("Leaf compiler.");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect arguments length.");
        panic!();
    }

    let text = fs::read_to_string(&args[1]).expect("");
    let tokens = lexer::lex(&text);
    printer::tokens(&tokens);

    println!("=====");

    if let Some(tree) = parser::run(&tokens) {
        printer::tree(&tree);
    }
}