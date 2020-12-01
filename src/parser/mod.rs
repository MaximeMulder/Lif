#![allow(unused_variables)]
#![allow(dead_code)]

mod descent;
mod ascent;
mod nodes;
mod arena;

use crate::code::Code;
use crate::lexer::lex;
use crate::node::Node;
use arena::Arena;
use ascent::Ascent;
use descent::Descent;
use std::cmp::min;

pub struct Parser {
    descents: Arena<dyn Descent>,
    ascents: Arena<dyn Ascent>,
    program: usize,
}

impl Parser {
    pub fn new() -> Self {
        let grammar = nodes::get();
        Self {
            descents: grammar.0,
            ascents:  grammar.1,
            program:  grammar.2,
        }
    }

    pub fn parse<'a>(&self, code: &'a Code) -> Option<Node<'a>> {
        let tokens = lex(code);
        let mut parse = Parse::new(self, code, &tokens);
        parse.parse(self.program)
    }
}

pub struct Parse<'a, 'b> {
    parser: &'a Parser,
    code: &'b Code,
    tokens: &'a [Node<'b>],
    cursor: usize,
    reach: usize,
}

impl<'a, 'b> Parse<'a, 'b> {
    fn new(parser: &'a Parser, code: &'b Code, tokens: &'a [Node<'b>]) -> Self {
        Self {
            parser,
            code,
            tokens,
            cursor: 0,
            reach: 0,
        }
    }

    fn done(&self) -> bool {
        self.cursor == self.tokens.len()
    }

    fn next(&mut self) -> Option<Node<'b>> {
        let option = self.tokens.get(self.cursor);
        if let Some(token) = option {
            if self.reach < self.cursor {
                self.reach = self.cursor;
            }

            self.cursor += 1;
            return Some(token.clone());
        }

        None
    }

    fn run(&mut self, callback: impl FnOnce(&mut Self) -> Option<Vec<Node<'b>>>) -> Option<Vec<Node<'b>>>{
        let cursor = self.cursor;
        let nodes = callback(self);
        if nodes.is_none() {
            self.cursor = cursor;
        }

        nodes
    }

    fn run_predicate(&mut self, callback: impl FnOnce(&mut Self) -> Option<Vec<Node<'b>>>) -> bool {
        let cursor = self.cursor;
        let nodes = callback(self);
        self.cursor = cursor;
        nodes.is_some()
    }

    fn descent(&mut self, index: usize) -> Option<Vec<Node<'b>>> {
        self.run(|parse| parse.parser.descents.get(index).descent(parse))
    }

    fn descent_predicate(&mut self, index: usize) -> bool {
        self.run_predicate(|parse| parse.parser.descents.get(index).descent(parse))
    }

    fn ascent(&mut self, index: usize, nodes: Vec<Node<'b>>) -> Option<Vec<Node<'b>>> {
        self.run(|parse| parse.parser.ascents.get(index).ascent(parse, nodes))
    }

    fn ascent_predicate(&mut self, index: usize, nodes: Vec<Node<'b>>) -> bool {
        self.run_predicate(|parse| parse.parser.ascents.get(index).ascent(parse, nodes))
    }

    pub fn parse(&mut self, program: usize) -> Option<Node<'b>> {
        let begin = std::time::Instant::now();
        let node = if let Some(mut nodes) = self.parser.descents.get(program).descent(self) {
            nodes.pop()
        } else {
            return None;
        };

        if self.done() {
            node
        } else {
            let token = &self.tokens[self.reach];
            println!("PARSING ERROR, LINE {}, POSITION {}, UNEXPECTED TOKEN: {:?} - {}\n\n{}\n{}{}",
                self.code.node_y(token),
                self.code.node_x(token),
                self.code.node_str(token),
                token.element.name,
                self.code.node_line(token),
                " ".repeat(self.code.node_shift_left(token)),
                "^".repeat(min(self.code.node_str(token).len(), self.code.node_shift_right(token)))
            );

            None
        }
    }
}
