use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

use std::ops::Deref;

pub struct APreop {
    expression: ANode<AExpression>,
    operator:   Ref<str>,
}

impl APreop {
    pub fn new(operator: Ref<str>, expression: ANode<AExpression>) -> Self {
        Self {
            expression,
            operator: Ref::new(match operator.deref() {
                "~" => "__bnot__",
                "+" => "__pos__",
                "-" => "__neg__",
                "!" => "__not__",
                _   => panic!(),
            }),
        }
    }
}

impl WExpression for APreop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let expression = flow!(self.expression.get().walk(engine)?).read()?;
        Flow::reference(expression.call_method(engine, &self.operator, &mut [])?)
    }
}