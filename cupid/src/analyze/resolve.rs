use crate::{
    error::CupidError,
    for_expr_variant,
    parse::{
        Array, BinOp, Block, Break, Call, Class, Constant, Define, Expr, Fun, Get, GetProperty,
        GetSuper, Header, If, Invoke, InvokeSuper, Loop, Method, Return, Set, SetProperty, UnOp,
    },
};

pub trait Resolve
where
    Self: Sized,
{
    fn resolve(self) -> Result<Self, CupidError>;
}

impl<T: Resolve> Resolve for Vec<T> {
    fn resolve(self) -> Result<Self, CupidError> {
        self.into_iter().map(|item| item.resolve()).collect()
    }
}

impl<T: Resolve> Resolve for Box<T> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Box::new((*self).resolve()?))
    }
}

impl<T: Resolve> Resolve for Option<T> {
    fn resolve(self) -> Result<Self, CupidError> {
        match self {
            Some(inner) => Ok(Some(inner.resolve()?)),
            None => Ok(None),
        }
    }
}

impl<'src> Resolve for Expr<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        for_expr_variant!(self => |inner| Ok(inner.resolve()?.into()))
    }
}

macro_rules! update_symbol {
    ($self:ident => $token_field:ident, $symbol_field:ident) => {{
        let name = $self.$token_field.lexeme;
        let symbol = $self.scope().lookup(name);
        $self.$symbol_field = symbol;
    }};
}

impl<'src> Resolve for Array<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Array {
            items: self.items.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for BinOp<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(BinOp {
            left: self.left.resolve()?,
            right: self.right.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Block<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Block {
            body: self.body.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Break<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Break {
            value: self.value.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Call<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Call {
            callee: self.callee.resolve()?,
            args: self.args.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Class<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        let name = self.name.lexeme;
        self.scope_mut().define(name);

        let scope = self.header.scope.clone();
        self.scope_mut().insert_class(name, scope);

        self.scope_mut().define("self");
        self.scope_mut().annotate_class(name);

        if let Some(super_class) = self.super_class {
            self.scope_mut().define("super");
            self.scope_mut().annotate_class(super_class.lexeme);
        }
        Ok(Class {
            methods: self.methods.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Constant<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(self)
    }
}

impl<'src> Resolve for Define<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        let name = self.name.lexeme;
        self.scope_mut().define(name);
        Ok(Define {
            value: self.value.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Fun<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        if let Some(name) = self.name {
            self.scope_mut().define(name.lexeme);
        }
        Ok(Fun {
            params: self.params.resolve()?,
            body: self.body.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Get<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(self)
    }
}

impl<'src> Resolve for GetProperty<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        self.receiver = self.receiver.resolve()?;
        Ok(self)
    }
}

impl<'src> Resolve for GetSuper<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(self)
    }
}

impl<'src> Resolve for If<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        self.condition = self.condition.resolve()?;
        self.body = self.body.resolve()?;
        self.else_body = self.else_body.resolve()?;
        Ok(self)
    }
}

impl<'src> Resolve for Invoke<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        self.receiver = self.receiver.resolve()?;
        Ok(self)
    }
}

impl<'src> Resolve for InvokeSuper<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(self)
    }
}

impl<'src> Resolve for Loop<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Loop {
            body: self.body.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Method<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        let name = self.name.lexeme;
        self.scope_mut().define(name);
        Ok(Method {
            fun: self.fun.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Return<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(Return {
            value: self.value.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for Set<'src> {
    fn resolve(mut self) -> Result<Self, CupidError> {
        update_symbol!(self => name, symbol);
        Ok(Set {
            value: self.value.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for SetProperty<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(SetProperty {
            receiver: self.receiver.resolve()?,
            value: self.value.resolve()?,
            ..self
        })
    }
}

impl<'src> Resolve for UnOp<'src> {
    fn resolve(self) -> Result<Self, CupidError> {
        Ok(UnOp {
            expr: self.expr.resolve()?,
            ..self
        })
    }
}
