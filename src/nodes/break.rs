use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Break {
    expression: Option<Node>,
}

impl Break {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Break {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.control_new(Control::Break, self.expression.as_ref())
    }
}
