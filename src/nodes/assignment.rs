use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Assignment {
    reference:  Node,
    expression: Node,
    operator:   Option<Ref<str>>,
}

impl Assignment {
    pub fn new(reference: Node, expression: Node, operator: Ref<str>) -> Self {
        Self {
            reference,
            expression,
            operator: if operator.len() > 1 {
                Some(Ref::from_ref(&operator[.. operator.len() - 1]))
            } else {
                None
            },
        }
    }
}

impl Executable for Assignment {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut reference  = execute!(engine, Ref::from_ref(&self.reference));
        let mut expression = execute!(engine, Ref::from_ref(&self.expression)).read()?;
        if let Some(operator) = &self.operator {
            let left = reference.read()?;
            expression = left.get_method(operator).unwrap().call(engine, vec![left, expression])?.read()?;
        }

        reference.write(expression)?;
        Ok(engine.undefined())
    }
}
