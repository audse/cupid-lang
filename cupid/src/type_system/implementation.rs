use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Implementation {
	pub functions: HashMap<ValueNode, ValueNode>,
	pub traits: HashMap<TypeHintNode, Implementation>,
	pub generics: Vec<GenericType>,
}

impl From<HashMap<ValueNode, ValueNode>> for Implementation {
	fn from(functions: HashMap<ValueNode, ValueNode>) -> Self {
		Self {
			functions,
			traits: HashMap::new(),
			generics: vec![]
		}
	}
}

impl Implementation {
	// TODO make sure params match
	pub fn get_function(&self, symbol: &SymbolNode) -> Option<FunctionNode> {
		if let Some(func) = self.functions.get(&symbol.0) {
			if let Value::Function(function) = &func.value {
				return Some(function.to_owned())
			}
		}
		None
	}
	pub fn get_trait_function(&self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<(Implementation, FunctionNode)> {
		if let Some(function) = self.get_function(symbol) {
			return Some((self.to_owned(), function.to_owned()))
		}
		for implement in self.traits.iter() {
			if let Some(function) = implement.1.get_function(symbol) {
				return Some((implement.1.to_owned(), function.to_owned()));
			} else {
				let prev_implement = scope.get_value(&SymbolNode::from(implement.0), &Self::from_scope_value);
				if let Ok(Some(prev)) = prev_implement {
					if let Some(trait_function) = prev.get_trait_function(symbol, scope) {
						return Some(trait_function);
					}
				}
			}
		}
		None
	}
	pub fn from_scope_value(symbol_value: &SymbolValue) -> Result<Option<Implementation>, Error> {
		Ok(Option::<Implementation>::from(symbol_value))
	}
	pub fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) {
		functions.into_iter().for_each(|(k, v)| {
			self.functions.insert(k, v); 
		});
	}
	pub fn implement_trait(&mut self, trait_symbol: TypeHintNode, implement: Implementation) {
		self.traits.insert(trait_symbol, implement);
	}
	pub fn set_generic_symbols(&self, meta: &Meta<Flag>, scope: &mut LexicalScope) -> Result<(), Error> {
		for generic in self.generics.iter() {
			create_generic_symbol(generic, meta, scope)?;
		}
		Ok(())
	}
}

impl From<&SymbolValue> for Option<Implementation> {
	fn from(symbol_value: &SymbolValue) -> Self {
		if let SymbolValue::Declaration { value, .. } = symbol_value {
			if let Value::Implementation(v) = &value.value {
				return Some(v.to_owned());
			}
		}
		None
	}
}

impl Hash for Implementation {
	fn hash<H: Hasher>(&self, state: &mut H) {
		for (symbol, _) in self.functions.iter() {
			symbol.hash(state);
		}
		for (trait_symbol, _) in self.traits.iter() {
			trait_symbol.hash(state);
		}
	}
}

impl PartialEq for Implementation {
	fn eq(&self, other: &Self) -> bool {
		self.functions == other.functions
			&& self.traits == other.traits
	}
}

impl Eq for Implementation {}

impl Display for Implementation {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {		
		let functions: Vec<String> = self.functions
			.iter()
			.map(|(key, _)| key.to_string())
			.collect();
		let traits: Vec<String> = self.traits
			.iter()
			.map(|(key, _)| key.to_string())
			.collect();
		let generics: Vec<String> = self.generics
			.iter()
			.map(|generic| generic.to_string())
			.collect();
		let generics: String = if !generics.is_empty() { 
			format!("{} ", generics.join(", ")) 
		} else { 
			String::new() 
		};
		write!(f, "[{generics}functions: [{}], traits: [{}]]", functions.join(", "), traits.join(", "))
	}
}