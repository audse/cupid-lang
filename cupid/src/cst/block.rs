use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum BlockSource<'src> {
    ArrowBlock(ArrowBlockSource<'src>),
    BraceBlock(BraceBlockSource<'src>),
}

impl<'src> HasToken<'src> for BlockSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        match self {
            Self::ArrowBlock(inner) => inner.has_token(token),
            Self::BraceBlock(inner) => inner.has_token(token),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrowBlockSource<'src> {
    pub arrow: Token<'src>,
    pub body_src: SourceId,
}

impl<'src> HasToken<'src> for ArrowBlockSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.arrow == token
    }
}

#[derive(Debug, Clone)]
pub struct BraceBlockSource<'src> {
    pub open_brace: Token<'src>,
    pub close_brace: Token<'src>,
    pub body_src: Vec<SourceId>,
}

impl<'src> HasToken<'src> for BraceBlockSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.open_brace == token || self.close_brace == token
    }
}

impl<'src> From<BlockSource<'src>> for ExprSource<'src> {
    fn from(value: BlockSource<'src>) -> Self {
        ExprSource::Block(value.into())
    }
}

impl<'src> From<ArrowBlockSource<'src>> for ExprSource<'src> {
    fn from(value: ArrowBlockSource<'src>) -> Self {
        BlockSource::ArrowBlock(value).into()
    }
}

impl<'src> From<BraceBlockSource<'src>> for ExprSource<'src> {
    fn from(value: BraceBlockSource<'src>) -> Self {
        BlockSource::BraceBlock(value).into()
    }
}
