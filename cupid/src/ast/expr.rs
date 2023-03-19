use crate::{
    arena::{EntryId, ExprArena},
    cst::{expr::ExprSource, SourceId},
    error::CupidError,
    pointer::Pointer,
    scope::symbol::Symbol,
};

use std::{
    cell::{Ref, RefMut},
    fmt,
};

use super::{
    Array, BinOp, Block, Break, Call, Class, Constant, Define, Fun, Get, GetProperty, GetSuper,
    Header, If, Invoke, InvokeSuper, Loop, Return, Set, SetProperty, UnOp,
};

#[derive(Clone)]
pub enum Expr<'src> {
    Array(Array<'src>),
    BinOp(BinOp<'src>),
    Block(Block<'src>),
    Break(Break<'src>),
    Call(Call<'src>),
    Class(Class<'src>),
    Constant(Constant<'src>),
    Define(Define<'src>),
    Fun(Fun<'src>),
    Get(Get<'src>),
    GetProperty(GetProperty<'src>),
    GetSuper(GetSuper<'src>),
    If(If<'src>),
    Invoke(Invoke<'src>),
    InvokeSuper(InvokeSuper<'src>),
    Loop(Loop<'src>),
    Return(Return<'src>),
    Set(Set<'src>),
    SetProperty(SetProperty<'src>),
    UnOp(UnOp<'src>),
}

#[macro_export]
macro_rules! for_expr_variant {
    ($self:expr => |$inner:ident| $fun:expr) => {
        match $self {
            Self::Array($inner) => $fun,
            Self::BinOp($inner) => $fun,
            Self::Block($inner) => $fun,
            Self::Break($inner) => $fun,
            Self::Call($inner) => $fun,
            Self::Class($inner) => $fun,
            Self::Constant($inner) => $fun,
            Self::Define($inner) => $fun,
            Self::Fun($inner) => $fun,
            Self::Get($inner) => $fun,
            Self::GetProperty($inner) => $fun,
            Self::GetSuper($inner) => $fun,
            Self::If($inner) => $fun,
            Self::Invoke($inner) => $fun,
            Self::InvokeSuper($inner) => $fun,
            Self::Loop($inner) => $fun,
            Self::Return($inner) => $fun,
            Self::Set($inner) => $fun,
            Self::SetProperty($inner) => $fun,
            Self::UnOp($inner) => $fun,
        }
    };
}

pub(crate) use for_expr_variant;

pub trait HasSymbol<'src> {
    fn symbol_name(&self) -> &'src str;
    fn symbol(&self) -> Option<&Pointer<Symbol<'src>>>;
    fn symbol_mut(&mut self) -> Option<&mut Pointer<Symbol<'src>>>;
    fn set_symbol(&mut self, symbol: Option<Pointer<Symbol<'src>>>);
    fn expect_symbol(&self) -> Result<Ref<Symbol<'src>>, CupidError> {
        match self.symbol().as_ref() {
            Some(symbol) => Ok(symbol.borrow()),
            None => {
                let name = self.symbol_name();
                Err(CupidError::name_error(format!("Undefined: `{}`", name), name.to_string()))
            }
        }
    }
    fn expect_symbol_mut(&mut self) -> Result<RefMut<Symbol<'src>>, CupidError> {
        let name = self.symbol_name();
        if let Some(symbol) = self.symbol_mut() {
            return Ok(symbol.borrow_mut());
        }
        Err(CupidError::name_error(format!("Undefined: `{}`", name), name.to_string()))
    }
}

impl fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for_expr_variant!(self => |inner| write!(f, "Expr {:#?}", inner))
    }
}

pub trait GetSource<'src> {
    fn source_id(&self, arena: &ExprArena<'src>) -> SourceId;
    fn source<'a, 'b>(&'a self, arena: &'b ExprArena<'src>) -> &'b ExprSource<'src> {
        arena.expect_source(self.source_id(arena))
    }
}

impl<'src> GetSource<'src> for EntryId {
    fn source_id(&self, arena: &ExprArena<'src>) -> SourceId {
        arena.expect_expr(*self).header().source
    }
}

impl<'src, T: Header<'src>> GetSource<'src> for T {
    fn source_id(&self, _arena: &ExprArena<'src>) -> SourceId {
        self.header().source
    }
}
