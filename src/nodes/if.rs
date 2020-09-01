use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct If {
	condition: Expression,
	then:      Expression,
	r#else:    Option<Box<dyn Node>>,
}

impl If {
	pub fn new(condition: Expression, then: Expression, r#else: Option<Box<dyn Node>>) -> Self {
		return Self {
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return if {
			let reference = execute!(engine, &self.condition);
			*reference.read().get_cast_boolean(engine)
		} {
			execute!(engine, &self.then)
		} else if let Some(r#else) = &self.r#else {
			execute!(engine, r#else.as_ref())
		} else {
			return engine.new_undefined();
		}
	}
}
