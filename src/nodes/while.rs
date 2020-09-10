use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct While {
	condition: Expression,
	body:      Block,
}

impl While {
	pub fn new(condition: Expression, body: Block) -> Self {
		return Self {
			condition,
			body,
		};
	}
}

impl Node for While {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		while {
			let reference = execute!(engine, &self.condition);
			*reference.read()?.get_cast_boolean(engine)?
		} {
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
}
