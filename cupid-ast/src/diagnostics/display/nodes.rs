use super::*;

// impl Display for Val {
// 	fn fmt(&self, f: &mut Formatter) -> Result {		
// 		match self {
// 			Val::Array(val) | Val::Tuple(val) => write!(f, "{}", fmt_list!(val, ", ")),
// 			Val::Boolean(b) => write!(f, "{b}"),
// 			Val::Char(c) => write!(f, "{c}"),
// 			Val::Decimal(a, b) => write!(f, "{a}.{b}"),
// 			Val::Function(fun) => write!(f, "{fun}"),
// 			Val::Integer(i) => write!(f, "{i}"),
// 			Val::None => write!(f, "none"),
// 			Val::String(s) => write!(f, "{s}"),
// 			Val::Type(t) => write!(f, "{t}"),
// 			Val::Trait(t) => write!(f, "{t}"),
// 		}
// 	}
// }

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", &*self.val)
	}
}

impl Display for GenericParam {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let name = fmt_option!(&self.0);
		let arg = fmt_option!(&self.1, |x| format!(": {x}"));
		write!(f, "<{name}{arg}>")
	}
}

impl Display for Trait {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let bounds = fmt_list!(self.bounds, ", ");
		let methods = fmt_list!(self.methods, |m| m.signature.to_string(), ", ");
		write!(f, "(trait {}: {bounds} = [{methods}])", self.name)
	}
}


impl Display for Function {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let params = fmt_list!(
			self.params, 
			|p| format!("{} {}", &*p.type_hint, p.name.name), 
			", "
		);
		write!(f, "({params} => {{ .. }})")
	}
}


impl Display for Ident {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let g = fmt_list!(self.attributes.generics.0);
		let g = fmt_if_nonempty!(g, format!(" [{}]", g.join(", ")));
		write!(f, "{}{g}", self.name)
	}
}

impl Display for FieldSet {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Self::Unnamed(fields) => write!(f, "{}", fmt_list!(fields, ", ")),
			Self::Named(fields) => write!(f, "{}", fmt_list!(fields, |(a, b)| format!("{a} {b}"), ", ")),
			_ => Ok(())
		}
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let traits = fmt_list!(self.traits, ", ");
		let methods = fmt_list!(self.methods, |m| m.signature.to_string(), ", ");
		write!(f, "(type {} = [{}], [{traits}], [{methods}])", self.name, self.fields)
	}
}
