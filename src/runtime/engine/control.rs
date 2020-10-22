
use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

impl<'a, 'b> Engine<'a, 'b> {
    pub fn control_new(&mut self, control: Control, node: &'b Option<Node<'a>>) -> ReturnReference<'a, 'b> {
        let reference = if let Some(node) = node {
            let value = self.execute(node)?.read()?;
            self.new_constant(value)
        } else {
            self.undefined()
        };

        if self.control.is_none() {
            self.control = Some(control);
        }

        Ok(reference)
    }

    pub fn control_none(&mut self) -> bool {
        self.control.is_none()
    }

    pub fn control_is(&mut self, other: Control) -> bool {
        if let Some(control) = &self.control {
            if *control == other {
                return true;
            }
        }

        false
    }

    pub fn control_consume(&mut self, control: Control) -> bool {
        if self.control_is(control) {
            self.control = None;
            return true;
        }

        false
    }
}
