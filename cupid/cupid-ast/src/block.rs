use super::*;

build_struct! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub BlockBuilder => pub Block {
        pub body: Vec<Exp>,
        pub attributes: Attributes,
    }
}

impl UseAttributes for Block {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}