use crate::{
    error::CupidError,
    token::{Token, TokenType},
};

use super::{
    Array, BinOp, Block, Break, Call, Class, Define, Expr, Fun, GetProperty, Header, If, Invoke,
    InvokeSuper, Loop, Method, Return, Set, SetProperty, UnOp,
};

pub trait Bx {
    fn bx(self) -> Box<Self>;
}

impl<T> Bx for T {
    fn bx(self) -> Box<Self> {
        Box::new(self)
    }
}

/// `Recompose` trait converts parsed instructions into other instructions.
/// The main use case for `recompose` is to convert binary operations.
pub trait Recompose {
    type Output;
    fn recompose(self) -> Result<Self::Output, CupidError>;
}

impl<'src, T: Recompose> Recompose for Vec<T> {
    type Output = Vec<T::Output>;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        self.into_iter().map(|item| item.recompose()).collect()
    }
}

impl<'src, T: Recompose> Recompose for Option<T> {
    type Output = Option<T::Output>;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        match self {
            Some(value) => Ok(Some(value.recompose()?)),
            None => Ok(None),
        }
    }
}

impl<T: Recompose> Recompose for Box<T> {
    type Output = Box<T::Output>;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Box::new((*self).recompose()?))
    }
}

impl<'src> Recompose for Expr<'src> {
    type Output = Expr<'src>;
    fn recompose(self) -> Result<Expr<'src>, CupidError> {
        match self {
            Expr::Array(array) => Ok(Expr::Array(array.recompose()?)),
            Expr::BinOp(binop) => Ok(binop.recompose()?),
            Expr::Block(block) => Ok(Expr::Block(block.recompose()?)),
            Expr::Break(inner) => Ok(Expr::Break(inner.recompose()?)),
            Expr::Call(call) => Ok(call.recompose()?),
            Expr::Class(class) => Ok(Expr::Class(class.recompose()?)),
            Expr::Define(def) => Ok(Expr::Define(def.recompose()?)),
            Expr::If(stmt) => Ok(Expr::If(stmt.recompose()?)),
            Expr::Fun(fun) => Ok(Expr::Fun(fun.recompose()?)),
            Expr::Loop(inner) => Ok(Expr::Loop(inner.recompose()?)),
            Expr::Return(ret) => Ok(Expr::Return(ret.recompose()?)),
            Expr::UnOp(unop) => Ok(Expr::UnOp(unop.recompose()?)),
            _ => Ok(self),
        }
    }
}

impl<'src> Recompose for Array<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Array {
            items: self.items.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for BinOp<'src> {
    type Output = Expr<'src>;
    fn recompose(self) -> Result<Expr<'src>, CupidError> {
        let left = self.left.recompose()?;
        let right = self.right.recompose()?;
        match self.op {
            TokenType::Equal => match &*left {
                Expr::GetProperty(get) => Ok(SetProperty {
                    header: get.header.clone(),
                    receiver: get.receiver.to_owned(),
                    property: get.property,
                    value: right,
                    receiver_scope: None,
                }
                .into()),
                _ => Ok(Set {
                    header: left.header().clone(),
                    name: extract_token(&left)?,
                    value: right,
                    symbol: None,
                }
                .into()),
            },
            TokenType::Dot => match &*right {
                Expr::Call(call) => Ok(Invoke {
                    header: call.header.clone(),
                    receiver: left,
                    callee: extract_token(&*call.callee)?,
                    args: call.args.to_owned().recompose()?,
                }
                .into()),
                _ => Ok(GetProperty {
                    header: left.header().clone(),
                    receiver: left,
                    property: extract_token(&*right)?,
                    receiver_scope: None,
                }
                .into()),
            },
            _ => Ok(BinOp {
                header: left.header().clone(),
                left,
                op: self.op,
                right,
            }
            .into()),
        }
    }
}

impl<'src> Recompose for Block<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Block {
            body: self.body.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Break<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Break {
            value: self.value.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Call<'src> {
    type Output = Expr<'src>;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        let callee = self.callee.recompose()?;
        let args = self.args.recompose()?;
        match &*callee {
            Expr::GetSuper(get) => Ok(InvokeSuper {
                header: get.header.clone(),
                name: get.name,
                symbol: None,
                args,
            }
            .into()),
            _ => Ok(Call {
                callee,
                args,
                ..self
            }
            .into()),
        }
    }
}

impl<'src> Recompose for Class<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Class {
            methods: self.methods.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Define<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Define {
            name: self.name,
            value: self.value.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for If<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(If {
            condition: self.condition.recompose()?,
            body: self.body.recompose()?,
            else_body: self.else_body.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Loop<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Loop {
            body: self.body.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Method<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Method {
            name: self.name,
            fun: self.fun.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Fun<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Fun {
            name: self.name,
            kind: self.kind,
            params: self.params.recompose()?,
            body: self.body.recompose()?,
            ..self
        })
    }
}

impl<'src> Recompose for Return<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(Return {
            value: match self.value {
                Some(value) => Some(value.recompose()?),
                None => None,
            },
            ..self
        })
    }
}

impl<'src> Recompose for UnOp<'src> {
    type Output = Self;
    fn recompose(self) -> Result<Self::Output, CupidError> {
        Ok(UnOp {
            expr: self.expr.recompose()?,
            ..self
        })
    }
}

fn extract_token<'src>(expr: &Expr<'src>) -> Result<Token<'src>, CupidError> {
    match expr {
        Expr::Get(var) => Ok(var.name),
        _ => Err(CupidError::parse_error(format!("Expected token: {:#?}", expr), None)),
    }
}
