use crate::{pre_analysis, package_resolution, type_scope_analysis, type_name_resolution, scope_analysis, name_resolution, type_inference, type_checking, flow_checking, linting};

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