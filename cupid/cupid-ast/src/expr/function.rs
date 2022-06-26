use crate::{attr::Attr, expr::{block::Block, ident::Ident}, stmt::decl::Decl};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub FunctionBuilder => pub Function {
        pub params: Vec<Decl>,
        pub body: Block,
        pub return_type_annotation: Option<Ident>,
        pub attr: Attr,
    }
}