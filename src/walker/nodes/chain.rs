use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

pub struct AChain {
    expression: ANode<AExpression>,
    member:     Ref<str>,
}

impl AChain {
    pub fn new(expression: ANode<AExpression>, member: Ref<str>) -> Self {
        Self {
            expression,
            member,
        }
    }
}

impl AExpressionTrait for AChain {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = get!(self.expression.get().walk(engine)?).read()?;
        let name = engine.new_string(self.member.to_string());
        Flow::new(value.call_method(engine, "__cn__", &mut [name.read()?])?)
    }
}
