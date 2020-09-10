use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Loop<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	body: Block<'a, 'b>,
}

impl<'a, 'b> Loop<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, body: Block<'a, 'b>) -> Self {
		return Self {
			node,
			body,
		};
	}
}

impl Node for Loop<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		loop {
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
