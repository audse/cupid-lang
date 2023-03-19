use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Block<'src> {
        pub body: Vec<EntryId>,
    }
}

pub enum BlockSource<'src> {
    ArrowBlock(ArrowBlockSource<'src>),
    BraceBlock(BraceBlockSource<'src>),
}

pub struct ArrowBlockSource<'src> {
    pub arrow: Token<'src>,
    pub body_src: SourceId,
}

pub struct BraceBlockSource<'src> {
    pub open_brace: Token<'src>,
    pub close_brace: Token<'src>,
    pub body_src: Vec<SourceId>,
}

impl<'src> From<Block<'src>> for Expr<'src> {
    fn from(value: Block<'src>) -> Self {
        Expr::Block(value.into())
    }
}
