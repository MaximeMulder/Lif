mod boolean;
mod integer;
mod float;
mod string;
mod identifier;

pub use boolean::ABoolean;
pub use integer::AInteger;
pub use float::AFloat;
pub use string::AString;
pub use identifier::AIdentifier;

use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow, ReturnReference };
use crate::walker::ANode;
use crate::walker::nodes::AExpressionTrait;

pub trait ALiteralTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub struct ALiteral {
    literal: Box<ANode<dyn ALiteralTrait>>,
}

impl ALiteral {
	pub fn new(literal: Box<ANode<dyn ALiteralTrait>>) -> Self {
		Self {
			literal,
		}
	}
}

impl AExpressionTrait for ALiteral {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::reference(self.literal.get().walk(engine)?)
    }
}