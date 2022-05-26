use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Context {
	Global,
	Loop,
	Function,
	Boxed,
	Map,
	Block,
	Implementation,
	Closure,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymbolValue {
	Declaration {
		type_hint: Option<TypeHintNode>,
		mutable: bool,
		value: ValueNode
	},
	Assignment {
		value: ValueNode
	},
	Implementation {
		trait_symbol: Option<TypeHintNode>,
		value: Implementation
	},
}

impl From<ValueNode> for SymbolValue {
	fn from(value: ValueNode) -> Self {
		Self::Declaration { 
			type_hint: value.type_hint.to_owned(),
			mutable: false, 
			value
		}
	}
}

impl SymbolValue {
	pub fn get_value(&self, symbol: &SymbolNode) -> ValueNode {
		match self {
			Self::Declaration { value, .. } => value.to_owned(),
			Self::Assignment { value } => value.to_owned(),
			Self::Implementation { value, .. } => {
				ValueNode {
					value: Value::Implementation(value.to_owned()),
					type_hint: None,
					meta: symbol.0.meta.to_owned()
				}
			},
		}
	}
}

pub type ScopeError = (ValueNode, String, String);

pub trait Scope {
	fn get_symbol(&mut self, symbol: &SymbolNode) -> Result<ValueNode, ScopeError>;
	fn get_value<T>(&mut self, symbol: &SymbolNode, function: &dyn Fn(&SymbolValue) -> Result<T, ScopeError>) -> Result<T, ScopeError>;
	fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, ScopeError>;
	fn modify_symbol(&mut self, symbol: &SymbolNode, function: &dyn Fn(&mut SymbolValue)) -> Result<ValueNode, ScopeError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LexicalScope {
	pub scopes: Vec<SingleScope>,
	pub tokens: Vec<Vec<Token>>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleScope {
	pub storage: HashMap<ValueNode, SymbolValue>,
	pub context: Context,
}

impl Default for LexicalScope {
	fn default() -> Self {
		let global_scope = SingleScope::new(Context::Global);
		Self {
			scopes: vec![global_scope],
			tokens: vec![]
		}
	}
}

impl LexicalScope {
	pub fn new_from(&self) -> Self {
		let global_scopes: Vec<SingleScope> = self.scopes
			.iter()
			.filter_map(|s| if s.context == Context::Global {
				Some(s.to_owned())
			} else {
				None
			})
			.collect();
		Self {
			scopes: global_scopes,
			tokens: vec![],
		}
	}
	pub fn add(&mut self, context: Context) {
		self.scopes.push(SingleScope::new(context));
	}
	pub fn pop(&mut self) {
		self.scopes.pop();
	}
	pub fn last(&mut self) -> Option<&mut SingleScope> {
		self.scopes.iter_mut().last()
	}
	pub fn add_closure(&mut self, scope: SingleScope) {
		self.scopes.push(scope)
	}
	pub fn drop_closure(&mut self) -> SingleScope {
		self.scopes.pop().unwrap_or_else(|| SingleScope::new(Context::Closure))
	}
	fn get_scope_of(&mut self, symbol: &SymbolNode, break_on_closure: bool) -> Option<&mut SingleScope> {
		for scope in self.scopes.iter_mut().rev() {
			if let Ok(_) = scope.get_symbol(symbol) {
				return Some(scope);
			}
			if break_on_closure && scope.context == Context::Closure { break; }
		}
		None
	}
	pub fn push_tokens(&mut self, tokens: Vec<Token>) -> usize {
		self.tokens.push(tokens);
		self.tokens.len() - 1
	}
	pub fn get_symbol(&mut self, symbol: &SymbolNode) -> Result<ValueNode, Error> {
		for scope in self.scopes.iter_mut().rev() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value)
			}
			if scope.context == Context::Closure { break; }
		}
		if let Ok(value) = self.scopes.first_mut().unwrap().get_symbol(symbol)  {
			return Ok(value)
		}
		Err(into_error(error_undefined(symbol, self), self))
	}
	pub fn get_value<T>(&mut self, symbol: &SymbolNode, function: &dyn Fn(&SymbolValue) -> Result<T, ScopeError>) -> Result<T, Error> {
		for scope in self.scopes.iter_mut().rev() {
			if let Ok(value) = scope.get_value(symbol, function) {
				return Ok(value)
			}
		}
		Err(into_error(error_undefined(symbol, self), self))
	}
	pub fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, Error> {
		// see if symbol already exists
		let result = if let Some(scope) = self.get_scope_of(symbol, true) {
			// if symbol does exist, alter it
			scope.set_symbol(symbol, body)
		} else if let Some(scope) = self.last() {
			// otherwise, create it in latest scope
			scope.set_symbol(symbol, body)
		} else {
			return Err(into_error(error_cannot_set(symbol, Some(&body)), self))
		};
		self.to_result(result)
	}
	pub fn modify_symbol(&mut self, symbol: &SymbolNode, function: &dyn Fn(&mut SymbolValue)) -> Result<ValueNode, Error> {
		// see if symbol already exists
		let result = if let Some(scope) = self.get_scope_of(symbol, true) {
			// if symbol does exist, alter it
			scope.modify_symbol(symbol, function)
		} else if let Some(scope) = self.last() {
			// otherwise, create it in latest scope
			scope.modify_symbol(symbol, function)
		} else {
			return Err(into_error(error_cannot_set(symbol, None), self))
		};
		self.to_result(result)
	}
	pub fn to_result(&mut self, result: Result<ValueNode, ScopeError>) -> Result<ValueNode, Error> {
		match result {
			Ok(value) => Ok(value),
			Err(err) => Err(into_error(err, self))
		}
	}
}


impl Display for LexicalScope {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let scopes: Vec<String> = self.scopes
			.iter()
			.map(|s| s.to_string())
			.collect();
		write!(f, "[\n{}\n]\n", scopes.join(",\n"))
	}
}

impl SingleScope {
	pub fn new(context: Context) -> Self {
		Self {
			storage: HashMap::new(),
			context
		}
	}
}

fn into_error(err: ScopeError, scope: &mut LexicalScope) -> Error {
	let (value, message, context) = err;
	value.error_context(message, context, scope)
}


impl Scope for SingleScope {
	fn get_symbol(&mut self, symbol: &SymbolNode) -> Result<ValueNode, ScopeError> {
    	if let Some(result) = self.storage.get(&symbol.0) {
			Ok(result.get_value(&symbol))
		} else {
			// attempt to match symbol to type hint
			if let Some((_, result)) = self.storage.iter().find(|(key, _)| {
				(&key.value).type_hint_eq_to(&symbol.0.value)
			}) {
				Ok(result.get_value(&symbol))
			} else {
				Err(error_undefined(symbol, self))
			}
		}
	}	
	fn get_value<T>(&mut self, symbol: &SymbolNode, function: &dyn Fn(&SymbolValue) -> Result<T, ScopeError>) -> Result<T, ScopeError> {
		if let Some(result) = self.storage.get(&symbol.0) {
			function(result)
		} else {
			Err(error_undefined(symbol, self))
		}
	}
	fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, ScopeError> {
		use SymbolValue::*;
		
		let mut result: Result<(), ScopeError> = Ok(());
		let entry = self.storage.entry(symbol.0.to_owned()).and_modify(|e| match e {
			Declaration { mutable: m, value: ref mut v, .. } => if *m {
				*v = body.get_value(&symbol);
			} else {
				// Types can bypass immutability reasons in two cases
				if let Value::Type(ref mut type_kind) = &mut v.value {
					// Types can be implemented
					if let SymbolValue::Implementation { trait_symbol, mut value } = body.to_owned() {
						if let Some(trait_symbol) = trait_symbol {
							type_kind.implement_trait(trait_symbol, value);
						} else {
							type_kind.implement(value.functions);
							type_kind.implement_generics(&mut value.generics);
						}
					// Generic types can be replaced
					} else if let TypeKind::Generic(_) = type_kind {
						*v = body.get_value(&symbol);
					}
				} else {
					result = Err(error_immutable(
						symbol, 
						&e.get_value(symbol), 
						&body.get_value(symbol)
					));
				}
			},
			Assignment { value } => *value = body.get_value(&symbol),
			Implementation { .. } => {}
		}).or_insert_with(|| body.to_owned());
		
		match result {
			Ok(()) => Ok(entry.get_value(&symbol)),
			Err(e) => Err(e),
		}
	}
	fn modify_symbol(&mut self, symbol: &SymbolNode, function: &dyn Fn(&mut SymbolValue)) -> Result<ValueNode, ScopeError> {
		match self.storage.entry(symbol.0.to_owned()).and_modify(function) {
			Entry::Occupied(value) => Ok(value.get().get_value(symbol)),
			_ => Err(error_undefined(symbol, self))
		}
	}
}

impl Display for SingleScope {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let storage: Vec<String> = self.storage
			.iter()
			.map(|(k, v)| {
				let value = v.get_value(&SymbolNode(k.to_owned()));
				format!(
					"{:12}: {} {}", 
					k.to_string(),
					if let Some(type_hint) = &value.type_hint {
						type_hint.to_string()
					} else {
						String::new()
					},
					value.to_string()
				)
			})
			.collect();
		
		write!(f, "  {:?}: [{}  ]", self.context, crate::pretty!(storage))
	}
}

pub fn create_generic_symbol(generic: &GenericType, meta: &Meta<Flag>, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
	let symbol = SymbolNode::from(&TypeHintNode::new(generic.identifier.to_owned(), vec![TypeFlag::Primitive], vec![], meta.tokens.to_owned()));
	let value = ValueNode::from((Value::Type(TypeKind::Generic(generic.to_owned())), meta));
	
	let declare = SymbolValue::Declaration { 
		type_hint: None, 
		mutable: false, 
		value
	};
	scope.set_symbol(&symbol, declare)
}

fn error_immutable(symbol: &SymbolNode, original_value: &ValueNode, assign_value: &ValueNode) -> ScopeError {
	(
		symbol.0.to_owned(),
		format!("immutable: `{symbol}` is immutable and cannot be reassigned"),
		format!(
			"original value: {original_value} (type {}) \nattempted value: {assign_value} (type {})",
			unwrap_or_string(&original_value.type_hint),
			unwrap_or_string(&assign_value.type_hint)
		)
	)
}

fn error_undefined(symbol: &SymbolNode, scope: &impl std::fmt::Display) -> ScopeError {
	(
		symbol.0.to_owned(),
		format!("undefined: `{symbol}` could not be found in the current scope"), 
		format!("current scope: {scope}")
	)
}

fn error_cannot_set(symbol: &SymbolNode, assign_value: Option<&SymbolValue>) -> ScopeError {
	if let Some(assign_value) = assign_value {
		let assign_value = assign_value.get_value(symbol);
		(symbol.0.to_owned(), format!("cannot assign value `{assign_value}` to `{symbol}`"), String::new())
	} else {
		(symbol.0.to_owned(), format!("cannot assign to `{symbol}`"), String::new())
	}
}