use crate::{attr::{Attr, GetAttr}, expr::{block::Block, ident::Ident}, stmt::decl::Decl};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub FunctionBuilder => pub Function {
        pub params: Vec<Decl>,
        pub body: Block,
        pub return_type_annotation: Option<Ident>,
        pub attr: Attr,
    }
}

impl GetAttr for Function {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}