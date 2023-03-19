use crate::{
    arena::{EntryId, ExprArena, UseArena},
    error::CupidError,
    token::{Token, TokenType},
};

use super::{
    Array, BinOp, Block, Break, Call, Class, Define, Expr, Fun, GetProperty, Header, If, Invoke,
    InvokeSuper, Loop, Method, Return, Set, SetProperty, UnOp,
};

/// `Recompose` trait converts parsed instructions into other instructions.
/// The main use case for `recompose` is to convert binary operations into more descriptive AST nodes.
pub trait Recompose<'src> {
    type Output;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError>;
}

impl<'src, T: Recompose<'src>> Recompose<'src> for Vec<T> {
    type Output = Vec<T::Output>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        self.into_iter().map(|item| item.recompose(arena)).collect()
    }
}

impl<'src, T: Recompose<'src>> Recompose<'src> for Option<T> {
    type Output = Option<T::Output>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        match self {
            Some(value) => Ok(Some(value.recompose(arena)?)),
            None => Ok(None),
        }
    }
}

impl<'src, T: Recompose<'src>> Recompose<'src> for Box<T> {
    type Output = Box<T::Output>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Box::new((*self).recompose(arena)?))
    }
}

impl<'src> Recompose<'src> for EntryId {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        let expr: Expr<'src> = arena.take(self);
        let expr = expr.recompose(arena)?;
        arena.replace(self, expr);
        Ok(self)
    }
}

impl<'src> Recompose<'src> for Expr<'src> {
    type Output = Expr<'src>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Expr<'src>, CupidError> {
        match self {
            Expr::Array(array) => Ok(Expr::Array(array.recompose(arena)?)),
            Expr::BinOp(binop) => Ok(binop.recompose(arena)?),
            Expr::Block(block) => Ok(Expr::Block(block.recompose(arena)?)),
            Expr::Break(inner) => Ok(Expr::Break(inner.recompose(arena)?)),
            Expr::Call(call) => Ok(call.recompose(arena)?),
            Expr::Class(class) => Ok(Expr::Class(class.recompose(arena)?)),
            Expr::Define(def) => Ok(Expr::Define(def.recompose(arena)?)),
            Expr::If(stmt) => Ok(Expr::If(stmt.recompose(arena)?)),
            Expr::Fun(fun) => Ok(Expr::Fun(fun.recompose(arena)?)),
            Expr::Loop(inner) => Ok(Expr::Loop(inner.recompose(arena)?)),
            Expr::Return(ret) => Ok(Expr::Return(ret.recompose(arena)?)),
            Expr::UnOp(unop) => Ok(Expr::UnOp(unop.recompose(arena)?)),
            _ => Ok(self),
        }
    }
}

impl<'src> Recompose<'src> for Array<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Array {
            items: self.items.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for BinOp<'src> {
    type Output = Expr<'src>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Expr<'src>, CupidError> {
        let left = self.left.recompose(arena)?;
        let right = self.right.recompose(arena)?;
        let left_ref = arena.expect(left);
        let right_ref = arena.expect(right);
        match self.op {
            TokenType::Equal => match left_ref {
                Expr::GetProperty(get) => Ok(SetProperty {
                    header: get.header.clone(),
                    receiver: get.receiver.to_owned(),
                    property: get.property,
                    value: right,
                    symbol: None,
                }
                .into()),
                _ => Ok(Set {
                    header: left_ref.header().clone(),
                    name: extract_token(&left_ref)?,
                    value: right,
                    symbol: None,
                }
                .into()),
            },
            TokenType::Dot => match right_ref {
                Expr::Call(call) => Ok(Invoke {
                    header: call.header.clone(),
                    receiver: left,
                    callee: extract_entry_token(call.callee, arena)?,
                    args: call.args.to_owned().recompose(arena)?,
                    symbol: None,
                }
                .into()),
                _ => Ok(GetProperty {
                    header: left_ref.header().clone(),
                    receiver: left,
                    property: extract_token(right_ref)?,
                    symbol: None,
                }
                .into()),
            },
            _ => Ok(BinOp {
                header: left_ref.header().clone(),
                left,
                op: self.op,
                right,
            }
            .into()),
        }
    }
}

impl<'src> Recompose<'src> for Block<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Block {
            body: self.body.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Break<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Break {
            value: self.value.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Call<'src> {
    type Output = Expr<'src>;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        let callee = self.callee.recompose(arena)?;
        let args = self.args.recompose(arena)?;
        let callee_ref = arena.expect(callee);
        match callee_ref {
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

impl<'src> Recompose<'src> for Class<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Class {
            methods: self.methods.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Define<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Define {
            name: self.name,
            value: self.value.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for If<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(If {
            condition: self.condition.recompose(arena)?,
            body: self.body.recompose(arena)?,
            else_body: self.else_body.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Loop<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Loop {
            body: self.body.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Method<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Method {
            name: self.name,
            fun: self.fun.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Fun<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Fun {
            name: self.name,
            kind: self.kind,
            params: self.params.recompose(arena)?,
            body: self.body.recompose(arena)?,
            ..self
        })
    }
}

impl<'src> Recompose<'src> for Return<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(Return {
            value: match self.value {
                Some(value) => Some(value.recompose(arena)?),
                None => None,
            },
            ..self
        })
    }
}

impl<'src> Recompose<'src> for UnOp<'src> {
    type Output = Self;
    fn recompose(self, arena: &mut ExprArena<'src>) -> Result<Self::Output, CupidError> {
        Ok(UnOp {
            expr: self.expr.recompose(arena)?,
            ..self
        })
    }
}

fn extract_entry_token<'src>(
    id: EntryId,
    arena: &ExprArena<'src>,
) -> Result<Token<'src>, CupidError> {
    extract_token(arena.expect(id))
}

fn extract_token<'src>(expr: &Expr<'src>) -> Result<Token<'src>, CupidError> {
    match expr {
        Expr::Get(var) => Ok(var.name),
        _ => Err(CupidError::parse_error(format!("Expected token: {:#?}", expr), None)),
    }
}
