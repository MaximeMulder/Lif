use crate::runtime::data::{ Array, Class, Data, Function, Generic, Method, Nullable, Object };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;
use crate::runtime::utilities::tag::Tag;

pub type GcValue<'a> = GcRef<Value<'a>>;

pub struct Value<'a> {
    pub class: GcValue<'a>,
    data: Data<'a>,
}

impl<'a> Value<'a> {
    pub fn new(class: GcValue<'a>, data: Data<'a>) -> Self {
        Self {
            class,
            data,
        }
    }
}

impl<'a> GcValue<'a> {
    pub fn is(self, class: GcValue<'a>) -> bool {
        if self == class {
            true
        } else if let Some(parent) = self.data_class().parent() {
            parent.is(class)
        } else {
            false
        }
    }

    pub fn is_generic(self, generic: GcValue<'a>) -> bool {
        if let Some(constructor) = self.data_class().constructor() {
            if constructor.generic == generic {
                return true;
            }
        }

        false
    }
}

impl<'a> GcValue<'a> {
    pub fn isa(self, class: GcValue<'a>) -> bool {
        self.class.is(class)
    }

    pub fn isa_generic(self, generic: GcValue<'a>) -> bool {
        self.class.is_generic(generic)
    }

    pub fn cast(self, class: GcValue<'a>) -> Return<()> {
        if self.isa(class) {
            Ok(())
        } else {
            Err(error_cast(self, class))
        }
    }
}

impl<'a> GcValue<'a> {
    pub fn get_method(&self, name: &str) -> ReturnValue<'a> {
        if let Some(method) = self.class.data_class().get_method(name) {
            Ok(method)
        } else {
            Err(error_undefined_method(name, self.class))
        }
    }

    pub fn call_method(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        let mut values = Vec::new();
        values.push(self);
        values.extend_from_slice(arguments);
        self.call_method_self(engine, name, &mut values)
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        let method = self.get_method(name)?;
        let array = parameters::pack(engine, arguments);
        method.get_method("__cl__")?.data_function().call(engine, &mut [method, array])
    }

    pub fn call_fstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__fstr__", &mut [])?.read()?.data_string().clone())
    }

    pub fn call_sstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__sstr__", &mut [])?.read()?.data_string().clone())
    }
}

impl<'a> GcValue<'a> {
    pub fn get_cast_array(&self, engine: &Engine<'a>) -> Return<&Array<'a>> {
        if self.isa_generic(engine.primitives.array) {
            Ok(&self.data_array())
        } else {
            Err(error_cast(*self, engine.primitives.array_any))
        }
    }

    pub fn get_cast_boolean(&self, engine: &Engine<'a>) -> Return<&bool> {
        self.cast(engine.primitives.boolean)?;
        Ok(self.data_boolean())
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
        self.data.trace();
    }
}

macro_rules! data {
    ( $this:expr, $variant:ident ) => {
        if let Data::$variant(variant) = &$this.data {
            return variant;
        }

        panic!();
    };
}

macro_rules! data_mut {
    ( $this:expr, $variant:ident ) => {
        if let Data::$variant(variant) = &mut $this.data {
            return variant;
        }

        panic!();
    };
}

impl<'a> Value<'a> {
    pub fn data_tag(&self) -> Tag {
        match &self.data {
            Data::Class(class)       => class.tag().clone(),
            Data::Function(function) => function.tag().clone(),
            Data::Generic(generic)   => generic.tag().clone(),
            _ => panic!(),
        }
    }

    pub fn data_array(&self) -> &Array<'a> {
        data!(self, Array);
    }

    pub fn data_array_mut(&mut self) -> &mut Array<'a> {
        data_mut!(self, Array);
    }

    pub fn data_boolean(&self) -> &bool {
        data!(self, Boolean);
    }

    pub fn data_boolean_mut(&mut self) -> &mut bool {
        data_mut!(self, Boolean);
    }

    pub fn data_class(&self) -> &Class<'a> {
        data!(self, Class);
    }

    pub fn data_class_mut(&mut self) -> &mut Class<'a> {
        data_mut!(self, Class);
    }

    pub fn data_float(&self) -> &f64 {
        data!(self, Float);
    }

    pub fn data_float_mut(&mut self) -> &mut f64 {
        data_mut!(self, Float);
    }

    pub fn data_function(&self) -> &Function<'a> {
        data!(self, Function);
    }

    pub fn data_function_mut(&mut self) -> &mut Function<'a> {
        data_mut!(self, Function);
    }

    pub fn data_generic(&self) -> &Generic<'a> {
        data!(self, Generic);
    }

    pub fn data_generic_mut(&mut self) -> &mut Generic<'a> {
        data_mut!(self, Generic);
    }

    pub fn data_integer(&self) -> &isize {
        data!(self, Integer);
    }

    pub fn data_integer_mut(&mut self) -> &mut isize {
        data_mut!(self, Integer);
    }

    pub fn data_method(&self) -> &Method<'a> {
        data!(self, Method);
    }

    pub fn data_method_mut(&mut self) -> &mut Method<'a> {
        data_mut!(self, Method);
    }

    pub fn data_object(&self) -> &Object<'a> {
        data!(self, Object);
    }

    pub fn data_object_mut(&mut self) -> &mut Object<'a> {
        data_mut!(self, Object);
    }

    pub fn data_nullable(&self) -> &Nullable<'a> {
        data!(self, Nullable);
    }

    pub fn data_nullable_mut(&mut self) -> &mut Nullable<'a> {
        data_mut!(self, Nullable);
    }

    pub fn data_string(&self) -> &String {
        data!(self, String);
    }

    pub fn data_string_mut(&mut self) -> &mut String {
        data_mut!(self, String);
    }
}

fn error_undefined_method(method: &str, class: GcValue) -> Error {
    Error::new_runtime(&format!("Method `{}` is undefined for type `{}`.", method, class.data_class().tag()))
}

fn error_cast(value: GcValue, r#type: GcValue) -> Error {
    Error::new_runtime(&format!("Cannot cast a value of the type `{}` to the type `{}`.", value.class.data_class().tag(), r#type.data_class().tag()))
}
