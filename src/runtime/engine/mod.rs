mod new;
mod primitive;
mod scope;

use crate::memory::{ Own, Ref };
use crate::parser::{ Code, Grammar };
use crate::runtime::data::Data;
use crate::runtime::primitives::Primitives;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcRef, GcTrace };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::registries::Registries;
use crate::runtime::r#return::{ ReturnFlow, ReturnReference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::utilities::tag::Tagger;
use crate::runtime::value::{ GcValue, Value };
use crate::walker::WNode;

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
    pub grammar:    &'a Grammar,
    pub input:      &'a mut dyn Read,
    pub output:     &'a mut dyn Write,
    pub error:      &'a mut dyn Write,
    pub primitives: Primitives<'a>,
    registries:     Registries,
    taggers:        Taggers,
    gc:             Gc,
    codes:          Vec<Own<Code>>,
    frames:         Vec<GcScope<'a>>,
    scope:          GcScope<'a>,
    undefined:      GcReference<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(grammar: &'a Grammar, input: &'a mut dyn Read, output: &'a mut dyn Write, error: &'a mut dyn Write) -> Self {
        let mut engine = Self {
            grammar,
            input,
            output,
            error,
            primitives:  Primitives::new(),
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
        self.scope.get_variable(name)
    }

    pub fn walk(&mut self, node: &WNode) -> ReturnFlow<'a> {
        self.registries.push();
        let r#return = node.walk(self);
        if let Ok(flow) = r#return.as_ref() {
            self.registries.cache(flow.reference);
        }

        self.registries.pop();
        if self.gc.get_allocations() > GC_THRESHOLD {
            self.trace();
            self.gc.collect();
        }

        r#return
    }

    pub fn run(&mut self, code: Own<Code>) -> Option<GcReference<'a>> {
        self.codes.push(code);
        let node = Ref::new(self.codes.last().unwrap().walk_tree.as_ref().unwrap());
        let executable = Ref::as_ref(&node);
        match self.walk(executable) {
            Ok(flow) => Some(flow.reference),
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
