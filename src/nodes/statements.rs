use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Statements {
    statements: Vec<Node>,
}

impl Statements {
    pub fn new(statements: Vec<Node>) -> Self {
        Self {
            statements,
        }
    }
}

impl Executable for Statements {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        for statement in self.statements.iter() {
            execute!(engine, statement);
        }

        Ok(engine.undefined())
    }
}
