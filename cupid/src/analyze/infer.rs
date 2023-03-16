use crate::{
    error::CupidError,
    for_expr_variant,
    parse::{
        Array, BinOp, Block, Break, Call, Class, Constant, Define, Expr, Fun, Get, GetProperty,
        GetSuper, Header, If, Invoke, InvokeSuper, Loop, Method, Return, Set, SetProperty, UnOp,
    },
    ty::Type,
    value::Value,
};

pub trait Infer
where
    Self: Sized,
{
    fn infer(self) -> Result<Self, CupidError>;
}

fn unwrapped_ty<'src, T: Header<'src>>(value: &Option<T>) -> Type<'src> {
    match value {
        Some(inner) => inner.ty(),
        None => Type::Nil,
    }
}

impl<T: Infer> Infer for Vec<T> {
    fn infer(self) -> Result<Self, CupidError> {
        self.into_iter().map(|item| item.infer()).collect()
    }
}

impl<T: Infer> Infer for Box<T> {
    fn infer(self) -> Result<Self, CupidError> {
        Ok(Box::new((*self).infer()?))
    }
}

impl<T: Infer> Infer for Option<T> {
    fn infer(self) -> Result<Self, CupidError> {
        match self {
            Some(inner) => Ok(Some(inner.infer()?)),
            None => Ok(None),
        }
    }
}

impl<'src> Infer for Expr<'src> {
    fn infer(self) -> Result<Self, CupidError> {
        for_expr_variant!(self => |inner| Ok(inner.infer()?.into()))
    }
}

impl<'src> Infer for Array<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.items = self.items.infer()?;
        self.header.ty = Type::Array;
        Ok(self)
    }
}

impl<'src> Infer for BinOp<'src> {
    fn infer(self) -> Result<Self, CupidError> {
        Ok(BinOp {
            left: self.left.infer()?,
            right: self.right.infer()?,
            ..self
        })
    }
}

impl<'src> Infer for Block<'src> {
    fn infer(self) -> Result<Self, CupidError> {
        Ok(Block {
            body: self.body.infer()?,
            ..self
        })
    }
}

impl<'src> Infer for Break<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.value = self.value.infer()?;
        self.set_ty(unwrapped_ty(&self.value));
        Ok(self)
    }
}

impl<'src> Infer for Call<'src> {
    fn infer(self) -> Result<Self, CupidError> {
        Ok(Call {
            callee: self.callee.infer()?,
            args: self.args.infer()?,
            ..self
        })
    }
}

impl<'src> Infer for Class<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        let name = self.name.lexeme;
        let ty = Type::class(name);
        self.scope_mut().annotate_ty(name, ty);
        self.scope_mut().annotate_ty("self", ty);
        if let Some(super_class) = self.super_class {
            self.scope_mut().annotate_ty("super", Type::class(super_class.lexeme));
        }
        self.header.ty = Type::class(name);
        self.methods = self.methods.infer()?;
        Ok(self)
    }
}

impl<'src> Infer for Constant<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
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

impl<'src> Infer for Define<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.value = self.value.infer()?;
        let ty = unwrapped_ty(&self.value);
        let name = self.name.lexeme;
        self.scope_mut().annotate_ty(name, ty);
        self.set_ty(Type::Unit);
        Ok(self)
    }
}

impl<'src> Infer for Fun<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.params = self.params.infer()?;
        self.body = self.body.infer()?;
        self.header.ty = Type::Function;
        if let Some(name) = self.name {
            let ty = self.ty();
            self.scope_mut().annotate_ty(name.lexeme, ty);
        }
        Ok(self)
    }
}

impl<'src> Infer for Get<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        let ty = self.expect_symbol()?.ty;
        self.set_ty(ty);
        Ok(self)
    }
}

impl<'src> Infer for GetProperty<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.receiver = self.receiver.infer()?;
        Ok(self)
    }
}

impl<'src> Infer for GetSuper<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        let ty = self.expect_symbol()?.ty;
        self.set_ty(ty);
        Ok(self)
    }
}

impl<'src> Infer for If<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.condition = self.condition.infer()?;
        self.body = self.body.infer()?;
        self.else_body = self.else_body.infer()?;
        Ok(self)
    }
}

impl<'src> Infer for Invoke<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.receiver = self.receiver.infer()?;
        Ok(self)
    }
}

impl<'src> Infer for InvokeSuper<'src> {
    fn infer(self) -> Result<Self, CupidError> {
        Ok(self)
    }
}

impl<'src> Infer for Loop<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.body = self.body.infer()?;
        self.set_ty(self.body.ty());
        Ok(self)
    }
}

impl<'src> Infer for Method<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.fun = self.fun.infer()?;
        self.set_ty(self.fun.header.ty);
        Ok(self)
    }
}

impl<'src> Infer for Return<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.value = self.value.infer()?;
        self.set_ty(unwrapped_ty(&self.value));
        Ok(self)
    }
}

impl<'src> Infer for Set<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.set_ty(Type::Unit);
        Ok(Set {
            value: self.value.infer()?,
            ..self
        })
    }
}

impl<'src> Infer for SetProperty<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.set_ty(Type::Unit);
        Ok(SetProperty {
            receiver: self.receiver.infer()?,
            value: self.value.infer()?,
            ..self
        })
    }
}

impl<'src> Infer for UnOp<'src> {
    fn infer(mut self) -> Result<Self, CupidError> {
        self.expr = self.expr.infer()?;
        self.set_ty(self.expr.ty());
        Ok(self)
    }
}
