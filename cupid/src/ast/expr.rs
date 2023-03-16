use crate::{
    compiler::FunctionType,
    error::CupidError,
    pointer::Pointer,
    scope::{symbol::Symbol, Scope},
    token::{Token, TokenType},
    ty::Type,
    value::Value,
};

use std::{
    cell::{Ref, RefMut},
    fmt,
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

#[derive(Debug, Clone)]
pub struct InstHeader<'src> {
    pub ty: Type<'src>,
    pub scope: Pointer<Scope<'src>>,
}

impl<'src> InstHeader<'src> {
    pub fn new() -> Self {
        Self {
            ty: Type::Unknown,
            scope: Pointer::<Scope>::global(),
        }
    }
}

pub trait Header<'src> {
    fn header(&self) -> &InstHeader<'src>;
    fn header_mut(&mut self) -> &mut InstHeader<'src>;
    fn ty(&self) -> Type<'src> {
        self.header().ty
    }
    fn set_ty(&mut self, ty: Type<'src>) {
        self.header_mut().ty = ty;
    }
    fn scope(&self) -> Ref<Scope<'src>> {
        self.header().scope.borrow()
    }
    fn scope_mut(&mut self) -> RefMut<Scope<'src>> {
        self.header_mut().scope.borrow_mut()
    }
}

impl<'src, T: Header<'src>> Header<'src> for Box<T> {
    fn header(&self) -> &InstHeader<'src> {
        (**self).header()
    }
    fn header_mut(&mut self) -> &mut InstHeader<'src> {
        (**self).header_mut()
    }
}

impl fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for_expr_variant!(self => |inner| write!(f, "Expr {:#?}", inner))
    }
}

impl<'src> Header<'src> for Expr<'src> {
    fn header(&self) -> &InstHeader<'src> {
        for_expr_variant!(self => |inner| inner.header())
    }
    fn header_mut(&mut self) -> &mut InstHeader<'src> {
        for_expr_variant!(self => |inner| inner.header_mut())
    }
}

impl<'src> Expr<'src> {
    pub fn new_binop(left: Expr<'src>, right: Expr<'src>, op: Token<'src>) -> Expr<'src> {
        Expr::BinOp(
            BinOp {
                header: InstHeader::new(),
                left: Box::new(left),
                right: Box::new(right),
                op: op.kind,
            }
            .into(),
        )
    }
}

macro_rules! with_header {
    (
        #[derive( $($der:ty),* )]
        $v:vis struct $name:ident<'src> {
            $( $field_v:vis $field_name:ident: $field_ty:ty ),* $(,)?
        }
    ) => {
        #[derive( $($der),* )]
        $v struct $name<'src> {
            pub header: InstHeader<'src>,
            $( $field_v $field_name: $field_ty ),*
        }
        impl<'src> Header<'src> for $name<'src> {
            fn header(&self) -> &InstHeader<'src> {
                &self.header
            }
            fn header_mut(&mut self) -> &mut InstHeader<'src> {
                &mut self.header
            }
        }
    };
}

macro_rules! expect_symbol {
    ($token_field:ident) => {
        pub fn expect_symbol(&self) -> Result<Ref<Symbol<'src>>, CupidError> {
            match self.symbol.as_ref() {
                Some(symbol) => Ok(symbol.borrow()),
                None => Err(CupidError::name_error(
                    format!("Undefined: `{}`", self.$token_field.lexeme),
                    self.$token_field.to_static(),
                )),
            }
        }
    };
}

with_header! {
    #[derive(Clone)]
    pub struct Array<'src> {
        pub items: Vec<Expr<'src>>,
    }
}

impl<'src> Array<'src> {
    pub fn new(items: Vec<Expr<'src>>) -> Self {
        Array {
            header: InstHeader::new(),
            items,
        }
    }
}

impl fmt::Debug for Array<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array ")?;
        f.debug_list().entries(&self.items).finish()
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct BinOp<'src> {
        pub left: Box<Expr<'src>>,
        pub right: Box<Expr<'src>>,
        pub op: TokenType,
    }
}

with_header! {
    #[derive(Clone)]
    pub struct Block<'src> {
        pub body: Vec<Expr<'src>>,
    }
}

impl fmt::Debug for Block<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block ")?;
        f.debug_list().entries(&self.body).finish()
    }
}

impl<'src> Block<'src> {
    pub fn new(body: Vec<Expr<'src>>) -> Self {
        Self {
            header: InstHeader::new(),
            body,
        }
    }
}

with_header! {
    #[derive(Clone)]
    pub struct Break<'src> {
        pub value: Option<Box<Expr<'src>>>,
    }
}

impl<'src> Break<'src> {
    pub fn new(expr: impl Into<Expr<'src>>) -> Self {
        Self {
            header: InstHeader::new(),
            value: Some(Box::new(expr.into())),
        }
    }
}

impl fmt::Debug for Break<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "Break({:#?})", value),
            None => write!(f, "Break"),
        }
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Call<'src> {
        pub callee: Box<Expr<'src>>,
        pub args: Vec<Expr<'src>>,
    }
}

impl<'src> Call<'src> {
    pub fn new(callee: impl Into<Expr<'src>>, args: Vec<Expr<'src>>) -> Self {
        Self {
            header: InstHeader::new(),
            callee: Box::new(callee.into()),
            args,
        }
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Class<'src> {
        pub name: Token<'src>,
        pub super_class: Option<Token<'src>>,
        pub methods: Vec<Method<'src>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Constant<'src> {
        pub value: Value,
    }
}

impl<'src> Constant<'src> {
    pub fn new(val: impl Into<Value>) -> Self {
        Self {
            header: InstHeader::new(),
            value: val.into(),
        }
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Define<'src> {
        pub name: Token<'src>,
        pub value: Option<Box<Expr<'src>>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Fun<'src> {
        pub kind: FunctionType,
        pub name: Option<Token<'src>>,
        pub params: Vec<Define<'src>>,
        pub body: Block<'src>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Get<'src> {
        pub name: Token<'src>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

impl<'src> Get<'src> {
    expect_symbol! { name }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct GetProperty<'src> {
        pub receiver: Box<Expr<'src>>,
        pub property: Token<'src>,
        pub receiver_scope: Option<Pointer<Scope<'src>>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct GetSuper<'src> {
        pub name: Token<'src>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

impl<'src> GetSuper<'src> {
    expect_symbol! { name }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct If<'src> {
        pub condition: Box<Expr<'src>>,
        pub body: Box<Expr<'src>>,
        pub else_body: Option<Box<Expr<'src>>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Invoke<'src> {
        pub receiver: Box<Expr<'src>>,
        pub callee: Token<'src>,
        pub args: Vec<Expr<'src>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct InvokeSuper<'src> {
        pub name: Token<'src>,
        pub args: Vec<Expr<'src>>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

impl<'src> InvokeSuper<'src> {
    expect_symbol! { name }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Loop<'src> {
        pub body: Block<'src>,
    }
}

impl<'src> Loop<'src> {
    pub fn new(body: Block<'src>) -> Self {
        Self {
            header: InstHeader::new(),
            body,
        }
    }
}

with_header! {
    #[derive(Clone)]
    pub struct Method<'src> {
        pub name: Token<'src>,
        pub fun: Fun<'src>,
    }
}

impl fmt::Debug for Method<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name)
            .field("params", &self.fun.params)
            .field("body", &self.fun.body)
            .finish()
    }
}

with_header! {
    #[derive(Clone)]
    pub struct Return<'src> {
        pub value: Option<Box<Expr<'src>>>,
    }
}

impl<'src> Return<'src> {
    pub fn new(value: impl Into<Expr<'src>>) -> Self {
        Self {
            header: InstHeader::new(),
            value: Some(Box::new(value.into())),
        }
    }
}

impl fmt::Debug for Return<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "Return({:#?})", value),
            None => write!(f, "Return"),
        }
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct Set<'src> {
        pub name: Token<'src>,
        pub value: Box<Expr<'src>>,
        pub symbol: Option<Pointer<Symbol<'src>>>,
    }
}

impl<'src> Set<'src> {
    expect_symbol! { name }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct SetProperty<'src> {
        pub receiver: Box<Expr<'src>>,
        pub property: Token<'src>,
        pub value: Box<Expr<'src>>,
        pub receiver_scope: Option<Pointer<Scope<'src>>>,
    }
}

with_header! {
    #[derive(Debug, Clone)]
    pub struct UnOp<'src> {
        pub expr: Box<Expr<'src>>,
        pub op: TokenType,
    }
}

impl<'src> From<Array<'src>> for Expr<'src> {
    fn from(value: Array<'src>) -> Self {
        Expr::Array(value.into())
    }
}

impl<'src> From<BinOp<'src>> for Expr<'src> {
    fn from(value: BinOp<'src>) -> Self {
        Expr::BinOp(value.into())
    }
}

impl<'src> From<Block<'src>> for Expr<'src> {
    fn from(value: Block<'src>) -> Self {
        Expr::Block(value.into())
    }
}

impl<'src> From<Break<'src>> for Expr<'src> {
    fn from(value: Break<'src>) -> Self {
        Expr::Break(value.into())
    }
}

impl<'src> From<Call<'src>> for Expr<'src> {
    fn from(value: Call<'src>) -> Self {
        Expr::Call(value.into())
    }
}

impl<'src> From<Class<'src>> for Expr<'src> {
    fn from(value: Class<'src>) -> Self {
        Expr::Class(value.into())
    }
}

impl<'src> From<Define<'src>> for Expr<'src> {
    fn from(value: Define<'src>) -> Self {
        Expr::Define(value.into())
    }
}

impl<'src> From<Fun<'src>> for Expr<'src> {
    fn from(value: Fun<'src>) -> Self {
        Expr::Fun(value.into())
    }
}

impl<'src> From<Get<'src>> for Expr<'src> {
    fn from(value: Get<'src>) -> Self {
        Expr::Get(value.into())
    }
}

impl<'src> From<GetProperty<'src>> for Expr<'src> {
    fn from(value: GetProperty<'src>) -> Self {
        Expr::GetProperty(value.into())
    }
}

impl<'src> From<GetSuper<'src>> for Expr<'src> {
    fn from(value: GetSuper<'src>) -> Self {
        Expr::GetSuper(value.into())
    }
}

impl<'src> From<If<'src>> for Expr<'src> {
    fn from(value: If<'src>) -> Self {
        Expr::If(value.into())
    }
}

impl<'src> From<Invoke<'src>> for Expr<'src> {
    fn from(value: Invoke<'src>) -> Self {
        Expr::Invoke(value.into())
    }
}

impl<'src> From<InvokeSuper<'src>> for Expr<'src> {
    fn from(value: InvokeSuper<'src>) -> Self {
        Expr::InvokeSuper(value.into())
    }
}

impl<'src> From<Loop<'src>> for Expr<'src> {
    fn from(value: Loop<'src>) -> Self {
        Expr::Loop(value.into())
    }
}

impl<'src> From<Return<'src>> for Expr<'src> {
    fn from(value: Return<'src>) -> Self {
        Expr::Return(value.into())
    }
}

impl<'src> From<Set<'src>> for Expr<'src> {
    fn from(value: Set<'src>) -> Self {
        Expr::Set(value.into())
    }
}

impl<'src> From<SetProperty<'src>> for Expr<'src> {
    fn from(value: SetProperty<'src>) -> Self {
        Expr::SetProperty(value.into())
    }
}

impl<'src> From<UnOp<'src>> for Expr<'src> {
    fn from(value: UnOp<'src>) -> Self {
        Expr::UnOp(value.into())
    }
}

impl<'src> From<Value> for Expr<'src> {
    fn from(value: Value) -> Self {
        Expr::Constant(Constant {
            header: InstHeader::new(),
            value,
        })
    }
}
impl<'src> From<Constant<'src>> for Expr<'src> {
    fn from(value: Constant<'src>) -> Self {
        Expr::Constant(value)
    }
}
