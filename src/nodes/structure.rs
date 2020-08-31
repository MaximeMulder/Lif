use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Structure {
	node: Box<dyn Node>,
}

impl Structure {
	pub fn new(node: Box<dyn Node>) -> Self {
		return Self {
			node,
		};
	}
}

impl Node for Structure {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return execute!(engine, self.node.as_ref());
	}
}
