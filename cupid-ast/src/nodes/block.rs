use crate::*;

build_struct! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
    pub BlockBuilder => pub Block {
        #[tabled(display_with="fmt_vec")]
        pub body: Vec<Exp>,
        #[tabled(skip)]
        pub attributes: Attributes,
    }
}