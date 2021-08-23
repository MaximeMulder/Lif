use crate::runtime::engine::Engine;
use crate::runtime::environment::Environment;
use crate::runtime::primitives::Function;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    let Environment { array_any, function, .. } = engine.environment;
    engine.populate_class("Function", function);
    engine.primitive_method(function, "__cl__", [("arguments", array_any)], None, None, &call);
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    arguments[0].get_gc::<Function>(engine).call(engine, &mut parameters::unpack(engine, arguments[1])?)
}