use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ClassSource<'src> {
    pub open_brace: Token<'src>,
    pub close_brace: Token<'src>,
    pub class_kw: Token<'src>,
    pub name: Token<'src>,
    pub super_class: Option<Token<'src>>,
    pub super_class_name: Option<Token<'src>>,
    pub fields: Vec<SourceId>,
    pub methods: Vec<SourceId>,
}

impl<'src> HasToken<'src> for ClassSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.open_brace == token
            || self.close_brace == token
            || self.name == token
            || self.super_class == Some(token)
            || self.super_class_name == Some(token)
    }
}

impl<'src> From<ClassSource<'src>> for ExprSource<'src> {
    fn from(value: ClassSource<'src>) -> Self {
        ExprSource::Class(value.into())
    }
}
