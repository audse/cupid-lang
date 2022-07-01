use crate::infer::Infer;
use cupid_ast::{expr, stmt::decl::Decl, types, types::typ::Type};

impl Infer<Type> for expr::Expr {
    fn infer(&self) -> Type {
        use expr::Expr::*;
        match self {
            Block(block) => block.infer(),
            Function(function) => function.infer(),
            FunctionCall(function_call) => function_call.infer(),
            Namespace(namespace) => namespace.infer(),
            Ident(ident) => ident.infer(),
            Trait(t) => t.infer(),
            Type(typ) => typ.infer(),
            Value(value) => value.infer(),
            Stmt(stmt) => stmt.infer(),
            Empty => ().infer(),
        }
    }
}

impl Infer<Type> for expr::block::Block {
    fn infer(&self) -> Type {
        self.expressions
            .iter()
            .last()
            .map(|expr| expr.infer())
            .unwrap_or_else(|| ().infer())
    }
}

impl Infer<Type> for expr::function::Function {
    fn infer(&self) -> Type {
        let mut fields = self.params.clone();
        let return_type = Decl {
            ident: "returns".into(),
            type_annotation: self.return_type_annotation.clone(),
            ..Decl::default()
        };
        fields.push(return_type);
        Type {
            ident: "fun".into(),
            fields,
            ..Type::default()
        }
    }
}

impl Infer<Type> for expr::function_call::FunctionCall {
    fn infer(&self) -> Type {
        Type::variable()
    }
}

impl Infer<Type> for expr::ident::Ident {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for expr::namespace::Namespace {
    fn infer(&self) -> Type {
        self.value.infer()
    }
}

impl Infer<Type> for types::traits::Trait {
    fn infer(&self) -> Type {
        Type::traits()
    }
}

impl Infer<Type> for types::typ::Type {
    fn infer(&self) -> Type {
        Type::typ()
    }
}

impl Infer<Type> for expr::value::Value {
    fn infer(&self) -> Type {
        use expr::value::Val::*;
        match &self.inner {
            VBoolean(x) => x.infer(),
            VChar(x) => x.infer(),
            VDecimal(x, y) => (*x, *y).infer(),
            VInteger(x) => x.infer(),
            VNone => ().infer(),
            VString(x) => x.infer(),
        }
    }
}
