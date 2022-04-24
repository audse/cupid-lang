use std::collections::HashMap;
use crate::{TypeKind, Symbol, Value, Token};

#[derive(Debug, Clone)]
pub struct LexicalScope {
	pub scopes: Vec<Scope>
}

impl Default for LexicalScope {
	fn default() -> Self {
		Self::new(ScopeContext::Global)
	}
}

pub trait SymbolFinder {
	fn get_symbol(&self, symbol: &Symbol) -> Option<Value>;
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<TypeKind>;
	fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value>;
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value>;
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value>;
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value>;
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)>;
}

impl LexicalScope {
	pub fn new(context: ScopeContext) -> Self {
		let mut scope = Self {
			scopes: vec![]
		};
		scope.add(context);
		scope
	}
	pub fn last(&self) -> Option<&Scope> {
		self.scopes.last()
	}
	pub fn last_mut(&mut self) -> Option<&mut Scope> {
		self.scopes.last_mut()
	}
	pub fn add(&mut self, context: ScopeContext) -> &Scope {
		let scope = Scope::new(context);
		self.scopes.push(scope);
		self.last().unwrap()
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.scopes.pop()
	}
	pub fn within_function(&self) -> bool {
		for scope in self.scopes.iter().rev() {
			if scope.context == ScopeContext::Function {
				return true
			}
		}
		false
	}
	pub fn pretty_print_storage(&self) {
		for scope in self.scopes.iter().rev() {
			scope.pretty_print_storage();
		}
	}
}
impl SymbolFinder for LexicalScope {
	fn get_symbol(&self, symbol: &Symbol) -> Option<Value> {
		for scope in self.scopes.iter().rev() {
			if let Some(value) = scope.get_symbol(symbol) {
				return Some(value);
			}
		}
		None
	}
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<TypeKind> {
		for scope in self.scopes.iter().rev() {
			if let Some(symbol_type) = scope.get_symbol_type(symbol) {
				return Some(symbol_type);
			}
		}
		None
	}
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)> {
		for scope in self.scopes.iter().rev() {
			if let Some((mutable, deep_mutable)) = scope.is_mutable(symbol) {
				return Some((mutable, deep_mutable));
			}
		}
		None
	}
	fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value> {
		let current_scope = self.scopes.iter_mut().rev().find(|scope| 
			scope.get_symbol(symbol).is_some()
		);
		if let Some(scope) = current_scope {
			return scope.set_symbol(symbol, value);
		}
		Err(not_found_error(symbol))
	}
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.create_symbol(symbol, value, mutable, deep_mutable);
		}
		None
	}
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.create_symbol_of_type(symbol, value, symbol_type, mutable, deep_mutable);
		}
		None
	}
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.define_type(symbol, value)
		}
		None
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeContext {
	Global,
	Loop,
	Function,
	Boxed,
	Map,
	Block,
}

#[derive(Debug, Clone)]
pub struct Scope {
	pub storage: ScopeStorage,
	pub closed: bool,
	pub context: ScopeContext
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
	SymbolVal(SymbolVal),
	TypeValue(TypeKind),
}

#[derive(Debug, Clone)]
pub struct SymbolVal {
	pub value: Value,
	pub r#type: TypeKind,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl SymbolValue {
	pub fn get_string(&self) -> String {
		match self {
			SymbolValue::SymbolVal(v) => v.value.to_string(),
			SymbolValue::TypeValue(t) => t.to_string()
		}
	}
}

impl Default for Scope {
	fn default() -> Self {
		Self::new(ScopeContext::Global)
	}
}

type ScopeStorage = HashMap<Value, SymbolValue>;

impl Scope {

	pub fn new(context: ScopeContext) -> Self {
		Self {
			storage: HashMap::new(),
			closed: false,
			context,
		}
	}
	pub fn pretty_print_storage(&self) {
		let items: Vec<String> = self.storage
			.iter()
			.map(|(k, v)|  format!("{}: {}", k.to_string(), v.get_string()))
			.collect();
		println!("Scope: {:#?}", items);
	}
}

impl SymbolFinder for Scope {

	fn get_symbol(&self, symbol: &Symbol) -> Option<Value> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return match stored_symbol {
				SymbolValue::SymbolVal(symbol_value) => Some(symbol_value.value.clone()),
				SymbolValue::TypeValue(type_value) => Some(Value::Type(type_value.clone()))
			}
		}
		None
	}
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<TypeKind> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return match stored_symbol {
				SymbolValue::SymbolVal(symbol_value) => Some(symbol_value.r#type.clone()),
				SymbolValue::TypeValue(_) => None
			}
		}
		None
	}
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return match stored_symbol {
				SymbolValue::SymbolVal(symbol_value) => Some((symbol_value.mutable, symbol_value.deep_mutable)),
				SymbolValue::TypeValue(type_value) => None
			};
		}
		None
	}

	fn set_symbol(&mut self, symbol: &Symbol, new_value: Value) -> Result<Option<Value>, Value> {
		if let Some(stored_value) = self.storage.get_mut(&symbol.identifier) {
			if let Some(immutable_assign_error) = assign_to_immutable_error(symbol, stored_value) {
				return Err(immutable_assign_error)
			}
			match stored_value {
				SymbolValue::SymbolVal(symbol_value) => {
					symbol_value.value = new_value;
				},
				SymbolValue::TypeValue(_) => {
					return Err(symbol.error_unable_to_assign(&new_value))
				}
			}
			return Ok(self.get_symbol(symbol))
		}
		Err(not_found_error(symbol))
	}
	
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue::SymbolVal(SymbolVal {
			r#type: TypeKind::from_value(&value),
			value, 
			mutable, 
			deep_mutable,
		}));
		self.get_symbol(symbol)
	}
	
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue::SymbolVal(SymbolVal {
			r#type: symbol_type,
			value, 
			mutable, 
			deep_mutable,
		}));
		self.get_symbol(symbol)
	}
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue::TypeValue(value));
		self.get_symbol(symbol)
	}
}

fn assign_to_immutable_error(symbol: &Symbol, stored_value: &SymbolValue) -> Option<Value> {
	if let SymbolValue::SymbolVal(SymbolVal { value: _, r#type: _, mutable, deep_mutable: _ }) = stored_value {
		if !mutable {
			return Some(Value::error(
				&symbol.token,
				format!(
					"variable `{symbol}` is immutable and cannot be reassigned",
					symbol = symbol.get_identifier(),
				), String::new()
			))
		}
		None
	} else {
		return Some(Value::error(
			&symbol.token,
			format!(
				"variable `{symbol}` is a type and cannot be reassigned",
				symbol = symbol.get_identifier(),
			), String::new()
		))
	}
}

fn not_found_error(symbol: &Symbol) -> Value {
	Value::error(
		&symbol.token,
		format!(
			"variable `{symbol}` is not defined",
			symbol = symbol.get_identifier(),
		), String::new()
	)
}

fn type_not_found_error(symbol: &Symbol, token: &Token) -> Value {
	Value::error(
		token,
		format!(
			"type `{symbol}` is not defined",
			symbol = symbol.identifier
		), String::new()
	)
}
