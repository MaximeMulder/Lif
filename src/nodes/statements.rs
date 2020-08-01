use super::statement::Statement;
use super::{ Node, SyntaxNode };

pub struct Statements {
	statements: Vec<Statement>,
}

impl Statements {
	pub fn build(node: &SyntaxNode) -> Statements {
		let mut statements = Vec::new();
		for child in node.children() {
			statements.push(Statement::build(child));
		}

		return Statements {
			statements,
		};
	}
}

impl Node for Statements {
	fn execute(&self) {
		for statement in self.statements.iter() {
			statement.execute();
		}
	}
}
