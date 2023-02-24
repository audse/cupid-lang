use cupid_lex::token::Token;
use cupid_util::{FilterSome, Plus};
use std::{fmt, ops::Deref, rc::Rc};
use thiserror::Error;

use crate::{highlight::HighlightedLineSet, severity::Severity};

#[derive(Debug, Error, Clone, serde::Serialize, serde::Deserialize)]
pub struct Source(pub Rc<String>);
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// Gathers references to all tokens associated with the current node and its children
pub trait CollectTokens {
    fn collect_tokens(&self) -> Vec<&Token<'static>>;
}

#[derive(
    Debug,
    Default,
    Clone,
    derive_more::From,
    derive_more::TryInto,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ExprSource {
    Block(BlockSource),
    Assign(AssignSource),
    Decl(DeclSource),
    Function(FunctionSource),
    FunctionCall(FunctionCallSource),
    Ident(IdentSource),
    Impl(ImplSource),
    Namespace(NamespaceSource),
    Trait(TraitSource),
    TraitDef(TraitDefSource),
    Type(TypeSource),
    TypeDef(TypeDefSource),
    Value(Vec<Token<'static>>),
    #[default]
    Empty,
}

impl CollectTokens for Vec<Token<'static>> {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.iter().collect()
    }
}

impl CollectTokens for ExprSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        use ExprSource::*;
        match self {
            Block(x) => x.collect_tokens(),
            Assign(x) => x.collect_tokens(),
            Decl(x) => x.collect_tokens(),
            Function(x) => x.collect_tokens(),
            FunctionCall(x) => x.collect_tokens(),
            Ident(x) => x.collect_tokens(),
            Impl(x) => x.collect_tokens(),
            Namespace(x) => x.collect_tokens(),
            Trait(x) => x.collect_tokens(),
            TraitDef(x) => x.collect_tokens(),
            Type(x) => x.collect_tokens(),
            TypeDef(x) => x.collect_tokens(),
            Value(x) => x.collect_tokens(),
            Empty => vec![],
        }
    }
}

impl ExprSource {
    pub fn stringify(&self, severity: Severity, source: &str) -> String {
        HighlightedLineSet::new(&self.collect_tokens(), severity, source).finish()
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub BlockSourceBuilder => pub BlockSource {
        pub token_delimiters: (Token<'static>, Token<'static>), // e.g. ("{", "}") or ("=", ">")
        pub expressions: Vec<Rc<ExprSource>>,
    }
}

impl<T: CollectTokens> CollectTokens for Vec<T> {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.iter()
            .map(CollectTokens::collect_tokens)
            .flatten()
            .collect()
    }
}

impl<T: CollectTokens> CollectTokens for Option<T> {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.as_ref()
            .map(|s| s.collect_tokens())
            .unwrap_or_default()
    }
}

impl<T: CollectTokens> CollectTokens for Rc<T> {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.deref().collect_tokens()
    }
}

impl CollectTokens for BlockSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        let mut tokens = vec![&self.token_delimiters.0, &self.token_delimiters.1];
        tokens.extend(self.expressions.collect_tokens());
        tokens
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub AssignSourceBuilder => pub AssignSource {
        pub token_eq: Token<'static>,
        pub ident: Rc<ExprSource>,
        pub value: Option<Rc<ExprSource>>,
    }
}

impl CollectTokens for AssignSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_eq]
            .plus(self.ident.collect_tokens())
            .plus(self.value.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub DeclSourceBuilder => pub DeclSource {
        pub token_let: Option<Token<'static>>,
        pub token_mut: Option<Token<'static>>,
        pub token_colon: Option<Token<'static>>,
        pub token_eq: Option<Token<'static>>,
        pub ident: Rc<ExprSource>,
        pub type_annotation: Option<Rc<ExprSource>>,
        pub value: Option<Rc<ExprSource>>,
    }
}

impl CollectTokens for DeclSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![
            &self.token_let,
            &self.token_mut,
            &self.token_colon,
            &self.token_eq,
        ]
        .into_iter()
        .filter_map(|t| t.as_ref())
        .collect::<Vec<&Token<'static>>>()
        .plus(self.ident.collect_tokens())
        .plus(self.type_annotation.collect_tokens())
        .plus(self.value.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub FunctionSourceBuilder => pub FunctionSource {
        pub token_empty: Option<Token<'static>>,
        pub params: Vec<Rc<ExprSource>>,
        pub body: Rc<ExprSource>,
        pub return_type_annotation: Option<Rc<ExprSource>>,
    }
}

impl CollectTokens for FunctionSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.token_empty
            .as_ref()
            .map(|t| vec![t])
            .unwrap_or_default()
            .plus(self.params.collect_tokens())
            .plus(self.body.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub FunctionCallSourceBuilder => pub FunctionCallSource {
        pub token_parens: Option<(Token<'static>, Token<'static>)>,
        pub token_operator: Option<Token<'static>>,
        pub args: Vec<Rc<ExprSource>>,
        pub function: Rc<ExprSource>,
    }
}

impl CollectTokens for FunctionCallSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        let (open_paren, close_paren) = self
            .token_parens
            .as_ref()
            .map(|(a, b)| (Some(a), Some(b)))
            .unwrap_or_default();
        vec![open_paren, close_paren, self.token_operator.as_ref()]
            .into_iter()
            .filter_some()
            .collect::<Vec<&Token<'static>>>()
            .plus(self.args.collect_tokens())
            .plus(self.function.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub IdentSourceBuilder => pub IdentSource {
        pub token_name: Token<'static>,
        pub generics: Vec<Rc<ExprSource>>,
    }
}

impl CollectTokens for IdentSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_name].plus(self.generics.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub ImplSourceBuilder => pub ImplSource {
        pub token_impl: Token<'static>,
        pub token_for: Token<'static>,
        pub token_equal: Token<'static>,
        pub token_delimiters: (Token<'static>, Token<'static>),
        pub type_ident: Rc<ExprSource>,
        pub trait_ident: Rc<ExprSource>,
        pub methods: Vec<Rc<ExprSource>>,
    }
}

impl CollectTokens for ImplSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![
            &self.token_impl,
            &self.token_for,
            &self.token_equal,
            &self.token_delimiters.0,
            &self.token_delimiters.1,
        ]
        .plus(self.type_ident.collect_tokens())
        .plus(self.trait_ident.collect_tokens())
        .plus(self.methods.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub NamespaceSourceBuilder => pub NamespaceSource {
        pub token_delimiter: Option<Token<'static>>,
        pub namespace: Rc<ExprSource>,
        pub value: Rc<ExprSource>,
    }
}

impl CollectTokens for NamespaceSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        self.token_delimiter
            .as_ref()
            .map(|n| vec![n])
            .unwrap_or_default()
            .plus(self.namespace.collect_tokens())
            .plus(self.value.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TraitSourceBuilder => pub TraitSource {
        pub token_brackets: (Token<'static>, Token<'static>),
        pub ident: Rc<ExprSource>,
        pub methods: Vec<Rc<ExprSource>>,
    }
}

impl CollectTokens for TraitSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_brackets.0, &self.token_brackets.1]
            .plus(self.ident.collect_tokens())
            .plus(self.methods.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TraitDefSourceBuilder => pub TraitDefSource {
        pub token_trait: Token<'static>,
        pub token_eq: Token<'static>,
        pub ident: Rc<ExprSource>,
        pub value: Rc<ExprSource>,
    }
}

impl CollectTokens for TraitDefSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_trait, &self.token_eq]
            .plus(self.ident.collect_tokens())
            .plus(self.value.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TypeSourceBuilder => pub TypeSource {
        pub token_brackets: (Token<'static>, Token<'static>),
        pub ident: Rc<ExprSource>,
        pub fields: Vec<Rc<ExprSource>>,
    }
}

impl CollectTokens for TypeSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_brackets.0, &self.token_brackets.1]
            .plus(self.ident.collect_tokens())
            .plus(self.fields.collect_tokens())
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TypeDefSourceBuilder => pub TypeDefSource {
        pub token_type: Token<'static>,
        pub token_eq: Token<'static>,
        pub ident: Rc<ExprSource>,
        pub value: Rc<ExprSource>,
    }
}

impl CollectTokens for TypeDefSource {
    fn collect_tokens(&self) -> Vec<&Token<'static>> {
        vec![&self.token_type, &self.token_eq]
            .plus(self.ident.collect_tokens())
            .plus(self.value.collect_tokens())
    }
}
