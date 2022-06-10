use super::*;

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {		
		match self {
			VArray(val, ..) | VTuple(val, ..) => write!(f, "{}", fmt_list!(val, ", ")),
			VBoolean(b, ..) => write!(f, "{b}"),
			VChar(c, ..) => write!(f, "{c}"),
			VDecimal(a, b, ..) => write!(f, "{a}.{b}"),
			VFunction(fun) => write!(f, "{fun}"),
			VInteger(i, ..) => write!(f, "{i}"),
			VNone(_) => write!(f, "none"),
			VString(s, ..) => write!(f, "{s}"),
			VType(t) => write!(f, "{t}"),
			VTrait(t) => write!(f, "{t}"),
			VBuiltinPlaceholder(_) => write!(f, "placeholder!")
		}
	}
}

impl AsTable for Value {}

impl AsTable for Attributes {}
impl Display for Attributes {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table().with(Rotate::Left).with(Style::modern()))
	}
}

impl AsTable for Trait {}
impl Display for Trait {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut table = self.as_table();
		if self.bounds.is_empty() {
			table = table.with(Disable::Column(2..3));
		}
		if self.methods.is_empty() {
			table = table.with(Disable::Column(1..2));
		}
		write!(f, "{table}")
	}
}

impl AsTable for Function {}
impl Display for Function {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let func = tabled::builder::Builder::new()
			.set_columns(0..2)
			.add_record(["params", &(&self.params).table().with(Style::modern()).to_string()])
			.add_record(["=>", &self.return_type.to_string()])
			.build()
			.with(Disable::Row(0..1))
			.with(Style::modern())
			.with_bold_header("Function");
		write!(f, "\n{func}")
	}
}


impl AsTable for Ident {}
impl Display for Ident {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let g = fmt_list!(self.attributes.generics.0);
		let g = fmt_if_nonempty!(g, format!(" [{}]", g.join(", ")));
		write!(f, "{}{g}", self.name)
	}
}

impl AsTable for Field {}
impl Display for Field {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} {}", &self.name, fmt_option!(&self.type_hint, |x| format!(" : {x}")))
	}
}

impl AsTable for FieldSet {}
impl Display for FieldSet {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for Type {}
impl Display for Type {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut table = self.as_table()
			.with(Rotate::Left)
    		.with(
				Modify::new(object::Columns::single(0))
					.with(Format::new( |s| s.bold().to_string()) )
					.with(Alignment::right())
			);
			
		if self.fields.is_empty() {
			table = table.with(Disable::Row(3..4));
		}
		if self.traits.is_empty() {
			table = table.with(Disable::Row(2..3));
		}
		if self.methods.is_empty() {
			table = table.with(Disable::Row(1..2));
		}
		write!(f, "{}", 
			table.with(Style::modern())
				.with_bold_header("Type")
				.with(
					Modify::new(object::Rows::last())
						.with(Format::new( |s| s.bold().to_string()) )
				)
		)
	}
}

impl AsTable for Block {}
impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{{ {} }}", fmt_list!(self.body, "\n"))
	}
}

impl AsTable for Declaration {}
impl Display for Declaration {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for Exp {
	fn as_table(&self) -> Table {
		for_each_exp!(self, as_table)
	}
}

impl Display for Exp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		if let Exp::Empty = self {
			write!(f, "empty")
		} else {
			write!(f, "{}", for_each_exp!(self, to_string))
		}
	}
}

impl AsTable for FunctionCall {}
impl Display for FunctionCall {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}[{}]({})", self.function.0, fmt_option!(&self.function.1, |x| format!("({x})")), fmt_list!(self.args, ", "))
	}
}

impl AsTable for Property {}
impl Display for Property {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}.{}", self.object, self.property)
	}
}

impl AsTable for PropertyTerm {}
impl Display for PropertyTerm {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Self::Index(index, _) => write!(f, "{index}"),
			Self::FunctionCall(function_call) => write!(f, "{function_call}"),
			Self::Term(term) => write!(f, "{term}"),
		}
	}
}

impl AsTable for Method {}
impl Display for Method {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} {}", self.name, self.value)
	}
}

impl AsTable for GenericList {}
impl Display for GenericList {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", fmt_list!(self.0, ", "))
	}
}

impl AsTable for BaseType {}
impl AsTable for Context {}

impl AsTable for SymbolValue {}
impl Display for SymbolValue {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for TypeDef {}
impl Display for TypeDef {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for Implement {}
impl Display for Implement {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for TraitDef {}
impl Display for TraitDef {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table())
	}
}