use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;
use super::{ Node, Product };

pub struct Declaration {
	identifier: Box<str>,
}

impl Declaration {
	pub fn new(identifier: Box<str>) -> Self {
		return Self {
			identifier,
		};
	}
}

impl Node for Declaration {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let reference = Reference::new_undefined();
		engine.new_variable(&self.identifier, reference);
		return Product::new(reference);
	}
}
