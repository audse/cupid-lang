use crate::{
    arena::{EntryId, ExprArena, UseArena},
    ast::{
        Array, BinOp, Block, Break, Call, Class, Constant, Define, Expr, Fun, Get, GetProperty,
        GetSuper, GetTy, Header, If, Invoke, InvokeSuper, Loop, Method, Return, Set, SetProperty,
        UnOp,
    },
    auto_impl, base_pass,
    error::CupidError,
    for_expr_variant, pass,
    scope::Lookup,
    ty::Type,
};

auto_impl! {
    pub trait Resolve<'src>
    where
        Self: Sized
    {
        fn resolve(self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError>;
    }
}

base_pass! {
    impl Resolve::resolve(_arena: &mut ExprArena<'src>) for {
        Array,
        BinOp,
        Block,
        Break,
        Call,
        Constant,
        If,
        Loop,
        Return,
        UnOp
    }
}

macro_rules! update_symbol {
    ($self:ident => $token_field:ident, $symbol_field:ident) => {{
        let name = $self.$token_field;
        let symbol = $self.scope().lookup(name);
        $self.$symbol_field = symbol;
    }};
}

impl<'src> Resolve<'src> for EntryId {
    fn resolve(self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let expr: Expr<'src> = arena.take(self);
        let expr = expr.resolve(arena)?;
        arena.replace(self, expr);
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Class<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let name = self.name;
        self.scope_mut().define(name);

        let scope = self.class_scope.clone();
        self.scope_mut().insert_class(name, scope);

        self.class_scope_mut().define("self");
        self.class_scope_mut().annotate_class(name);

        if let Some(super_class) = self.super_class {
            self.class_scope_mut().define("super");
            self.class_scope_mut().annotate_class(super_class);
        }
        pass!(Class::resolve(self, arena));
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Define<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let name = self.name;
        self.scope_mut().define(name);
        pass!(Define::resolve(self, arena));
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Fun<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        if let Some(name) = self.name {
            self.scope_mut().define(name);
        }
        pass!(Fun::resolve(self, arena));
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Get<'src> {
    fn resolve(mut self, _arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(self)
    }
}

impl<'src> Resolve<'src> for GetProperty<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(GetProperty::resolve(self, arena));
        let receiver_ty = UseArena::<Expr>::expect(arena, self.receiver).ty();
        match receiver_ty {
            Type::Class(_) => {
                let symbol = self.scope().lookup_property(receiver_ty, self.property)?;
                self.symbol = symbol;
            }
            _ => (),
        }
        Ok(self)
    }
}

impl<'src> Resolve<'src> for GetSuper<'src> {
    fn resolve(mut self, _arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Invoke<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(Invoke::resolve(self, arena));
        let receiver_ty = UseArena::<Expr>::expect(arena, self.receiver).ty();
        match receiver_ty {
            Type::Class(_) => {
                let symbol = self.scope().lookup_property(receiver_ty, self.callee)?;
                self.symbol = symbol;
            }
            _ => (),
        }
        Ok(self)
    }
}

impl<'src> Resolve<'src> for InvokeSuper<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        self.args = self.args.resolve(arena)?;
        Ok(self)
    }
}

impl<'src> Resolve<'src> for Method<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        let name = self.name;
        self.scope_mut().define(name);
        Ok(Method {
            fun: self.fun.resolve(arena)?,
            ..self
        })
    }
}

impl<'src> Resolve<'src> for Set<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(Set {
            value: self.value.resolve(arena)?,
            ..self
        })
    }
}

impl<'src> Resolve<'src> for SetProperty<'src> {
    fn resolve(mut self, arena: &mut ExprArena<'src>) -> Result<Self, CupidError> {
        pass!(SetProperty::resolve(self, arena));
        let receiver_ty = UseArena::<Expr>::expect(arena, self.receiver).ty();
        match receiver_ty {
            Type::Class(_) => {
                let symbol = self.scope().lookup_property(receiver_ty, self.property)?;
                self.symbol = symbol;
            }
            _ => (),
        }
        Ok(self)
    }
}
