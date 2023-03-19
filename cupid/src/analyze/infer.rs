use crate::{
    arena::{EntryId, ExprArena, UseArena},
    ast::{
        Array, BinOp, Block, Break, Call, Class, Constant, Define, Expr, Fun, Get, GetProperty,
        GetSuper, GetTy, HasSymbol, Header, If, Invoke, InvokeSuper, Loop, Method, Return, Set,
        SetProperty, UnOp,
    },
    auto_impl, base_pass,
    error::CupidError,
    for_expr_variant, pass,
    ty::Type,
    value::Value,
};

auto_impl! {
    pub trait Infer<'src>
    where
        Self: Sized,
    {
        fn infer(self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError>;
    }
}

base_pass! {
    impl Infer::infer(arena: &mut ExprArena<'src>) for {
        Block,
    }
}

fn entry_ty<'src>(id: EntryId, arena: &mut ExprArena<'src>) -> Type<'src> {
    UseArena::<Expr>::expect(arena, id).ty()
}

fn unwrapped_ty<'src, T: GetTy<'src>>(value: &Option<T>) -> Type<'src> {
    match value {
        Some(inner) => inner.ty(),
        None => Type::Unknown,
    }
}

fn unwrapped_entry_ty<'src>(id: Option<EntryId>, arena: &mut ExprArena<'src>) -> Type<'src> {
    match id {
        Some(inner) => entry_ty(inner, arena),
        None => Type::Unknown,
    }
}

impl<'src> Infer<'src> for EntryId {
    fn infer(self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let expr: Expr<'src> = arena.take(self);
        let expr = expr.infer(arena)?;
        arena.replace(self, expr);
        Ok(self)
    }
}

impl<'src> Infer<'src> for Array<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Array::infer(self, arena));
        let item_ty = match self.items.get(0) {
            Some(id) => entry_ty(*id, arena),
            None => Type::Unknown,
        };
        self.header.ty = Type::Array(arena.insert(item_ty));
        Ok(self)
    }
}

impl<'src> Infer<'src> for BinOp<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(BinOp::infer(self, arena));
        self.set_ty(entry_ty(self.left, arena));
        Ok(self)
    }
}

impl<'src> Infer<'src> for Break<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Break::infer(self, arena));
        match self.value {
            Some(id) => self.set_ty(entry_ty(id, arena)),
            None => (),
        }
        Ok(self)
    }
}

impl<'src> Infer<'src> for Call<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Call::infer(self, arena));
        match entry_ty(self.callee, arena) {
            Type::Function { returns } => self.set_ty(*arena.expect_ty(returns)),
            Type::Class(class) => self.set_ty(Type::Instance(class)),
            Type::Unknown => (),
            _ => return Err(CupidError::type_error("Not a function", "")),
        }
        Ok(self)
    }
}

impl<'src> Infer<'src> for Class<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let name = self.name.lexeme;
        let ty = Type::class(name);
        self.scope_mut().annotate_ty(name, ty);
        self.class_scope_mut().annotate_ty("self", ty);
        if let Some(super_class) = self.super_class {
            self.class_scope_mut().annotate_ty("super", Type::class(super_class.lexeme));
        }
        self.set_ty(Type::class(name));
        pass!(Class::infer(self, arena));
        Ok(self)
    }
}

impl<'src> Infer<'src> for Constant<'src> {
    fn infer(mut self, _arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let ty = match self.value {
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Nil => Type::Nil,
            Value::String(_) => Type::String,
            _ => Type::Unknown,
        };
        self.header.ty = ty;
        Ok(self)
    }
}

impl<'src> Infer<'src> for Define<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Define::infer(self, arena));
        let ty = unwrapped_entry_ty(self.value, arena);
        let name = self.name.lexeme;
        self.scope_mut().annotate_ty(name, ty);
        self.set_ty(Type::Unit);
        Ok(self)
    }
}

impl<'src> Infer<'src> for Fun<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Fun::infer(self, arena));
        let body_ty = entry_ty(self.body, arena);
        self.header.ty = Type::Function {
            returns: arena.insert(body_ty),
        };
        if let Some(name) = self.name {
            let ty = self.ty();
            self.scope_mut().annotate_ty(name.lexeme, ty);
        }
        Ok(self)
    }
}

impl<'src> Infer<'src> for Get<'src> {
    fn infer(mut self, _arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let ty = self.expect_symbol()?.ty;
        self.set_ty(ty);
        Ok(self)
    }
}

impl<'src> Infer<'src> for GetProperty<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(GetProperty::infer(self, arena));
        self.set_ty(unwrapped_ty(&self.symbol));
        Ok(self)
    }
}

impl<'src> Infer<'src> for GetSuper<'src> {
    fn infer(mut self, _arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let ty = self.expect_symbol()?.ty;
        self.set_ty(ty);
        Ok(self)
    }
}

impl<'src> Infer<'src> for If<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(If::infer(self, arena));
        self.set_ty(entry_ty(self.body, arena));
        Ok(self)
    }
}

impl<'src> Infer<'src> for Invoke<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Invoke::infer(self, arena));
        self.set_ty(unwrapped_ty(&self.symbol));
        Ok(self)
    }
}

impl<'src> Infer<'src> for InvokeSuper<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(InvokeSuper::infer(self, arena));
        self.set_ty(Type::Unit);
        Ok(self)
    }
}

impl<'src> Infer<'src> for Loop<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Loop::infer(self, arena));
        self.set_ty(entry_ty(self.body, arena));
        Ok(self)
    }
}

impl<'src> Infer<'src> for Method<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Method::infer(self, arena));
        let ty = self.fun.header.ty;
        let name = self.name.lexeme;
        self.set_ty(ty);
        self.scope_mut().annotate_ty(name, ty);
        Ok(self)
    }
}

impl<'src> Infer<'src> for Return<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Return::infer(self, arena));
        self.set_ty(unwrapped_entry_ty(self.value, arena));
        Ok(self)
    }
}

impl<'src> Infer<'src> for Set<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Set::infer(self, arena));
        self.set_ty(Type::Unit);

        // Infer types from assignments, if the type is unknown
        if self.symbol.is_some() {
            let value_ty = entry_ty(self.value, arena);
            let mut symbol = self.expect_symbol_mut()?;
            match symbol.ty {
                Type::Unknown => symbol.ty = value_ty,
                _ => (),
            }
        }

        Ok(self)
    }
}

impl<'src> Infer<'src> for SetProperty<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(SetProperty::infer(self, arena));
        self.set_ty(Type::Unit);

        // Infer types from assignments, if the type is unknown
        if self.symbol.is_some() {
            let value_ty = entry_ty(self.value, arena);
            let mut symbol = self.expect_symbol_mut()?;
            match symbol.ty {
                Type::Unknown => symbol.ty = value_ty,
                _ => (),
            }
        }

        Ok(self)
    }
}

impl<'src> Infer<'src> for UnOp<'src> {
    fn infer(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(UnOp::infer(self, arena));
        self.set_ty(entry_ty(self.expr, arena));
        Ok(self)
    }
}
