use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ flow, ReturnFlow };

pub struct Class {
    name: Option<Ref<str>>,
    parent: Option<Node>,
    methods: Box<[Node]>,
}

impl Class {
    pub fn new(name: Option<Ref<str>>, parent: Option<Node>, methods: Box<[Node]>) -> Self {
        Self {
            name,
            parent,
            methods,
        }
    }
}

impl Executable for Class {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parent = if let Some(parent) = self.parent.as_ref() {
            flow(engine.execute(parent)?.read())?
        } else {
            engine.primitives.object
        };

        let class = engine.new_class(Ref::as_option(&self.name), Some(parent));
        let mut value = flow(class.read())?;
        engine.run_frame(value.data_class().scope(), |engine| {
            let data = value.data_class_mut();
            for method in self.methods.iter() {
                let function = flow(engine.execute(method)?.read())?;
                data.set_method(function.data_tag().get_name().unwrap(), function);
            }

            Ok(())
        })?;

        Ok(class)
    }
}
