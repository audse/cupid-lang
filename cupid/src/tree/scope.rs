use std::collections::HashMap;
use crate::{Symbol, Value, Type, TypeSymbol, Token, TypeChecker, use_builtin_types};

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
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<Type>;
	fn get_definition(&self, symbol: &TypeSymbol) -> Option<Type>;
	fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value>;
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value>;
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: Type, mutable: bool, deep_mutable: bool) -> Option<Value>;
	fn define_type(&mut self, symbol: &TypeSymbol, value: Type) -> Result<Type, Value>;
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)>;
}

impl LexicalScope {
	pub fn new(context: ScopeContext) -> Self {
		let mut scope = Self {
			scopes: vec![]
		};
		scope.add(context);
		use_builtin_types(&mut scope);
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
	pub fn pretty_print_definitions(&self) {
		for scope in self.scopes.iter().rev() {
			scope.pretty_print_definitions();
		}
	}
	// fn assign_type_mismatch_error(&self, symbol: &Symbol, value: &Value) -> Result<(), Value> {
	// 	if let Some(symbol_value) = self.get_symbol(&symbol) {
	// 		if self.is_type(value, &symbol_value.r#type.symbol) {
	// 			return Ok(())
	// 		} else {
	// 			return Err(symbol.error_assign_type_mismatch(value, &symbol_value.r#type.symbol));
	// 		}
	// 	}
	// 	return Ok(())
	// }
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
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<Type> {
		for scope in self.scopes.iter().rev() {
			if let Some(symbol_type) = scope.get_symbol_type(symbol) {
				return Some(symbol_type);
			}
		}
		None
	}
	fn get_definition(&self, symbol: &TypeSymbol) -> Option<Type> {
		for scope in self.scopes.iter().rev() {
			if let Some(stored_type) = scope.get_definition(symbol) {
				return Some(stored_type);
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
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: Type, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.create_symbol_of_type(symbol, value, symbol_type, mutable, deep_mutable);
		}
		None
	}
	fn define_type(&mut self, symbol: &TypeSymbol, value: Type) -> Result<Type, Value> {
		if let Some(scope) = self.last_mut() {
			return scope.define_type(symbol, value);
		}
		Err(type_not_found_error(symbol, symbol.token.as_ref().unwrap()))
	}
}

impl TypeChecker for LexicalScope {}

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
	pub definitions: HashMap<String, Type>,
	pub closed: bool,
	pub context: ScopeContext
}

#[derive(Debug, Clone)]
pub struct SymbolValue {
	pub value: Value,
	pub r#type: Type,
	pub mutable: bool,
	pub deep_mutable: bool,
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
			definitions: HashMap::new(),
			closed: false,
			context,
		}
	}
	pub fn pretty_print_storage(&self) {
		let items: Vec<String> = self.storage
			.iter()
			.map(|(k, v)|  format!("{}: {}", k.to_string(), v.value))
			.collect();
		println!("Scope: {:#?}", items);
	}
	pub fn pretty_print_definitions(&self) {
		let items: Vec<String> = self.definitions
			.iter()
			.map(|(k, v)|  format!("{}: {}", k.to_string(), v))
			.collect();
		println!("Scope: {:#?}", items);
	}
	
	// fn set_or_create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Value {
	// 	if let Ok(new_symbol) = self.set_symbol(symbol, value.clone()) {
	// 		new_symbol.unwrap()
	// 	} else {
	// 		let new_symbol = self.create_symbol(symbol, value, mutable, deep_mutable);
	// 		new_symbol.unwrap()
	// 	}
	// }
}

impl SymbolFinder for Scope {

	fn get_symbol(&self, symbol: &Symbol) -> Option<Value> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return Some(stored_symbol.value.clone());
		}
		None
	}
	fn get_symbol_type(&self, symbol: &Symbol) -> Option<Type> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return Some(stored_symbol.r#type.clone());
		}
		None
	}
	fn get_definition(&self, symbol: &TypeSymbol) -> Option<Type> {
		if let Some(stored_type) = self.definitions.get(&symbol.name.to_string()) {
			return Some(stored_type.clone());
		}
		None
	}
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return Some((stored_symbol.mutable, stored_symbol.deep_mutable));
		}
		None
	}

	fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value> {
		if let Some(stored_value) = self.storage.get_mut(&symbol.identifier) {
			if let Some(immutable_assign_error) = assign_to_immutable_error(symbol, stored_value) {
				return Err(immutable_assign_error)
			}
			stored_value.value = value;
			return Ok(self.get_symbol(symbol))
		}
		Err(not_found_error(symbol))
	}
	
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, mutable: bool, deep_mutable: bool) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue {
			r#type: Type::from(&value),
			value, 
			mutable, 
			deep_mutable,
		});
		self.get_symbol(symbol)
	}
	
	fn create_symbol_of_type(&mut self, symbol: &Symbol, value: Value, symbol_type: Type, mutable: bool, deep_mutable: bool) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue {
			r#type: symbol_type,
			value, 
			mutable, 
			deep_mutable,
		});
		self.get_symbol(symbol)
	}
	
	fn define_type(&mut self, symbol: &TypeSymbol, value: Type) -> Result<Type, Value> {
		if self.definitions.contains_key(&symbol.name.to_string()) {
			let symbol_ref = symbol.token.as_ref().unwrap();
			return Err(Value::error(symbol_ref, format!("there is already a type called `{}`", symbol.name), String::new()));
		}
		self.definitions.insert(symbol.name.to_string(), value);
		Ok(self.get_definition(symbol).unwrap())
	}
	
	// fn closed_scope_error(&self, symbol: &Symbol) -> Option<Value> {
	// 	if !self.closed {
	// 		if let Some(_) = &self.parent {
	// 			return None;
	// 		}
	// 		return Some(Value::error(
	// 			&symbol.1[0],
	// 			format!(
	// 				"`{symbol}` could not be found in the current scope",
	// 				symbol = symbol.get_identifier()
	// 			)
	// 		));
	// 	} else {
	// 		Some(Value::error(
	// 			&symbol.1[0],
	// 			format!(
	// 				"`{symbol}` is outside the current scope and can't be changed",
	// 				symbol = symbol.get_identifier()
	// 			)
	// 		))
	// 	}
	// }
}

fn assign_to_immutable_error(symbol: &Symbol, stored_value: &SymbolValue) -> Option<Value> {
	if !stored_value.mutable {
		return Some(Value::error(
			&symbol.token,
			format!(
				"variable `{symbol}` is immutable and cannot be reassigned",
				symbol = symbol.get_identifier(),
			), String::new()
		))
	}
	None
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

fn type_not_found_error(symbol: &TypeSymbol, token: &Token) -> Value {
	Value::error(
		token,
		format!(
			"type `{symbol}` is not defined",
			symbol = symbol.name
		), String::new()
	)
}
