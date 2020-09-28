use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statement<'a> {
	node: &'a SyntaxNode<'a>,
	exe: Box<dyn Node<'a> + 'a>,
}

impl<'a> Statement<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, exe: Box<dyn Node<'a> + 'a>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl<'a> Node<'a> for Statement<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		execute!(engine, self.exe.as_ref());
		return Ok(engine.undefined());
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
