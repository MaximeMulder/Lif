use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };
use crate::runtime::utilities::variable::Variable;

pub struct FunctionPrimitive<'a> {
    callback: &'a Callable<'a>,
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new(callback: &'a Callable<'a>) -> Self {
        Self {
            callback,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionPrimitive<'a> {
    fn call(&self, engine: &mut Engine<'a>, _: &[Variable<'a>], _: &Option<Variable<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        (self.callback)(engine, arguments)
    }
}

impl GcTrace for FunctionPrimitive<'_> {
    fn trace(&mut self) {}
}
