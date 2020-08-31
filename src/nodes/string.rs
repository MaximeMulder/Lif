use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct String {
	string: Box<str>,
}

impl String {
	pub fn new(string: Box<str>) -> Self {
		return Self {
			string,
		};
	}
}

impl Node for String {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return engine.new_string(self.string.to_string());
	}
}
