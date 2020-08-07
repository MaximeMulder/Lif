use super::identifier::Identifier;
use super::{ Engine, Node, SyntaxNode };

pub struct Declaration {
	identifier: Identifier,
}

impl Declaration {
	pub fn build(node: &SyntaxNode) -> Declaration {
		return Declaration {
			identifier: Identifier::build(&node.children()[1]),
		};
	}
}

impl Node for Declaration {
	fn execute(&self, engine: &mut Engine) {
		let value = engine.new_variable(self.identifier.text.to_string());
		engine.set_value(value);
	}
}