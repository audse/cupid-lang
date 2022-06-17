use cupid_types::infer::Infer;
use cupid_util::{InvertOption, Bx};
use crate::{name_resolution as prev_pass, PassResult, util, Env, Value, Address, Typ, AsNode, Ident};

/// Stores the inferred type of a node in the environment, accessible by the node's `source` attribute
#[cupid_semantics::auto_implement(Vec, Option, Box)]
pub trait InferTypes<Output> where Self: Sized {
    fn infer_types(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident_address: Address,
            pub type_annotation_address: Option<Address>,
        }
    }
    Function: util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {}
    }
    TypeDef: util::completed_node! { prev_pass::TypeDef => InferTypes<infer_types> }
}

crate::util::impl_default_passes! {
    impl InferTypes + infer_types for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
        crate::Ident;
        crate::IsTyped<Ident>;
    }
}

impl InferTypes<Decl> for prev_pass::Decl {
    fn infer_types(self, env: &mut Env) -> PassResult<Decl> {
        env.symbols.set_typ(self.source(), self.infer());
        env.modify_symbol(self.ident_address, |env, symbol_value: crate::SymbolValue| {
            let crate::SymbolValue { value, ..} = symbol_value;
            let new_value = match *value {
                crate::PassExpr::NameResolved(expr) => expr.infer_types(env)?,
                _ => return Err(value.err(cupid_util::ERR_CANNOT_INFER))
            };
            let new_value = crate::PassExpr::TypeInferred(new_value).bx();
            Ok(crate::SymbolValue { value: new_value, ..symbol_value })
        })?;
        todo!()
    }
}

impl Infer<Typ> for prev_pass::Decl {
    fn infer(&self) -> Typ {
        Typ::none()
    }
}

impl InferTypes<Function> for prev_pass::Function {
    fn infer_types(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}

impl Infer<Typ> for prev_pass::Function {
    fn infer(&self) -> Typ {
        todo!()
    }
}

impl InferTypes<Value> for Value {
    fn infer_types(self, env: &mut Env) -> PassResult<Value> {
        env.symbols.set_typ(self.source(), self.infer());
        Ok(self)
    }
}

impl Infer<Typ> for Value {
	fn infer(&self) -> Typ {
        use Value::*;
        let attr = self.attr();
		match self {
			VBoolean(x, ..) => Ident::new(x.infer(), attr),
			VChar(x, ..) => Ident::new(x.infer(), attr),
			VDecimal(x, y, ..) => Ident::new((*x, *y).infer(), attr),
			VInteger(x, ..) => Ident::new(x.infer(), attr),
			VString(x, ..) => Ident::new(x.infer(), attr),
            VType(x, ..) => x.infer(),
			_ => Ident::new(().infer(), attr),
		}.into()
	}
}

impl Infer<Ident> for Typ {
    fn infer(&self) -> Ident {
        Ident::new("type!", self.attr)
    }
}