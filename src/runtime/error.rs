use crate::node::Node;
use crate::runtime::value::GcValue;

pub struct Error<'a> {
    pub message: Box<str>,
    pub node: Option<&'a Node<'a>>,
}

impl Error<'_> {
    fn new(message: String) -> Self {
        Self {
            message: Box::from(message),
            node: None
        }
    }

    pub fn new_runtime(error: &str) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: ";
        message += error;
        Self::new(message)
    }

    pub fn new_undefined_method(method: &str, class: GcValue) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: Method \"";
        message += method;
		message += "\" is undefined in the type ";
		message += &class.data_class().tag.to_string();
        message += ".";
        Self::new(message)
    }

    pub fn new_undeclared_variable(variable: &str) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: Variable \"";
        message += variable;
        message += "\" is not declared.";
        Self::new(message)
    }

    pub fn new_control() -> Self {
        Self::new(String::from("RUNTIME ERROR: Cannot loop control out of a function."))
    }

    pub fn new_undefined() -> Self {
        Self::new(String::from("RUNTIME ERROR: Cannot read an undefined reference."))
    }

    pub fn new_constant_write() -> Self {
        Self::new(String::from("RUNTIME ERROR: Cannot write data into a constant."))
    }

    pub fn new_arguments(parameters: usize, arguments: usize) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: Provided ";
        message += &arguments.to_string();
        message += " arguments while the function expects ";
        message += &parameters.to_string();
        message += " parameters.";
        Self::new(message)
    }

    pub fn new_cast(value: GcValue, r#type: GcValue) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: Cannot cast a value of the type ";
		message += &value.class.data_class().tag.to_string();
		message += " to the type ";
		message += &r#type.data_class().tag.to_string();
        message += ".";
        Self::new(message)
    }
}
