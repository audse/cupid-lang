use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::{ValueNode, SymbolNode, TypeKind, Error, ErrorHandler };


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Context {
	Global,
	Loop,
	Function,
	Boxed,
	Map,
	Block,
	Implementation,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymbolValue {
	Declaration {
		type_hint: TypeKind,
		mutable: bool,
		value: ValueNode
	},
	Assignment {
		value: ValueNode
	},
}

impl Display for SymbolValue {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.get_value())
	}
}

impl SymbolValue {
	fn get_value(&self) -> ValueNode {
		match self {
			Self::Declaration { value, .. } => value.to_owned(),
			Self::Assignment { value } => value.to_owned(),
		}
	}
}

pub trait Scope {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error>;
	fn set_symbol(&mut self, symbol: &SymbolNode, body: &SymbolValue) -> Result<ValueNode, Error>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LexicalScope {
	pub scopes: Vec<SingleScope>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleScope {
	pub storage: HashMap<ValueNode, SymbolValue>,
	pub context: Context,
}

impl LexicalScope {
	pub fn new() -> Self {
		let global_scope = SingleScope::new(Context::Global);
		Self {
			scopes: vec![global_scope]
		}
	}
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
			scopes: global_scopes
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
}

impl Scope for LexicalScope {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error> {
		for scope in self.scopes.iter().rev() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value)
			}
		}
		Err(symbol.error_raw("symbol could not be found in the current scope"))
	}
	fn set_symbol(&mut self, symbol: &SymbolNode, body: &SymbolValue) -> Result<ValueNode, Error> {
		if let Some(scope) = self.last() {
			scope.set_symbol(symbol, body)
		} else {
			Err(symbol.error_raw("symbol could not be set"))
		}
	}
}


impl Display for LexicalScope {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let scopes: Vec<String> = self.scopes
			.iter()
			.map(|s| s.to_string())
			.collect();
		write!(f, "[\n{}\n]", scopes.join(",\n"))
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

impl Scope for SingleScope {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error> {
    	if let Some(result) = self.storage.get(&symbol.0) {
			Ok(result.get_value())
		} else {
			Err(symbol.error_raw("symbol could not be found in the current scope"))
		}
	}
	fn set_symbol(&mut self, symbol: &SymbolNode, body: &SymbolValue) -> Result<ValueNode, Error> {
		use SymbolValue::*;
		
		let mut result: Result<(), Error> = Ok(());
		let entry = self.storage.entry(symbol.0.to_owned()).and_modify(|e| match e {
			Declaration { mutable: m, value: ref mut v, .. } => if *m {
				*v = body.to_owned().get_value();
			} else {
				result = Err(symbol.error_raw("immutable"));
			},
			Assignment { value } => *value = body.to_owned().get_value()
		}).or_insert(body.to_owned());
		
		match result {
			Ok(()) => Ok(entry.get_value()),
			Err(e) => Err(e),
		}
	}
}


impl Display for SingleScope {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let storage: Vec<String> = self.storage
			.iter()
			.map(|(k, v)| format!("    {k}: {v}"))
			.collect();
		write!(f, "  {:?}: [\n{}\n  ]", self.context, storage.join(", \n"))
	}
}