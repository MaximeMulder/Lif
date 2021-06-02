#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(bool_to_option)]
#![feature(drain_filter)]
#![feature(const_generics)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_extra)]
#![feature(new_uninit)]
#![feature(raw)]
#![feature(unsize)]
#![warn(clippy::all)]

mod element;
mod elements;
mod lexer;
mod memory;
mod node;
mod nodes;
mod parser;
mod printer;
mod runtime;
mod code;

#[cfg(test)]
mod tests;

use code::Code;
use parser::Parser;
use nodes::build;
use runtime::engine::Engine;
use std::env::args;
use std::io::{ stderr, stdin, stdout };

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect arguments length.");
        return;
    }

    let parser = Parser::new();
    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    let a = std::time::Instant::now();
    let mut engine = Engine::new(&parser, &mut input, &mut output, &mut error);
    let b = std::time::Instant::now();
    let code = Code::from_file(engine.parser, 0, &build::program, &args[1]).unwrap();
    let c = std::time::Instant::now();
    engine.run(code);
    let d = std::time::Instant::now();
    print!("INIT TIME: {}ns\nPARSING TIME: {}ns\nRUN TIME: {}ns\nTOTAL TIME: {}ns\n", (b - a).as_nanos(), (c - b).as_nanos(), (d - c).as_nanos(),(d - a).as_nanos());
}
