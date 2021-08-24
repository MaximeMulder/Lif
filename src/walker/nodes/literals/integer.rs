use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::traits::WLiteral;

pub struct AInteger {
    integer: isize,
}

impl AInteger {
    pub fn new(integer: Ref<str>) -> Self {
        let string = integer.replace("_", "");
        Self {
            integer: match string.chars().nth(1) {
                Some('b') => isize::from_str_radix(&string[2..], 2).unwrap(),
                Some('o') => isize::from_str_radix(&string[2..], 8).unwrap(),
                Some('x') => isize::from_str_radix(&string[2..], 16).unwrap(),
                _ => string.parse::<isize>().unwrap(),
            }
        }
    }

}

impl WLiteral for AInteger {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_integer(self.integer))
    }
}
