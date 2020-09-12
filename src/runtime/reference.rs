use crate::runtime::Return;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
	value: Option<GcValue<'a>>,
	r#type: Type<'a>,
}

enum Type<'a> {
	Variable(GcValue<'a>),
	Constant,
}

impl<'a> Reference<'a> {
	pub fn new_variable(value: Option<GcValue<'a>>, r#type: GcValue<'a>) -> Self {
		return Self {
			value,
			r#type: Type::Variable(r#type),
		};
	}

	pub fn new_constant(value: Option<GcValue<'a>>) -> Self {
		return Self {
			value,
			r#type: Type::Constant,
		};
	}

	pub fn read(&self) -> Return<GcValue<'a>> {
		return self.value.ok_or_else(|| Error::new_runtime("Trying to read an undefined value."));
	}

	pub fn write(&mut self, value: GcValue<'a>) -> Return<()> {
		match self.r#type {
			Type::Variable(r#type) => if value.isa(r#type) {
				self.set_value(value);
			} else {
				return Err(Error::new_runtime("Trying to write a value in a variable whose type does not match."));
			},
			Type::Constant => if self.value.is_none() {
				self.set_value(value);
			} else {
				return Err(Error::new_runtime("Trying to write a value in a constant."));
			},
		}

		return Ok(());
	}

	pub fn is_defined(&self) -> bool {
		return self.value.is_some();
	}

	pub fn is_undefined(&self) -> bool {
		return !self.is_defined();
	}

	pub fn get_value(&self) -> GcValue<'a> {
		return self.value.unwrap();
	}

	pub fn set_value(&mut self, value: GcValue<'a>) {
		self.value = Some(value);
	}
}

impl GcTraceable for Reference<'_> {
	fn trace(&mut self) {
		if let Some(value) = self.value.as_mut() {
			value.trace();
		}
	}
}
