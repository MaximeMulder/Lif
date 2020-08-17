use super::expression::Expression;
use crate::runtime::Engine;
use super::{ Node, Product, Control };

pub struct DoWhile {
	body:      Expression,
	condition: Expression,
}

impl DoWhile {
	pub fn new(body: Expression, condition: Expression) -> Self {
		return Self {
			body,
			condition,
		};
	}
}

impl Node for DoWhile {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let mut array = Vec::new();
		loop {
			let product = self.body.execute(engine);
			match &product.control {
				Some(control) => match control {
					Control::Return => return product,
					Control::Continue => {
						array.push(product.reference);
						continue;
					},
					Control::Break => {
						array.push(product.reference);
						break
					},
				},
				None => array.push(product.reference),
			}

			if {
				let reference = value!(self.condition.execute(engine));
				!engine.get_cast_boolean(engine.read(reference))
			} {
				break;
			}
		}

		return Product::new(engine.new_array(array));
	}
}
