use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, array, integer, string, .. } = engine.primitives;
    engine.add_constant_value("Array", array);
    engine.add_method_primitive(array, "to_string", [array],               &to_string);
    engine.add_method_primitive(array, "copy",      [array],               &copy);
    engine.add_method_primitive(array, "append",    [array, any],          &append);
    engine.add_method_primitive(array, "prepend",   [array, any],          &prepend);
    engine.add_method_primitive(array, "insert",    [array, integer, any], &insert);
    engine.add_method_primitive(array, "remove",    [array, integer],      &remove);
    engine.add_method_primitive(array, "__cn__",    [array, string],       &cn);
    engine.add_method_primitive(array, "__id__",    [array, array],        &id);
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    if name == "__id__" {
        return Ok(engine.new_constant(arguments[0]))
    }

    Ok(engine.new_method(this.get_method(&name)?, this))
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().clone();
    for element in elements.iter() {
        string.push_str(&element.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if !elements.is_empty() {
        string.truncate(string.len() - 2);
    }

    string.push(']');
    Ok(engine.new_string(string))
}

fn copy<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_array(arguments[0].data_array().clone()))
}

fn append<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().push(reference);
    Ok(engine.undefined())
}

fn prepend<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().insert(0, reference);
    Ok(engine.undefined())
}

fn insert<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[2]);
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().insert(index, reference);
    Ok(engine.undefined())
}

fn remove<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().remove(index);
    Ok(engine.undefined())
}

fn id<'a>(_: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(arguments[0].data_array()[*arguments[1].data_array()[0].read()?.data_integer() as usize])
}
