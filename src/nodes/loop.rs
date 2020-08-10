use super::expression::Expression;
use super::{ Engine, Node, SyntaxNode };

pub struct Loop {
	body: Expression,
}

impl Loop {
	pub fn build(node: &SyntaxNode) -> Loop {
		return Loop {
			body: Expression::build(&node.children()[1]),
		};
	}
}

impl Node for Loop {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Option<usize> {
		loop {
			self.body.execute(engine);
		}
	}
}
