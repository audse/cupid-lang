use crate::Attributes;
use Value::*;

// Since a block is just a collection of expressions, it doesn't need a new 
// structure for any passes. So we can just use a generic function to resolve
// each pass.

cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub BlockBuilder => pub Block<E: Default + Clone> {
        pub expressions: Vec<E>,
    }
}

impl<T: Default + Clone> Block<T> {
    pub fn pass<R: Default + Clone>(
        self, 
        fun: impl FnOnce(Vec<T>, &mut cupid_scope::Env) -> crate::PassResult<Vec<R>>, 
        env: &mut cupid_scope::Env
    ) -> crate::PassResult<Block<R>> {
        let Block { expressions, attr } = self;
        Ok(Block::build()
            .expressions(fun(expressions, env)?)
            .attr(attr)
            .build())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Field<Id>(pub Id, pub Option<Id>);

impl<Id: Default + Clone> Field<Id> {
	pub fn pass<NextId: Default + Clone>(
		self,
		fun: impl FnOnce(Id, &mut cupid_scope::Env) -> crate::PassResult<NextId>,
		option_fun: impl FnOnce(Option<Id>, &mut cupid_scope::Env) -> crate::PassResult<Option<NextId>>,
		env: &mut cupid_scope::Env,
	) -> crate::PassResult<Field<NextId>> {
		let Field(name, annotation) = self;
		Ok(Field(
			fun(name, env)?,
			option_fun(annotation, env)?
		))
	}
}

#[derive(Debug, Clone)]
pub enum Value {
	VBoolean(bool, Attributes),
	VChar(char, Attributes),
	VDecimal(i32, u32, Attributes),
	VInteger(i32, Attributes),
	VString(cupid_util::Str, Attributes),
	VNone(Attributes),
}


impl Default for Value {
	fn default() -> Self {
		Self::VNone(Attributes::default())
	}
}

impl Value {
	pub fn attr(&self) -> Attributes {
		match self {
			VBoolean(_, attr)
			| VChar(_, attr)
			| VDecimal(_, _, attr)
			| VInteger(_, attr)
			| VString(_, attr)
			| VNone(attr) => *attr
		}
	}
	pub fn attr_mut(&mut self) -> &mut Attributes {
		match self {
			VBoolean(_, attr)
			| VChar(_, attr)
			| VDecimal(_, _, attr)
			| VInteger(_, attr)
			| VString(_, attr)
			| VNone(attr) => attr
		}
	}
}

impl crate::AsNode for Value {
	fn source(&self) -> crate::Source { self.attr().source() }
	fn scope(&self) -> crate::Scope { self.attr().scope() }
	fn typ(&self) -> crate::Address { self.attr().typ() }
	fn set_source(&mut self, source: crate::Source) { self.attr_mut().source = source }
	fn set_scope(&mut self, scope: crate::Scope) { self.attr_mut().scope = scope }
	fn set_typ(&mut self, typ: crate::Address) { self.attr_mut().typ = typ }
}