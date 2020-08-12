use crate::runtime::{ Engine, Object, Reference };
use super::{ Node, SyntaxNode };
use crate::elements;

use super::token::Token;

enum Content {
	Identifier,
	String,
	Number,
}

pub struct Literal {
	content: Content,
	text: Box<str>
}

impl Literal {
	pub fn build(node: &SyntaxNode) -> Literal {
		let child = &node.children()[0];
		return Literal {
			content: match child.element {
				&elements::variables::IDENTIFIER => Content::Identifier,
				&elements::variables::STRING     => Content::String,
				&elements::variables::NUMBER     => Content::Number,
				_ => panic!(),
			},
			text: Token::build(child),
		};
	}
}

impl Node for Literal {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		return match &self.content {
			Content::Identifier => engine.get_variable(&self.text),
			Content::String     => engine.new_object(Object::new_string(engine, &self.text)),
			Content::Number     => engine.new_object(Object::new_integer(engine, self.text.parse::<usize>().unwrap())),
		};
	}
}
