use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Element(&elements::STRING)]) {
		return Some(Node::new_production(&elements::PRODUCTION_LITERAL, children));
	}

	if let Some(children) = parser.commit(vec![&Next::Element(&elements::NUMBER)]) {
		return Some(Node::new_production(&elements::PRODUCTION_LITERAL, children));
	}

	if let Some(children) = parser.commit(vec![&Next::Element(&elements::IDENTIFIER)]) {
		return Some(Node::new_production(&elements::PRODUCTION_LITERAL, children));
	}

	return None;
}
