use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Return {
    expression: Option<Node>}

impl Return {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Return {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.control_new(Control::Return, self.expression.as_ref())
    }
}
