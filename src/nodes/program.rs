use crate::nodes::statements::Statements;
use crate::nodes::{ Node, SyntaxNode };

pub struct Program {
	statements: Statements,
}

impl Program {
	pub fn build(node: &SyntaxNode) -> Program {
		return Program {
			statements: Statements::build(&node.children()[0]),
		};
	}
}

impl Node for Program {
	fn execute(&self) {
		self.statements.execute();
	}
}
