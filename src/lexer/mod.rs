mod node;
mod nodes;
use nodes::ROOT;
use crate::elements::ignores::{ WHITESPACE, ENDLINE };
use crate::element::Element;
use crate::node::Node;

pub fn lex<'a>(string: &'a str) -> Vec<Node<'static, 'a>> {
	let mut tokens = Vec::new();
	let mut shift = 0;
	while let Some((element, length)) = automaton(&string[shift ..]) {
		if element != &WHITESPACE && element != &ENDLINE {
			tokens.push(Node::new_token(element, &string[shift .. shift + length]));
		}

		shift += length;
	}

	return tokens;
}

fn automaton(string: &str) -> Option<(&'static Element, usize)> {
	let mut node = &ROOT;
	let mut counter = 0;
	for character in string.chars() {
		let next = (node.execute)(character);
		if next.is_none() {
			break;
		}

		node = next.unwrap();
		counter += 1;
	}

	if node.element.is_none() {
		return None;
	}

	return Some((node.element.unwrap(), counter));
}
