mod control;
mod new;

use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Data, Tagger };
use crate::runtime::primitives::Primitives;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcTraceable };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::{ GcValue, Value };

use std::cmp::min;
use std::io::{ Read, Write };

#[derive(PartialEq, Eq)]
pub enum Control {
    Return,
    Break,
    Continue,
}

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

pub struct Engine<'a, 'b> where 'a: 'b {
    pub input:      &'b mut dyn Read,
    pub output:     &'b mut dyn Write,
    pub error:      &'b mut dyn Write,
    pub primitives: Primitives<'a, 'b>,
    taggers:        Taggers,
    scopes:         Gc<Scope<'a, 'b>>,
    references:     Gc<Reference<'a, 'b>>,
    values:         Gc<Value<'a, 'b>>,
    registries:     Vec<Vec<GcReference<'a, 'b>>>,
    frames:         Vec<GcScope<'a, 'b>>,
    scope:          GcScope<'a, 'b>,
    undefined:      GcReference<'a, 'b>,
    control:        Option<Control>,
    allocations:    usize,
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn new(input: &'b mut dyn Read, output: &'b mut dyn Write, error: &'b mut dyn Write) -> Self {
        let mut engine = Self {
            input,
            output,
            error,
            primitives: Primitives::new(),
            taggers:     Taggers::new(),
            scopes:      Gc::new(),
            references:  Gc::new(),
            values:      Gc::new(),
            registries:  Vec::new(),
            frames:      Vec::new(),
            scope:       GcScope::null(),
            undefined:   GcReference::null(),
            control:     None,
            allocations: 0,
        };

        engine.scope = engine.alloc_scope(Scope::new());
        engine.undefined = engine.alloc_reference(Reference::new_constant(None));
        engine.registries.push(Vec::new());
        engine.populate();
        engine
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn alloc_value(&mut self, value: Value<'a, 'b>) -> GcValue<'a, 'b> {
        let value = self.values.alloc(value);
        self.allocations += 1;
        value
    }

    pub fn alloc_reference(&mut self, reference: Reference<'a, 'b>) -> GcReference<'a, 'b> {
        let reference = self.references.alloc(reference);
        self.allocations += 1;
        reference
    }

    pub fn alloc_scope(&mut self, scope: Scope<'a, 'b>) -> GcScope<'a, 'b> {
        let scope = self.scopes.alloc(scope);
        self.allocations += 1;
        scope
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn new_value(&mut self, class: GcValue<'a, 'b>, data: Data<'a, 'b>) -> GcValue<'a, 'b> {
        self.alloc_value(Value::new(class, data))
    }

    pub fn new_reference(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_variable(Some(value), self.primitives.any))
    }

    pub fn new_variable(&mut self, value: Option<GcValue<'a, 'b>>, r#type: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_variable(value, r#type))
    }

    pub fn new_constant(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_constant(Some(value)))
    }

    pub fn undefined(&mut self) -> GcReference<'a, 'b> {
        self.undefined
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn push_scope(&mut self) {
        self.scope = self.alloc_scope(Scope::new_child(self.scope));
    }

    pub fn pop_scope(&mut self) {
        self.scope = self.scope.parent.unwrap();
    }

    pub fn push_frame(&mut self, frame: GcScope<'a, 'b>) {
        self.frames.push(self.scope);
        self.scope = frame;
    }

    pub fn pop_frame(&mut self) {
        self.scope = self.frames.pop().unwrap();
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn add_variable(&mut self, name: &str, reference: GcReference<'a, 'b>) {
        self.scope.add_variable(name, reference);
    }

    pub fn get_variable(&self, name: &str) -> ReturnReference<'a, 'b> {
        let mut scope = self.scope;
        loop {
            if let Some(object) = scope.get_variable(name) {
                return Ok(object);
            }

            if let Some(parent) = scope.parent {
                scope = parent;
            } else {
                return Err(Error::new_undeclared_variable(name));
            }
        }
    }

    pub fn collect(&mut self) {
        self.trace();
        self.scopes.collect();
        self.references.collect();
        self.values.collect();
        self.allocations = 0;
    }

    pub fn execute(&mut self, node: &'b Node<'a>) -> ReturnReference<'a, 'b> {
        self.registries.push(Vec::new());
        let reference = match node.sem.execute(self) {
            Ok(reference) => reference,
            Err(mut error) => {
                if error.node.is_none() {
                    error.node = Some(node.syn);
                }

                return Err(error);
            },
        };

        let index = self.registries.len() - 2;
        self.registries[index].push(reference);
        self.registries.pop();
        if self.allocations > GC_THRESHOLD {
            self.collect();
        }

        Ok(reference)
    }

    pub fn run(&mut self, node: &'b Node<'a>) {
        let result = self.execute(node);
        if let Err(error) = result {
            let mut message = String::new();
            message += &error.message;
            if let Some(node) = error.node {
                let code = node.code;
                if let Some(name) = &code.name {
                    message += name;
                }

                message += "\n";
                message += code.node_line(&node);
                message += "\n";
                message += &" ".repeat(code.node_shift_left(&node));
                message += &"^".repeat(min(code.node_str(&node).len(), code.node_shift_right(&node)));
                writeln!(self.error, "{}", message).unwrap();
            }
        }
    }
}

impl GcTraceable for Engine<'_, '_> {
    fn trace(&mut self) {
        self.primitives.trace();
        self.scope.trace();
        self.undefined.trace();
        for registries in self.registries.iter_mut() {
            for registry in registries.iter_mut() {
                registry.trace();
            }
        }

        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}