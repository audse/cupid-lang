use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{TypeKind, Symbol, Value, Type, Implementation, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
	fn get_type_of_symbol(&self, symbol: &Symbol) -> Result<TypeKind, String>;
	fn get_type(&self, value: &Value) -> Result<TypeKind, &str>;
	fn set_symbol(&mut self, symbol: &Symbol, value: Value) -> Result<Option<Value>, Value>;
	// fn create_placeholder(&mut self, symbol: &Symbol) -> Result<(), Error>;
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, symbol_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value>;
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value>;
	fn implement_type(&mut self, symbol: &Symbol, new_value: TypeKind) -> Option<Value>;
	fn define_trait(&mut self, symbol: &Symbol, value: HashMap<Value, Value>) -> Option<Value>;
	fn implement_trait(&mut self, type_symbol: &Symbol, trait_symbol: &Symbol, implement: HashMap<Value, Value>) -> Option<Value>;
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
	fn get_type_of_symbol(&self, symbol: &Symbol) -> Result<TypeKind, String> {
		let mut err = String::new();
		for scope in self.scopes.iter().rev() {
			match scope.get_type_of_symbol(symbol) {
				Ok(value) => return Ok(value),
				Err(e) => err = e,
			};
		}
		Err(err)
	}
	fn get_type(&self, value: &Value) -> Result<TypeKind, &str> {
		for scope in self.scopes.iter().rev() {
			if let Ok(symbol_type) = scope.get_type(value) {
				return Ok(symbol_type);
			}
		}
		Err("type could not be found in scope")
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
		Err(symbol.error_undefined())
	}
	// fn create_placeholder(&mut self, symbol: &Symbol) -> Result<(), Error> {
	// 	if let Some(scope) = self.last_mut() {
	// 		return scope.create_placeholder(symbol);
	// 	}
	// 	Err(symbol.error_raw("Cannot create placeholder"))
	// }
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, symbol_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.create_symbol(symbol, value, symbol_type, mutable, deep_mutable);
		}
		None
	}
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.define_type(symbol, value)
		}
		None
	}
	fn define_trait(&mut self, symbol: &Symbol, value: HashMap<Value, Value>) -> Option<Value> {
		if let Some(scope) = self.last_mut() {
			return scope.define_trait(symbol, value)
		}
		None
	}
	fn implement_type(&mut self, symbol: &Symbol, new_value: TypeKind) -> Option<Value> {
		for scope in self.scopes.iter_mut().rev() {
			if let Some(_) = scope.get_symbol(symbol) {
				return scope.implement_type(symbol, new_value);
			}
		}
		None
	}
	fn implement_trait(&mut self, type_symbol: &Symbol, trait_symbol: &Symbol, implement: HashMap<Value, Value>) -> Option<Value> {
		for scope in self.scopes.iter_mut().rev() {
			if let Some(_) = scope.get_symbol(type_symbol) {
				return scope.implement_trait(type_symbol, trait_symbol, implement);
			}
		}
		None
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeContext {
	Global,
	Loop,
	Function,
	Boxed,
	Map,
	Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
	pub storage: ScopeStorage,
	pub closed: bool,
	pub context: ScopeContext
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolValue {
	Variable(Variable),
	Placeholder,
	Type(TypeKind),
	Trait(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
	pub value: Value,
	pub value_type: TypeKind,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl SymbolValue {
	pub fn get_string(&self) -> String {
		use SymbolValue::*;
		match self {
			Variable(v) => v.value.to_string(),
			Type(t) => t.to_string(),
			Trait(t) => t.to_string(),
			Placeholder => String::new(),
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
				SymbolValue::Variable(symbol_value) => Some(symbol_value.value.clone()),
				SymbolValue::Type(type_value) => Some(Value::Type(type_value.clone())),
				SymbolValue::Trait(trait_value) => Some(trait_value.clone()),
				_ => None,
			}
		}
		None
	}
	fn get_type_of_symbol(&self, symbol: &Symbol) -> Result<TypeKind, String> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return match stored_symbol {
				SymbolValue::Variable(symbol_value) => Ok(symbol_value.value_type.clone()),
				SymbolValue::Type(type_kind) => Ok(type_kind.to_owned()),
				SymbolValue::Trait(_) => Err(format!("symbol {symbol} is a trait, and does not have a type")),
				_ => Err(String::new())
			}
		}
		Err(format!("symbol {symbol} could not be found in the current scope"))
	}
	fn get_type(&self, value: &Value) -> Result<TypeKind, &str> {
		let type_symbol = Value::String(TypeKind::infer_name(value));
		if let Some(stored_symbol) = self.storage.get(&type_symbol) {
			return match stored_symbol {
				SymbolValue::Type(type_value) => Ok(type_value.clone()),
				_ => Err("stored identifier is not a type")
			}
		}
		Err("type could not be found in scope")
	}
	fn is_mutable(&self, symbol: &Symbol) -> Option<(bool, bool)> {
		if let Some(stored_symbol) = self.storage.get(&symbol.identifier) {
			return match stored_symbol {
				SymbolValue::Variable(symbol_value) => Some((symbol_value.mutable, symbol_value.deep_mutable)),
				_ => None,
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
				SymbolValue::Variable(symbol_value) => {
					if symbol_value.value_type.is_equal(&new_value) {
						symbol_value.value = new_value;
					} else {
						return Err(symbol.error_assign_type_mismatch(&new_value, symbol_value.value_type.clone()))
					}
				},
				SymbolValue::Type(_) => return Err(symbol.error_immutable_type()),
				SymbolValue::Trait(_) => return Err(symbol.error_immutable_trait()),
				_ => panic!()
			}
			return Ok(self.get_symbol(symbol))
		}
		Err(symbol.error_undefined())
	}
	
	// fn create_placeholder(&mut self, symbol: &Symbol) -> Result<(), Error> {
	// 	self.storage.insert(symbol.identifier.clone(), SymbolValue::Placeholder);
	// 	Ok(())
	// }
	
	fn create_symbol(&mut self, symbol: &Symbol, value: Value, value_type: TypeKind, mutable: bool, deep_mutable: bool) -> Option<Value> {
		if value_type.is_equal(&value) {
			self.storage.insert(symbol.identifier.clone(), SymbolValue::Variable(Variable {
				value_type,
				value, 
				mutable, 
				deep_mutable,
			}));
		} else {
			return Some(symbol.error_assign_type_mismatch(&value, value_type.clone()))
		}
		self.get_symbol(symbol)
	}
	
	fn define_type(&mut self, symbol: &Symbol, value: TypeKind) -> Option<Value> {
		self.storage.insert(symbol.identifier.clone(), SymbolValue::Type(value));
		self.get_symbol(symbol)
	}
	
	fn define_trait(&mut self, symbol: &Symbol, value: HashMap<Value, Value>) -> Option<Value> {
		let implementation = Implementation { functions: value, traits: HashMap::new(), };
		self.storage.insert(symbol.identifier.clone(), SymbolValue::Trait(Value::Trait(implementation)));
		self.get_symbol(symbol)
	}
	
	fn implement_type(&mut self, symbol: &Symbol, new_value: TypeKind) -> Option<Value> {
		self.storage.entry(symbol.identifier.clone())
			.and_modify(|entry| {
				match &entry {
					SymbolValue::Type(_) => {
						*entry = SymbolValue::Type(new_value);
					},
					_ => panic!()
				};
			});
		self.get_symbol(symbol)
	}
	
	fn implement_trait(&mut self, type_symbol: &Symbol, trait_symbol: &Symbol, implement: HashMap<Value, Value>) -> Option<Value> {
		self.storage.entry(type_symbol.identifier.clone())
			.and_modify(|entry| {
				match entry {
					SymbolValue::Type(ref mut type_kind) => {
						_ = type_kind.implement_trait(trait_symbol.clone(), implement);
					},
					_ => panic!()
				}
			});
		self.get_symbol(type_symbol)
	}
}

fn assign_to_immutable_error(symbol: &Symbol, stored_value: &SymbolValue) -> Option<Value> {
	match stored_value {
		SymbolValue::Variable(Variable { mutable, .. }) => if !mutable {
			Some(symbol.error_immutable())
		} else {
			None
		},
		_ => Some(symbol.error_immutable_type())
	}
}