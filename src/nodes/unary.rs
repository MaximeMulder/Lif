use super::expression::Expression;
use super::Node;

pub struct Unary<'a> {
	expression: Expression,
	operator: &'a str,
}

impl Node for Unary<'_> {
	fn execute(&self) {

	}
}
