use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum Val {
	Array(Vec<Val>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Function(Box<Typed<crate::Function>>),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	Tuple(Vec<Val>),
	Type(crate::Type),
	Trait(crate::Trait),
	BuiltinPlaceholder,
}

impl Default for Val {
	fn default() -> Self { Self::None }
}

impl TypeOf for Val {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
    	infer_type_from_scope(self, scope).map_err(|code| (0, code))
	}
}

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub ValueBuilder => pub Value {
		pub val: Typed<Val>, 

        #[tabled(skip)]
		pub attributes: Attributes
	}
}