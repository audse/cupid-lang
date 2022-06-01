use super::*;

impl Display for Val {
	fn fmt(&self, f: &mut Formatter) -> Result {		
		match self {
			Val::Array(val) | Val::Tuple(val) => write!(f, "{}", fmt_list!(val, ", ")),
			Val::Boolean(b) => write!(f, "{b}"),
			Val::Char(c) => write!(f, "{c}"),
			Val::Decimal(a, b) => write!(f, "{a}.{b}"),
			Val::Function(fun) => write!(f, "{fun}"),
			Val::Integer(i) => write!(f, "{i}"),
			Val::None => write!(f, "none"),
			Val::String(s) => write!(f, "{s}"),
			Val::Type(t) => write!(f, "{t}"),
			Val::Trait(t) => write!(f, "{t}"),
			Val::BuiltinPlaceholder => write!(f, "placeholder!")
		}
	}
}

impl AsTable for Value {}
impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", &*self.val)
	}
}

impl AsTable for Attributes {}
impl Display for Attributes {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.generics)
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
		let (return_type, params) = self.params.split_last().unwrap();

		let func = tabled::builder::Builder::new()
			.set_columns(0..2)
			.add_record(["params", &params.table().with(Style::modern()).to_string()])
			.add_record(["=>", &return_type.type_hint.to_string()])
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

impl AsTable for FieldSet {}
impl Display for FieldSet {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let fields = fmt_list!(self.0, |(a, b)| format!("{} {b}", fmt_option!(a, |x| format!("{x}:"))));
		write!(f, "{}", fmt_vec(&fields))
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
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{{ {} }}", fmt_list!(self.body, "\n"))
	}
}

impl AsTable for Declaration {}
impl Display for Declaration {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.as_table())
	}
}

impl AsTable for Exp {
	fn as_table(&self) -> Table {
		for_each_exp!(self, as_table)
	}
}

impl Display for Exp {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if let Exp::Empty = self {
			write!(f, "empty")
		} else {
			write!(f, "{}", for_each_exp!(self, to_string))
		}
	}
}

impl AsTable for FunctionCall {}
impl Display for FunctionCall {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}[{}]({})", self.function.0, fmt_option!(&self.function.1, |x| format!("({x})")), fmt_list!(self.args, ", "))
	}
}

impl AsTable for Property {}
impl Display for Property {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}.{}", self.object, self.property)
	}
}

impl AsTable for PropertyTerm {}
impl Display for PropertyTerm {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Index(index, _) => write!(f, "{index}"),
			Self::FunctionCall(function_call) => write!(f, "{function_call}"),
			Self::Term(term) => write!(f, "{term}"),
		}
	}
}

impl AsTable for Method {}
impl Display for Method {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{} \n{} {}", self.name, self.signature, fmt_option!(&self.value))
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

impl Display for SymbolValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", fmt_option!(&self.value))
	}
}