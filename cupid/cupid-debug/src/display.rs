use colored::Colorize;
use cupid_ast::*;
use cupid_util::*;
use std::fmt::{Display, Formatter, Result};

pub trait Fmt {
    fn fmt_node(&self) -> String;
}

impl Fmt for Value {
    fn fmt_node(&self) -> String {
        match self {
            VArray(val, ..) | VTuple(val, ..) => fmt_list!(val, ", " => |v| (v as &dyn Fmt).to_string()),
            VBoolean(b, ..) => format!("{b}"),
            VChar(c, ..) => format!("{c}"),
            VDecimal(a, b, ..) => format!("{a}.{b}"),
            VFunction(fun) => (&**fun as &dyn Fmt).to_string(),
            VInteger(i, ..) => format!("{i}"),
            VNone(_) => "none".to_string(),
            VString(s, ..) => format!("{s}"),
            VType(t) => (t as &dyn Fmt).to_string(),
            VTrait(t) => (t as &dyn Fmt).to_string(),
            VBuiltinPlaceholder(_) => "placeholder!".to_string(),
        }
    }
}

impl Fmt for Attributes {
    fn fmt_node(&self) -> String {
        todo!()
    }
}

impl Fmt for Block {
    fn fmt_node(&self) -> String {
        todo!()
    }
}

impl Fmt for Declaration {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            "\nDeclaration:",
            format!("\n  type_hint: {}", self.type_hint.fmt_node()),
            format!("\n       name: {}", self.name.fmt_node()),
            format!("\n      value: {}\n", (&**self.value).fmt_node())
        )
    }
}

impl Fmt for Exp {
    fn fmt_node(&self) -> String {
        if let Exp::Empty = self {
            String::new()
        } else {
            for_each_exp!(self, fmt_node)
        }
    }
}

impl Fmt for Field {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            self.name.fmt_node(),
            fmt_option!(&self.type_hint => |t| format!(": {}", t.fmt_node()))
        )
    }
}

impl Fmt for FieldSet {
    fn fmt_node(&self) -> String {
        todo!()
    }
}

impl Fmt for Function {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            "Function".bold(),
            fmt_list!(self.params, ", " => |p| p.fmt_node()),
            "=> ",
            self.return_type.fmt_node()
        )
    }
}

impl Fmt for FunctionCall {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            self.function.inner().0.fmt_node(),
            "(", fmt_list!(&self.args, ", " => |arg| arg.fmt_node()), ")"
        )
    }
}

impl Fmt for Ident {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            self.name,
            fmt_if_nonempty!(&*self.attributes.generics => |list: &[Typed<Ident>]| 
                format!("({})", fmt_list!(list, ", " => |i: &Typed<Ident>| i.fmt_node())
            ))
        )
    }
}

impl Fmt for Implement {
    fn fmt_node(&self) -> String {
        quick_fmt!(self.for_type.fmt_node())
    }
}

impl Fmt for Method {
    fn fmt_node(&self) -> String {
        quick_fmt!(self.name.fmt_node(), self.value.fmt_node())
    }
}

impl Fmt for Property {
    fn fmt_node(&self) -> String {
        quick_fmt!(self.object.fmt_node(), ".", self.property.fmt_node())
    }
}

impl Fmt for PropertyTerm {
    fn fmt_node(&self) -> String {
        match self {
            Self::FunctionCall(function_call) => function_call.fmt_node(),
            Self::Index(i, ..) => i.to_string(),
            Self::Term(term) => term.fmt_node()
        }
    }
}

impl Fmt for SymbolValue {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            self.type_hint.fmt_node(),
            fmt_option!(&self.value => |v| format!(": {}", v.fmt_node()))
        )
    }
}

impl Fmt for Trait {
    fn fmt_node(&self) -> String {
        self.name.fmt_node()
    }
}

impl Fmt for TraitDef {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            "Define trait:\n",
            "  name: ", self.name.fmt_node()
        )
    }
}

impl Fmt for Type {
    fn fmt_node(&self) -> String {
        quick_fmt!(
            self.name.fmt_node()
        )
    }
}

impl Fmt for TypeDef {
    fn fmt_node(&self) -> String {
        todo!()
    }
}

impl<T: Display + Default + std::fmt::Debug + Fmt> Fmt for Typed<T> {
    fn fmt_node(&self) -> String {
        self.inner().to_string()
    }
}

impl Display for dyn Fmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.fmt_node())
    }
}