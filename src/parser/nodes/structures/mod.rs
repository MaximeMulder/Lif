pub mod block;
pub mod conditionnals;
pub mod loops;

use crate::node::Node;
use crate::parser::Parser;

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>)  -> Result<Node<'a, 'b>, ()> {
	let functions: [&dyn Fn(&mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()>; 6] = [
		&block::block,
		&conditionnals::r#if,
		&loops::r#loop,
		&loops::r#while,
		&loops::do_while,
		&loops::for_in,
	];

	for function in functions.iter() {
		if let Ok(node) = parser.safe(&|parser| function(parser)) {
			return Ok(node);
		}
	}

	return Err(());
}