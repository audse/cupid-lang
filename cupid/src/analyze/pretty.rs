use crate::{
    arena::{EntryId, ExprArena, UseArena},
    ast::{self, Expr, GetTy},
    for_expr_variant,
    pointer::Pointer,
    scope::symbol::{Symbol, SymbolValue},
    ty,
};

pub trait PrettyPrint<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String;
}

impl<'src, T: PrettyPrint<'src>> PrettyPrint<'src> for Vec<T> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        let items: Vec<String> = self.iter().map(|item| item.pretty_print(arena)).collect();
        format!("[{}]", items.join(", "))
    }
}

impl<'src, T: PrettyPrint<'src>> PrettyPrint<'src> for Option<T> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        match self {
            Some(value) => value.pretty_print(arena),
            None => "None".to_string(),
        }
    }
}

impl<'src> PrettyPrint<'src> for EntryId {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        let expr: &Expr<'src> = arena.expect(*self);
        expr.pretty_print(arena)
    }
}

impl<'src> PrettyPrint<'src> for Pointer<Symbol<'src>> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        let value = match self.borrow().value {
            SymbolValue::Class(class) => format!("Class('{}')", class.0),
            SymbolValue::Instance(instance) => format!("Instance('{}')", instance.0),
            SymbolValue::Expr(expr) => expr.pretty_print(arena),
            SymbolValue::Unset => "None".to_string(),
        };
        let ty = self.borrow().ty.pretty_print(arena);
        format!("Symbol {{ value: {value}, ty: {ty} }}")
    }
}

impl<'src> PrettyPrint<'src> for Expr<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        for_expr_variant!(self => |inner| inner.pretty_print(arena))
    }
}

impl<'src> PrettyPrint<'src> for ast::Array<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Array {{ items: {}, ty: {} }}",
            self.items.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::BinOp<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "BinOp {{ left: {}, right: {}, op: {:?}, ty: {} }}",
            self.left.pretty_print(arena),
            self.right.pretty_print(arena),
            self.op,
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Block<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Block {{ body: {}, ty: {} }}",
            self.body.pretty_print(arena),
            self.ty().pretty_print(arena),
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Break<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Break {{ value: {}, ty: {} }}",
            self.value.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Call<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Call {{ callee: {}, args: {}, ty: {} }}",
            self.callee.pretty_print(arena),
            self.args.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Class<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Class {{ name: {:?}, fields: {}, methods: {}, ty: {} }}",
            self.name,
            self.fields.pretty_print(arena),
            self.methods.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Constant<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!("Constant {{ value: {}, ty: {} }}", self.value, self.ty().pretty_print(arena))
    }
}

impl<'src> PrettyPrint<'src> for ast::Define<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Define {{ name: {:?}, value: {}, ty: {} }}",
            self.name,
            self.value.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Fun<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Fun {{ name: {:?}, params: {}, body: {}, ty: {} }}",
            self.name,
            self.params.pretty_print(arena),
            self.body.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Get<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Get {{ name: {:?}, symbol: {}, ty: {} }}",
            self.name,
            self.symbol.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::GetProperty<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "GetProperty {{ receiver: {}, property: {:?}, symbol: {}, ty: {} }}",
            self.receiver.pretty_print(arena),
            self.property,
            self.symbol.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::GetSuper<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "GetSuper {{ super: {:?}, symbol: {}, ty: {} }}",
            self.name,
            self.symbol.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::If<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "If {{ condition: {}, body: {}, else_body: {}, ty: {} }}",
            self.condition.pretty_print(arena),
            self.body.pretty_print(arena),
            self.else_body.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Invoke<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Invoke {{ receiver: {}, callee: {:?}, symbol: {}, args: {}, ty: {} }}",
            self.receiver.pretty_print(arena),
            self.callee,
            self.symbol.pretty_print(arena),
            self.args.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::InvokeSuper<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "InvokeSuper {{ super: {:?}, symbol: {}, args: {}, ty: {} }}",
            self.name,
            self.symbol.pretty_print(arena),
            self.args.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Loop<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Loop {{ body: {}, ty: {} }}",
            self.body.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Method<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Method {{ name: {:?}, params: {}, body: {}, ty: {} }}",
            self.name,
            self.fun.params.pretty_print(arena),
            self.fun.body.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Return<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Return {{ value: {}, ty: {} }}",
            self.value.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::Set<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "Set {{ name: {:?}, value: {}, symbol: {}, ty: {} }}",
            self.name,
            self.value.pretty_print(arena),
            self.symbol.pretty_print(arena),
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::SetProperty<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "SetProperty {{ receiver: {}, property: {:?}, value: {}, symbol: {}, ty: {} }}",
            self.receiver.pretty_print(arena),
            self.property,
            self.value.pretty_print(arena),
            self.symbol.pretty_print(arena),
            self.ty().pretty_print(arena),
        )
    }
}

impl<'src> PrettyPrint<'src> for ast::UnOp<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        format!(
            "UnOp {{ expr: {}, op: {:?}, ty: {} }}",
            self.expr.pretty_print(arena),
            self.op,
            self.ty().pretty_print(arena)
        )
    }
}

impl<'src> PrettyPrint<'src> for ty::Type<'src> {
    fn pretty_print(&self, arena: &ExprArena<'src>) -> String {
        match self {
            Self::Array(items) => {
                let ty = arena.expect_ty(*items);
                format!("Array<{}>", ty.pretty_print(arena))
            }
            Self::Function { returns } => {
                let ty = arena.expect_ty(*returns);
                format!("Function<{}>", ty.pretty_print(arena))
            }
            _ => self.to_string(),
        }
    }
}
