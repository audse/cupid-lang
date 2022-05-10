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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymbolValue<'src> {
	Declaration {
		type_hint: TypeKind<'src>,
		mutable: bool,
		value: ValueNode<'src>
	},
	Assignment {
		value: ValueNode<'src>
	},
	Implementation {
		trait_symbol: Option<SymbolNode<'src>>,
		value: Implementation<'src>
	},
}

impl<'src> From<ValueNode<'src>> for SymbolValue<'src> {
	fn from(value: ValueNode) -> Self {
		Self::Declaration { 
			type_hint: value.type_kind.to_owned(),
			mutable: false, 
			value
		}
	}
}

impl<'src> SymbolValue<'src> {
	fn get_value(&self, symbol: &SymbolNode) -> ValueNode {
		match self {
			Self::Declaration { value, .. } => value.to_owned(),
			Self::Assignment { value } => value.to_owned(),
			Self::Implementation { value, .. } => {
				ValueNode {
					value: Value::Implementation(value.to_owned()),
					type_kind: TypeKind::Type,
					meta: symbol.0.meta.to_owned()
				}
			},
		}
	}
}

pub trait Scope {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error>;
	fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, Error>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LexicalScope<'src> {
	pub scopes: Vec<SingleScope<'src>>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleScope<'src> {
	pub storage: HashMap<ValueNode<'src>, SymbolValue<'src>>,
	pub context: Context,
}

impl<'src> Default for LexicalScope<'src> {
	fn default() -> Self {
		let global_scope = SingleScope::new(Context::Global);
		Self {
			scopes: vec![global_scope]
		}
	}
}

impl<'src> LexicalScope<'src> {
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

impl<'src> Scope for LexicalScope<'src> {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error> {
		for scope in self.scopes.iter().rev() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value)
			}
		}
		Err(symbol.error_raw_context(
			format!("{symbol:?} could not be found in the current scope"), 
			format!("current scope: {self}")
		))
	}
	fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, Error> {
		// see if symbol already exists
		let mut container_scope: Option<&mut SingleScope> = None;
		for scope in self.scopes.iter_mut().rev() {
			if let Ok(_) = scope.get_symbol(symbol) {
				container_scope = Some(scope);
			}
		}
		if let Some(scope) = container_scope {
			// if symbol does exist, alter it
			scope.set_symbol(symbol, body)
		} else if let Some(scope) = self.last() {
			// otherwise, create it in latest scope
			scope.set_symbol(symbol, body)
		} else {
			Err(symbol.error_raw("symbol could not be set"))
		}
	}
}


impl<'src> Display for LexicalScope<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let scopes: Vec<String> = self.scopes
			.iter()
			.map(|s| s.to_string())
			.collect();
		write!(f, "[\n{}\n]\n", scopes.join(",\n"))
	}
}

impl<'src> SingleScope<'src> {
	pub fn new(context: Context) -> Self {
		Self {
			storage: HashMap::new(),
			context
		}
	}
}

impl<'src> Scope for SingleScope<'src> {
	fn get_symbol(&self, symbol: &SymbolNode) -> Result<ValueNode, Error> {
    	if let Some(result) = self.storage.get(&symbol.0) {
			Ok(result.get_value(&symbol))
		} else {
			Err(symbol.error_raw("symbol could not be found in the current scope"))
		}
	}
	fn set_symbol(&mut self, symbol: &SymbolNode, body: SymbolValue) -> Result<ValueNode, Error> {
		use SymbolValue::*;
		
		let mut result: Result<(), Error> = Ok(());
		let entry = self.storage.entry(symbol.0.to_owned()).and_modify(|e| match e {
			Declaration { mutable: m, value: ref mut v, .. } => if *m {
				*v = body.to_owned().get_value(&symbol);
			} else {
				// Types can bypass immutability reasons in two cases
				if let Value::Type(ref mut type_kind) = &mut v.value {
					// Types can be implemented
					if let SymbolValue::Implementation { trait_symbol, value } = body.to_owned() {
						if let Some(trait_symbol) = trait_symbol {
							type_kind.implement_trait(trait_symbol, value);
						} else {
							type_kind.implement(value.functions);
						}
					// Generic types can be replaced
					} else if let TypeKind::Generic(_) = type_kind {
						*v = body.to_owned().get_value(&symbol);
					}
				} else {
					result = Err(error_immutable(
						symbol, 
						&e.get_value(symbol), 
						&body.get_value(symbol))
					);
				}
			},
			Assignment { value } => *value = body.to_owned().get_value(&symbol),
			Implementation { .. } => {}
		}).or_insert_with(|| body.to_owned());
		
		match result {
			Ok(()) => Ok(entry.get_value(&symbol)),
			Err(e) => Err(e),
		}
	}
}


impl<'src> Display for SingleScope<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let storage: Vec<String> = self.storage
			.iter()
			.map(|(k, v)| format!(
				"{:8}: {}", 
				k.to_string(), 
				v.get_value(&SymbolNode(k.to_owned())).to_string()
			))
			.collect();
		
		write!(f, "  {:?}: [{}  ]", self.context, crate::pretty!(storage))
	}
}

pub fn create_self_symbol<'src>(function: &FunctionNode, value: ValueNode, scope: &mut LexicalScope) -> Result<ValueNode<'src>, Error> {
	let self_symbol = &function.params.symbols[0].symbol;
	let declare = SymbolValue::Declaration { 
		type_hint: value.type_kind.to_owned(), 
		mutable: function.params.mut_self,
		value
	};
	scope.set_symbol(self_symbol, declare)
}

pub fn create_generic_symbol<'src>(generic: &GenericType, meta: &Meta<Flag>, scope: &mut LexicalScope) -> Result<ValueNode<'src>, Error> {
	let generic_name = Value::String(generic.identifier.to_owned());
	let symbol = SymbolNode(ValueNode::from((
		Value::TypeIdentifier(TypeId::from(generic_name)), 
		meta
	)));
	let value = if let Some(generic_value) = &generic.type_value {
		*generic_value.to_owned()
	} else {
		TypeKind::Generic(generic.to_owned())
	};
	let value = ValueNode::from((Value::Type(value), meta));
	
	let declare = SymbolValue::Declaration { 
		type_hint: TypeKind::Type, 
		mutable: false, 
		value
	};
	scope.set_symbol(&symbol, declare)
}

pub fn error_immutable(symbol: &SymbolNode, original_value: &ValueNode, assign_value: &ValueNode) -> Error {
	symbol.0.error_raw_context(
		format!("immutable: `{symbol}` is immutable and cannot be reassigned"),
		format!(
			"original value: {original_value} (type {}) \nattempted value: {assign_value} (type {})",
			&original_value.type_kind.get_name(),
			&assign_value.type_kind.get_name()
		)
	)
}