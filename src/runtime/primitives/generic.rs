use crate::memory::Ref;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, generic, string, .. } = engine.primitives;
    engine.add_constant_value("Generic", generic);
    engine.add_method_primitive(generic, "to_string", [generic],         &to_string);
    engine.add_method_primitive(generic, "__cn__",    [generic, string], &cn);
    engine.add_method_primitive(generic, "__gn__",    [generic, array],  &gn);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    if name == "__gn__" {
        return Ok(engine.new_constant(arguments[0]))
    }

    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    Err(Error::new_undefined_method(&name, this))
}

fn gn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    engine.push_scope();
    let value = arguments[0];
    let generic = value.data_generic();
    for (parameter, argument) in generic.generics.iter().zip(arguments[1].data_array()) {
        let reference = engine.new_reference(argument.read()?);
        engine.add_variable(parameter, reference);
    }

    let reference = engine.execute(Ref::as_ref(&generic.node))?;
    engine.pop_scope();
    Ok(reference)
}
