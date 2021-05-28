use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::ReturnReference;

pub struct Continue {
    expression: Option<Node>,
}

impl Continue {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Continue {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.jump_new(Jump::Continue, self.expression.as_ref())
    }
}
