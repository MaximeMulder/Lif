use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Structure<'a> {
	node: &'a SyntaxNode<'a>,
	exe: Box<dyn Node<'a> + 'a>,
}

impl<'a> Structure<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, exe: Box<dyn Node<'a> + 'a>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl<'a> Node<'a> for Structure<'a> {
	fn execute(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(self.exe.as_ref());
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
