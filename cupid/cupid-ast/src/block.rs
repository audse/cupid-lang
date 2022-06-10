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

#[trace_this]
impl Trace for Block {
    fn trace_enter_block(&self, scope: &mut Env) {
        scope.trace("Entered block");
    }
    fn trace_exit_block(&self, scope: &mut Env) {
        scope.trace("Exited block");
    }
}