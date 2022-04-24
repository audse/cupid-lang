use std::hash::{Hash, Hasher};
use crate::{TypeKind, Type, Symbol, GenericType, Expression, Tree, Value, SymbolFinder, ErrorHandler, Token};

#[derive(Debug, Clone)]
pub struct AliasType {
	pub true_type: Box<TypeKind>,
}

impl Type for AliasType {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		self.true_type.apply_arguments(arguments)
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		self.true_type.convert_primitives_to_generics(generics)
	}
}

impl PartialEq for AliasType {
	fn eq(&self, other: &Self) -> bool {
		self.true_type == other.true_type
	}
}

impl Eq for AliasType {}

impl Hash for AliasType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.true_type.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefineAlias {
	pub token: Token,
	pub symbol: Symbol,
	pub true_type: Box<Expression>,
	pub generics: Vec<Symbol>
}

impl Tree for DefineAlias {
	fn resolve(&self, scope: &mut crate::LexicalScope) -> Value {
		if let Value::Type(mut true_type) = self.true_type.resolve(scope) {
			true_type.convert_primitives_to_generics(&self.resolve_generics());
			let new_alias = TypeKind::Alias(AliasType { true_type: Box::new(true_type) });
			if let Some(new_alias) = scope.define_type(&self.symbol, new_alias) {
				new_alias
			} else {
				self.error("unable to define type")
			}
		} else {
			self.error("unable to define type")
		}
	}
}

impl ErrorHandler for DefineAlias {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("defining alias type {} for type {:?}", self.symbol, self.true_type)
	}
}

impl DefineAlias {
	fn resolve_generics(&self) -> Vec<GenericType> {
		self.generics
			.iter()
			.map(|g| GenericType::new(&g.get_identifier(), None))
			.collect()
	}
}