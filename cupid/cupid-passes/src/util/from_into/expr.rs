use cupid_util::error_codes::*;
use crate::{PassErr, PassResult, for_each_expr,  AsNode, PassExpr, PassExpr::*, Value, pre_analysis, package_resolution, type_scope_analysis, type_name_resolution, scope_analysis, name_resolution, type_inference, type_checking, flow_checking, linting};


/// For the `Expr` struct in each semantic pass, implements
/// `TryFrom<PassExpr>` to downcase a `PassExpr`
/// and `From<Expr> for PassExpr` to upcase an `Expr` struct
macro_rules! impl_try_from_pass_expr_for_expr {
    ( $( impl TryFrom<$pass:ident :: $node:ident> for crate :: PassExpr :: $variant:ident; )* ) => {
        $( 
            impl TryFrom<PassExpr> for $pass::$node {
                type Error = PassErr;
                fn try_from(value: PassExpr) -> PassResult<Self> {
                    match value {
                        $variant(x) => Ok(x),
                        _ => Err(value.err(ERR_EXPECTED_EXPRESSION))
                    }
                }
            }
            impl<'expr> TryFrom<&'expr PassExpr> for &'expr $pass::$node {
                type Error = PassErr;
                fn try_from(value: &'expr PassExpr) -> PassResult<Self> {
                    match value {
                        $variant(x) => Ok(x),
                        _ => Err(value.err(ERR_EXPECTED_EXPRESSION))
                    }
                }
            }
            impl From<$pass::Expr> for PassExpr {
                fn from(expr: $pass::Expr) -> Self {
                    Self::$variant(expr)
                }
            }
        )*
    };
    () => {};
}

impl TryFrom<PassExpr> for Value {
    type Error = PassErr;
    fn try_from(value: PassExpr) -> PassResult<Self> {
        for_each_expr!(value => |x| Value::try_from(x))
    }
}

macro_rules! impl_from_node_for_expr {
    ( $( $pass:ident ),* ) => {
        $(
            crate::util::from_into::expr::impl_from_node_for_expr! {
                impl From<crate::Block <$pass::Expr> > for $pass::Expr::Block;
                impl From<$pass::Decl> for $pass::Expr::Decl;
                impl From<$pass::Function> for $pass::Expr::Function;
                impl From<crate::Ident> for $pass::Expr::Ident;
                impl From<$pass::TypeDef> for $pass::Expr::TypeDef;
                impl From<crate::Value> for $pass::Expr::Value;
            }
        )*
    };
    ( $( 
        impl From
            <$pass:ident :: $node:ident $( <$($generics:ty),*> )? > 
            for $expr_pass:ident :: Expr :: $variant:ident ; 
        )* ) => {
        $(
            impl From<$pass::$node $( <$($generics),*> )?> for $expr_pass::Expr {
                fn from(from: $pass::$node $( <$($generics),*> )?) -> Self {
                    Self::$variant(from)
                }
            }
        )*
    };
    () => {};
}

pub(crate) use impl_from_node_for_expr;


impl_try_from_pass_expr_for_expr! {
    impl TryFrom<pre_analysis::Expr> for crate::PassExpr::PreAnalysis;
    impl TryFrom<package_resolution::Expr> for crate::PassExpr::PackageResolved;
    impl TryFrom<type_scope_analysis::Expr> for crate::PassExpr::TypeScopeAnalyzed;
    impl TryFrom<type_name_resolution::Expr> for crate::PassExpr::TypeNameResolved;
    impl TryFrom<scope_analysis::Expr> for crate::PassExpr::ScopeAnalyzed;
    impl TryFrom<name_resolution::Expr> for crate::PassExpr::NameResolved;
    impl TryFrom<type_inference::Expr> for crate::PassExpr::TypeInferred;
    impl TryFrom<type_checking::Expr> for crate::PassExpr::TypeChecked;
    impl TryFrom<flow_checking::Expr> for crate::PassExpr::FlowChecked;
    impl TryFrom<linting::Expr> for crate::PassExpr::Linted;
}

impl_from_node_for_expr! { 
    pre_analysis, 
    package_resolution,
    type_scope_analysis,
    type_name_resolution,
    scope_analysis,
    name_resolution,
    type_inference,
    type_checking,
    flow_checking,
    linting
}