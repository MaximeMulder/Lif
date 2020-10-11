use crate::runtime::Return;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub type GcReference<'a, 'b> = GcRef<Reference<'a, 'b>>;

pub struct Reference<'a, 'b> {
    value: Option<GcValue<'a, 'b>>,
    r#type: Type<'a, 'b>,
}

enum Type<'a, 'b> {
    Variable(GcValue<'a, 'b>),
    Constant,
}

impl<'a, 'b> Reference<'a, 'b> {
    pub fn new_variable(value: Option<GcValue<'a, 'b>>, r#type: GcValue<'a, 'b>) -> Self {
        Self {
            value,
            r#type: Type::Variable(r#type),
        }
    }

    pub fn new_constant(value: Option<GcValue<'a, 'b>>) -> Self {
        Self {
            value,
            r#type: Type::Constant,
        }
    }

    pub fn read(&self) -> Return<'a, GcValue<'a, 'b>> {
        self.value.ok_or_else(|| Error::new_undefined())
    }

    pub fn write(&mut self, value: GcValue<'a, 'b>) -> Return<'a, ()> {
        match self.r#type {
            Type::Variable(r#type) => {
                value.cast(r#type)?;
                self.set_value(value);
            },
            Type::Constant => if self.value.is_none() {
                self.set_value(value);
            } else {
                return Err(Error::new_constant_write());
            },
        }

        Ok(())
    }

    pub fn is_defined(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_undefined(&self) -> bool {
        !self.is_defined()
    }

    pub fn get_value(&self) -> GcValue<'a, 'b> {
        self.value.unwrap()
    }

    pub fn set_value(&mut self, value: GcValue<'a, 'b>) {
        self.value = Some(value);
    }
}

impl GcTraceable for Reference<'_, '_> {
    fn trace(&mut self) {
        if let Some(value) = self.value.as_mut() {
            value.trace();
        }
    }
}
