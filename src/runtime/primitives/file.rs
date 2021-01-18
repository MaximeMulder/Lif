use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

use std::fs;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, file, string, .. } = engine.primitives;
    engine.add_constant_value("File", file);
    engine.add_static_primitive(file, "read",  [string],      &read);
    engine.add_static_primitive(file, "write", [string, any], &write);
}

fn read<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let string = fs::read_to_string(arguments[0].data_string()).unwrap();
    Ok(engine.new_string(string))
}

fn write<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    fs::write(arguments[0].data_string(), arguments[1].call_to_string(engine)?).unwrap();
    Ok(engine.undefined())
}