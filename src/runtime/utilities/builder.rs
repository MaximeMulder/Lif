use crate::runtime::data::FunctionPrimitive;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::Callable;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

fn parameters<'a>(types: &[GcValue<'a>]) -> Box<[Variable<'a>]> {
    let mut parameters = Vec::new();
    for r#type in types.iter().copied() {
        parameters.push(Variable::new_unchecked(Box::from("__unused__"), Some(r#type)));
    }

    parameters.into_boxed_slice()
}

pub fn function<'a, const N: usize>(engine: &mut Engine<'a>, name: &str, types: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function(Some(name), parameters(&types), None, None, FunctionPrimitive::new(callback));
    engine.set_variable(name, primitive);
}

pub fn r#static<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, types: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.run_frame(value.data_class().scope(), |engine| {
        engine.new_function_value(Some(&name), parameters(&types), None, None, FunctionPrimitive::new(callback))
    });

    value.data_class_mut().set_static(name, engine.new_constant(primitive));
}

pub fn method<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, types: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.run_frame(value.data_class().scope(), |engine| {
        engine.new_function_value(Some(&name), parameters(&types), None, None, FunctionPrimitive::new(callback))
    });

    value.data_class_mut().set_method(name, primitive);
}

pub fn method_rest<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, types: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.run_frame(value.data_class().scope(), |engine| {
        engine.new_function_value(Some(&name), parameters(&types), Some(Variable::new_unchecked(Box::from("__unused__"), None)), None, FunctionPrimitive::new(callback))
    });

    value.data_class_mut().set_method(name, primitive);
}
