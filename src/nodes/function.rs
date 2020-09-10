use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Function<'a> {
	node: &'a SyntaxNode<'a>,
	parameters: Vec<Declaration<'a>>,
	r#type: Option<Expression<'a>>,
	block: Block<'a>,
}

impl<'a> Function<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, parameters: Vec<Declaration<'a>>, r#type: Option<Expression<'a>>, block: Block<'a>) -> Self {
		return Self {
			node,
			parameters,
			r#type,
			block,
		};
	}
}

impl Node for Function<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let r#type = if let Some(r#type) = self.r#type.as_ref() {
			Some(r#type.execute(engine)?.read()?)
		} else {
			None
		};

		return Ok(engine.new_function(&self.parameters, r#type, &self.block));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
