use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub type GcScope<'a> = GcRef<Scope<'a>>;

pub struct Scope<'a> {
	pub parent: Option<GcScope<'a>>,
	variables: HashMap<String, GcReference<'a>>,
}

impl<'a> Scope<'a> {
	pub fn new() -> Self {
		return Self {
			parent: None,
			variables: HashMap::new(),
		};
	}

	pub fn new_child(scope: GcScope<'a>) -> Self {
		return Self {
			parent: Some(scope),
			variables: HashMap::new(),
		};
	}

	pub fn get_variable(&self, name: &str) -> Option<GcReference<'a>> {
		if let Some(reference) = self.variables.get(name) {
			return Some(*reference);
		}

		return None;
	}

	pub fn add_variable(&mut self, name: &str, reference: GcReference<'a>) {
		self.variables.insert(name.to_string(), reference);
	}
}

impl GcTraceable for Scope<'_> {
	fn trace(&mut self) {
		if let Some(parent) = &mut self.parent {
			parent.trace();
		}

		for variable in self.variables.values_mut() {
			variable.trace();
		}
	}
}