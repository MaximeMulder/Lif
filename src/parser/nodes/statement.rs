use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_3::expression_3;

pub fn statement<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Function(&expression_3),
		&Next::Element(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}
