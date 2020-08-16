use super::expression::Expression;
use super::SyntaxNode;

pub fn expressions(node: &SyntaxNode) -> Vec<Expression> {
	let mut expressions = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		expressions.push(Expression::build(child));
	}

	return expressions;
}
