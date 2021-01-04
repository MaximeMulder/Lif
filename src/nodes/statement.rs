use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statement {
    node: Node,
}

impl Statement {
    pub fn new(node: Node) -> Self {
        Self {
            node,
        }
    }
}

impl Executable for Statement {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        execute!(engine, Ref::from_ref(&self.node));
        Ok(engine.undefined())
    }
}
