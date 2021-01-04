use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::data::Data;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

use std::ops::Deref;

impl<'a> Engine<'a> {
    pub fn new_array_value(&mut self, elements: Vec<GcReference<'a>>) -> GcValue<'a> {
        self.new_value(self.primitives.array, Data::new_array(elements))
    }

    pub fn new_boolean_value(&mut self, boolean: bool) -> GcValue<'a> {
        self.new_value(self.primitives.boolean, Data::new_boolean(boolean))
    }

    pub fn new_class_value(&mut self, name: Option<Ref<str>>, parent: GcValue<'a>) -> GcValue<'a> {
        let tag = self.taggers.classes.generate(name.map(|name| Box::from(name.deref())));
        self.new_value(self.primitives.class, Data::new_class(tag, Some(parent)))
    }

    pub fn new_class_primitive_value(&mut self, name: Ref<str>) -> GcValue<'a> {
        let tag = self.taggers.classes.generate(Some(Box::from(name.deref())));
        self.new_value(self.primitives.class, Data::new_class(tag, Some(self.primitives.any)))
    }

    pub fn new_function_value(&mut self, name: Option<Ref<str>>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> GcValue<'a> {
        let tag = self.taggers.functions.generate(name.map(|name| Box::from(name.deref())));
        self.new_value(self.primitives.function, Data::new_function(tag, self.scope, parameters, r#type, block))
    }

    pub fn new_generic_value(&mut self, name: Option<Ref<str>>, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> GcValue<'a> {
        let tag = self.taggers.generics.generate(name.map(|name| Box::from(name.deref())));
        self.new_value(self.primitives.generic, Data::new_generic(tag, generics, node))
    }

    pub fn new_integer_value(&mut self, integer: isize) -> GcValue<'a> {
        self.new_value(self.primitives.integer, Data::new_integer(integer))
    }

    pub fn new_method_value(&mut self, function: GcValue<'a>, this: GcValue<'a>) -> GcValue<'a> {
        self.new_value(self.primitives.method, Data::new_method(function, this))
    }

    pub fn new_object_value(&mut self, parent: GcValue<'a>) -> GcValue<'a> {
        self.new_value(parent, Data::new_object())
    }

    pub fn new_primitive_value(&mut self, name: Ref<str>, parameters: Box<[GcValue<'a>]>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> GcValue<'a> {
        let tag = self.taggers.functions.generate(Some(Box::from(name.deref())));
        self.new_value(self.primitives.function, Data::new_primitive(tag, parameters, callback))
    }

    pub fn new_string_value(&mut self, string: String) -> GcValue<'a> {
        self.new_value(self.primitives.string, Data::new_string(string))
    }
}

impl<'a> Engine<'a> {
    pub fn new_array(&mut self, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        let value = self.new_array_value(elements);
        self.new_constant(value)
    }

    pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a> {
        let value = self.new_boolean_value(boolean);
        self.new_constant(value)
    }

    pub fn new_class(&mut self, name: Option<Ref<str>>, parent: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_class_value(name, parent);
        self.new_constant(value)
    }

    pub fn new_function(&mut self, name: Option<Ref<str>>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> GcReference<'a> {
       let value = self.new_function_value(name, parameters, r#type, block);
        self.new_constant(value)
    }

    pub fn new_generic(&mut self, name: Option<Ref<str>>, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> GcReference<'a> {
        let value = self.new_generic_value(name, generics, node);
        self.new_constant(value)
    }

    pub fn new_integer(&mut self, integer: isize) -> GcReference<'a> {
        let value = self.new_integer_value(integer);
        self.new_constant(value)
    }

    pub fn new_method(&mut self, function: GcValue<'a>, this: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_method_value(function, this);
        self.new_constant(value)
    }

    pub fn new_object(&mut self, parent: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_object_value(parent);
        self.new_constant(value)
    }

    pub fn new_primitive(&mut self, name: Ref<str>, parameters: Box<[GcValue<'a>]>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> GcReference<'a> {
        let value = self.new_primitive_value(name, parameters, callback);
        self.new_constant(value)
    }

    pub fn new_string(&mut self, string: String) -> GcReference<'a> {
        let value = self.new_string_value(string);
        self.new_constant(value)
    }
}
