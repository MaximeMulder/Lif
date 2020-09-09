use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		return Ok(engine.new_string(self.string.to_string()));
	}
}
