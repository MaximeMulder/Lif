use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::GenericCode;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::traits::WDefinition;

pub struct AGeneric {
    name: Option<Ref<str>>,
    parameters: Box<[Ref<str>]>,
    node: Box<ANode<dyn WDefinition>>,
}

impl AGeneric {
    pub fn new(name: Option<Ref<str>>, parameters: Box<[Ref<str>]>, node: Box<ANode<dyn WDefinition>>) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }
}

impl WDefinition for AGeneric {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parameters = self.parameters.iter()
            .map(|parameter| Box::from(parameter.as_ref()))
            .collect();

        Ok(engine.new_generic(Ref::as_option(&self.name), parameters, GenericCode::new(Ref::new(self.node.as_ref()))))
    }
}