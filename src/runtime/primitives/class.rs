use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, array_any, class, string, .. } = engine.primitives;
    engine.set_constant_value("Class", class);
    engine.primitive_method(class, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(class, "__cn__", [("property", string)], None, Some(any), &chain);
    engine.primitive_method(class, "__cl__", [("arguments", array_any)], None, Some(any), &call);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let this = arguments[0].data_class();
    let mut string = this.tag().to_string();
    if let Some(constructor) = this.constructor() {
        string.push_str("[");
        string.push_str(&constructor.arguments.iter()
            .map(|argument| argument.call_sstr(engine))
            .collect::<Return<Box<[_]>>>()?
            .join(", ")
        );

        string.push_str("]");
    }

    Ok(engine.new_string(string))
}

fn chain<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let mut this = arguments[0];
    let name = arguments[1].data_string();
    if let Some(method) = this.class.data_class().get_method(name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.new_variable(None, engine.primitives.any);
    let class = this.data_class_mut();
    Ok(if let Some(member) = class.get_static(name) {
        member
    } else {
        class.set_static(name, member);
        member
    })
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let class = arguments[0];
    Ok(if let Some(member) = class.data_class().get_static("__init__") {
        member.read()?.data_function().call(engine, &mut parameters::unpack(arguments[1])?)?
    } else {
        return Err(error_constructor(class))
    })
}

fn error_constructor(class: GcValue) -> Error {
    Error::new_runtime(&format!("Class {} has no default constructor.", class.data_class().tag()))
}
