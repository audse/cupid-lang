use super::*;

build_struct! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
    pub BlockBuilder => pub Block {
        #[tabled(display_with="fmt_vec")]
        pub body: Vec<Exp>,
        #[tabled(skip)]
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

impl UseClosure for Block {}