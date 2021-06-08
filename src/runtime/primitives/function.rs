use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;
use crate::runtime::utilities::parameters;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, function, .. } = engine.primitives;
    engine.set_constant_value("Function", function);
    builder::method(engine, function, "to_string", [function],            &to_string);
    builder::method(engine, function, "__cl__",    [function, array_any], &cl);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn cl<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    arguments[0].data_function().call(engine, parameters::unpack(arguments[1])?)
}
