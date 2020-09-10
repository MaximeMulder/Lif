use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct ForIn<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	identifier: Box<str>,
	expression: Expression<'a, 'b>,
	body:       Block<'a, 'b>,
}

impl<'a, 'b> ForIn<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, identifier: Box<str>, expression: Expression<'a, 'b>, body: Block<'a, 'b>) -> Self {
		return Self {
			node,
			identifier,
			expression,
			body,
		};
	}
}

impl Node for ForIn<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		for element in {
			let reference = execute!(engine, &self.expression);
			reference.read()?.get_cast_array(engine)?.clone()
		} {
			engine.add_variable(&self.identifier, element);
			let reference = engine.execute(&self.body)?;
			match &engine.control {
				Some(control) => match control {
					Control::Return => return Ok(reference),
					Control::Continue => {
						engine.control = None;
						array.push(reference);
						continue;
					},
					Control::Break => {
						engine.control = None;
						array.push(reference);
						break
					},
				},
				None => array.push(reference),
			}
		}

		return Ok(engine.new_array(array));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
