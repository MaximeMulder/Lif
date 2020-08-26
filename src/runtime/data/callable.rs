use crate::nodes::{ block::Block, Control };
use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::proxy::Visitable;
use crate::runtime::reference::Reference;
use crate::runtime::scope::Scope;

pub trait Callable<'a>: Visitable {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a>;
	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a>;
}

#[derive(Clone)]
pub struct Primitive<'a> {
	callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
		return (self.callback)(engine, arguments);
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl Visitable for Primitive<'_> {
	fn visit(&mut self) {}
}

#[derive(Clone)]
pub struct Function<'a> {
	scope: Scope<'a>,
	parameters: &'a Vec<Box<str>>,
	block: &'a Block,
}

impl<'a> Function<'a> {
	pub fn new(scope: Scope<'a>, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			block,
		};
	}
}

impl<'a> Callable<'a> for Function<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
		let frame = engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let reference = argument.clone();
			engine.new_variable(&parameter, reference);
		}

		let product = self.block.execute(engine);
		let reference = match &product.control {
			Some(control) => match control {
				Control::Break | Control::Continue => panic!(),
				Control::Return => product.reference,
			},
			None => engine.new_undefined(),
		};

		engine.pop_frame(frame);
		return reference;
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl Visitable for Function<'_> {
	fn visit(&mut self) {
		self.scope.visit();
	}
}
