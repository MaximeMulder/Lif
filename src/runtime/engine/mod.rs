mod new;
mod scope;

use crate::code::Code;
use crate::memory::{ Own, Ref };
use crate::nodes::Executable;
use crate::parser::Parser;
use crate::runtime::data::{ Data, Tagger };
use crate::runtime::primitives::Primitives;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcRef, GcTrace };
use crate::runtime::jump::Jump;
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::registries::Registries;
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::constructors::{ GcConstructor, Constructor };
use crate::runtime::value::{ GcValue, Value };

use std::io::{ Read, Write };

pub struct Taggers {
    generics:  Tagger,
    classes:   Tagger,
    functions: Tagger,
}

impl Taggers {
    pub fn new() -> Self {
        Self {
            generics:  Tagger::new(),
            classes:   Tagger::new(),
            functions: Tagger::new(),
        }
    }
}

pub struct Engine<'a> {
    pub parser:     &'a Parser,
    pub input:      &'a mut dyn Read,
    pub output:     &'a mut dyn Write,
    pub error:      &'a mut dyn Write,
    pub primitives: Primitives<'a>,
    pub jump:       Jump,
    registries:     Registries,
    taggers:        Taggers,
    gc:             Gc,
    codes:          Vec<Own<Code>>,
    frames:         Vec<GcScope<'a>>,
    scope:          GcScope<'a>,
    undefined:      GcReference<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(parser: &'a Parser, input: &'a mut dyn Read, output: &'a mut dyn Write, error: &'a mut dyn Write) -> Self {
        let mut engine = Self {
            parser,
            input,
            output,
            error,
            primitives:  Primitives::new(),
            jump:        Jump::None,
            registries:  Registries::new(),
            taggers:     Taggers::new(),
            gc:          Gc::new(),
            codes:       Vec::new(),
            frames:      Vec::new(),
            scope:       GcScope::null(),
            undefined:   GcReference::null(),
        };

        engine.scope = engine.alloc(Scope::new(None));
        engine.undefined = engine.alloc(Reference::new_constant(None));
        engine.populate();
        engine
    }
}

impl Engine<'_> {
    pub fn alloc<T: GcTrace>(&mut self, object: T) -> GcRef<T> {
        let r#ref = self.gc.alloc(object);
        self.registries.store(r#ref);
        r#ref
    }
}

impl<'a> Engine<'a> {
    pub fn new_builder(&mut self, generic: GcValue<'a>, arguments: Box<[GcValue<'a>]>, value: GcValue<'a>) -> GcConstructor<'a> {
        self.alloc(Constructor::new(generic, arguments, value))
    }

    pub fn new_value(&mut self, class: GcValue<'a>, data: Data<'a>) -> GcValue<'a> {
        self.alloc(Value::new(class, data))
    }

    pub fn new_reference(&mut self, value: GcValue<'a>) -> GcReference<'a> {
        self.alloc(Reference::new_variable(Some(value), self.primitives.any))
    }

    pub fn new_variable(&mut self, value: Option<GcValue<'a>>, r#type: GcValue<'a>) -> GcReference<'a> {
        self.alloc(Reference::new_variable(value, r#type))
    }

    pub fn new_constant(&mut self, value: GcValue<'a>) -> GcReference<'a> {
        self.alloc(Reference::new_constant(Some(value)))
    }

    pub fn undefined(&mut self) -> GcReference<'a> {
        self.undefined
    }
}

impl<'a> Engine<'a> {
    pub fn set_constant_value(&mut self, name: &str, value: GcValue<'a>) {
        let reference = self.new_constant(value);
        self.set_variable(name, reference);
    }

    pub fn set_variable(&mut self, name: &str, reference: GcReference<'a>) {
        self.scope.set_variable(name, reference);
    }

    pub fn get_variable(&self, name: &str) -> ReturnReference<'a> {
        let mut scope = self.scope;
        loop {
            if let Some(object) = scope.get_variable(name) {
                return Ok(object);
            }

            if let Some(parent) = scope.parent() {
                scope = parent;
            } else {
                return Err(Error::new_undeclared_variable(name));
            }
        }
    }

    pub fn execute(&mut self, node: &dyn Executable) -> ReturnReference<'a> {
        self.registries.push();
        let reference = match node.execute(self) {
            Ok(reference) => reference,
            Err(error) => return Err(error),
        };

        self.registries.cache(reference);
        self.registries.pop();
        if self.gc.get_allocations() > GC_THRESHOLD {
            self.trace();
            self.gc.collect();
        }

        Ok(reference)
    }

    pub fn run(&mut self, code: Own<Code>) -> Option<GcReference<'a>> {
        self.codes.push(code);
        let node = Ref::new(self.codes.last().unwrap().cst.as_ref().unwrap());
        let executable = Ref::as_ref(&node);
        match self.execute(executable) {
            Ok(reference) => Some(reference),
            Err(error) => {
                writeln!(self.error, "{}", error.get_message()).unwrap();
                None
            },
        }
    }
}

impl GcTrace for Engine<'_> {
    fn trace(&mut self) {
        self.primitives.trace();
        self.registries.trace();
        self.scope.trace();
        self.undefined.trace();
        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}
