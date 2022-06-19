use crate::{AsNode, pre_analysis, package_resolution, type_scope_analysis, type_name_resolution, scope_analysis, name_resolution, type_inference, type_checking, flow_checking, linting};

macro_rules! impl_try_from_expr_for_node {
    ( $( $pass:ident ),* ) => {
        $(
            crate::util::from_into::expr::impl_try_from_expr_for_node! {
                impl TryFrom<$pass::Expr> for crate::Block <$pass::Expr>;
                impl TryFrom<$pass::Expr> for $pass::Decl;
                impl TryFrom<$pass::Expr> for $pass::Function;
                impl TryFrom<$pass::Expr> for crate::Ident;
                impl TryFrom<$pass::Expr> for $pass::TypeDef;
                impl TryFrom<$pass::Expr> for crate::Value;
            }
        )*
    };
    ( $( impl TryFrom<$pass:ident :: Expr> for $node_pass:ident :: $node:ident $( <$($generics:ty),*> )?; )* ) => {
        $(
            impl TryFrom<crate::$pass::Expr> for $node_pass::$node  $( <$($generics),*> )? {
                type Error = crate::PassErr;
                fn try_from(value: crate::$pass::Expr) -> crate::PassResult<$node_pass::$node $( <$($generics),*> )?> {
                    match value {
                        crate::$pass::Expr::$node(x) => Ok(x),
                        _ => Err(value.err(cupid_util::ERR_EXPECTED_EXPRESSION))
                    }
                }
            }
        )*
    };
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

pub(crate) use {impl_try_from_expr_for_node, impl_from_node_for_expr};

impl_try_from_expr_for_node! {
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